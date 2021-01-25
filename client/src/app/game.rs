use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

use sha2::Digest;
use web_sys::WebGlRenderingContext;
use yew::prelude::*;
use yew::services::render::{RenderService, RenderTask};
use yew::services::resize::{ResizeService, ResizeTask, WindowDimensions};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use super::chat;
use crate::render::Render;
use common::proto::{handshake, BinRead, BinWrite, Packet};

pub struct Game {
    link: ComponentLink<Self>,
    props: Properties,
    _resize_task: ResizeTask,
    render_task: Option<RenderTask>,
    render: Option<Render>,
    ws: WebSocketTask,
    step: Step,
    dim: WindowDimensions,
    chat: chat::List,
    graphics_canvas_ref: NodeRef,
    overlay_canvas_ref: NodeRef,
}

impl Game {
    fn graphics_canvas(&self) -> WebGlRenderingContext {
        use wasm_bindgen::JsCast;

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let elem = self.graphics_canvas_ref.cast::<web_sys::HtmlCanvasElement>().unwrap();
        elem.get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap()
    }

    fn run_render(&mut self) {
        self.render
            .as_mut()
            .expect("run_render called without initializing render context")
            .ren();
    }

    fn schedule_render(&mut self) {
        let task = RenderService::request_animation_frame(self.link.callback(Message::Render));
        self.render_task = Some(task);
    }
}

impl Component for Game {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Properties, link: ComponentLink<Self>) -> Self {
        let resize_task = ResizeService::new().register(link.callback(Message::WindowResize));

        let ws = WebSocketService::connect_binary(
            &props.addr_string(true),
            link.callback(Message::WsReceive),
            link.callback(Message::WsStatus),
        )
        .unwrap();

        Self {
            link,
            props,
            _resize_task: resize_task,
            render_task: None,
            render: None,
            ws,
            dim: WindowDimensions::get_dimensions(&web_sys::window().unwrap()),
            step: Step::SecureConnect,
            chat: chat::List::new(100),
            graphics_canvas_ref: NodeRef::default(),
            overlay_canvas_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Message) -> ShouldRender {
        match msg {
            Message::WsStatus(status) => match status {
                WebSocketStatus::Error => match self.step {
                    Step::SecureConnect if self.props.allow_insecure => {
                        // reconnect
                        self.step = Step::InsecureConnect;
                        let addr = format!("ws://{}:{}", self.props.addr, self.props.port);
                        self.ws = WebSocketService::connect_binary(
                            &addr,
                            self.link.callback(Message::WsReceive),
                            self.link.callback(Message::WsStatus),
                        )
                        .unwrap();
                        self.chat.push_system(
                            "Secure connection failed, trying insecure connection".to_owned(),
                        );
                        true
                    }
                    Step::SecureConnect | Step::InsecureConnect => {
                        // connection failed
                        self.props
                            .error_hook
                            .emit(Some("Connection failed".to_owned()));
                        false
                    }
                    _ => {
                        // connection broken
                        self.props
                            .error_hook
                            .emit(Some("A network error occurred".to_owned()));
                        false
                    }
                },
                WebSocketStatus::Opened => {
                    // TODO
                    self.chat.push_system("Connection established".to_owned());
                    self.step = Step::VersionHandshake;

                    let checksum = common::proto::VERSION;
                    self.ws.send_binary(Ok(checksum.to_vec()));
                    true
                }
                WebSocketStatus::Closed => {
                    self.props
                        .error_hook
                        .emit(Some("Connection closed".to_owned()));
                    true
                }
            },
            Message::WsReceive(recv) => {
                let ret: anyhow::Result<bool> = (|| {
                    let recv = recv?;
                    Ok(match self.step {
                        Step::VersionHandshake => {
                            if recv[..] != common::proto::VERSION {
                                let error = format!(
                            "Incompatible protocol versions (client = {:032x?}, server = {:032x?})",
                            &common::proto::VERSION[..],
                            &recv[..],
                        );
                                self.props.error_hook.emit(Some(error));
                                return Ok(true);
                            }

                            self.ws
                                .send_binary(Ok(Packet::HandshakeLogin(handshake::Login {
                                    identity: self.props.hashed_identity(),
                                    name: self.props.name.clone(),
                                })
                                .write_to_vec()));

                            self.chat.push_system("Logging in".to_owned());

                            self.step = Step::AcceptReject;
                            true
                        }
                        Step::AcceptReject => {
                            let packet = Packet::read(&mut &recv[..]);

                            match packet {
                                Ok(Packet::HandshakeAccept(_)) => {
                                    log::debug!("Handshake accepted");
                                }
                                Ok(Packet::HandshakeReject(packet)) => {
                                    anyhow::bail!("Connection rejected: {}", &packet.reason)
                                }
                                _ => anyhow::bail!("Server responded with invalid data"),
                            }

                            self.step = Step::Download;
                            true
                        }
                        step => {
                            log::warn!("Received message at unexpected step {:?}", step);
                            false
                        }
                    })
                })();
                match ret {
                    Ok(r) => r,
                    Err(err) => {
                        self.props.error_hook.emit(Some(err.to_string()));
                        true
                    }
                }
            }
            Message::WindowResize(dim) => {
                self.dim = dim;
                true
            }
            Message::Render(_time) => {
                self.run_render();
                self.schedule_render();
                false
            }
            Message::SendChat(message) => {
                log::debug!("SendChat({})", message);
                // TODO
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        unreachable!()
    }

    fn view(&self) -> Html {
        let parent_style = css! {
            "width": "100vw";
            "height": "100vh";
            "display": "block";
        };
        let chat_style = css! {
            "position": "absolute";
            "x": 0;
            "y": 0;
            "z-index": 2;
        };
        let overlay_style = css! {
            "width": "100vw";
            "height": "100vh";
            "display": "block";
            "z-index": 1;
        };
        let graphics_style = css! {
            "width": "100vw";
            "height": "100vh";
            "display": "block";
            "z-index": 0;
        };

        html! {
            <div style=parent_style>
                <chat::ChatComp style=chat_style
                    messages=&self.chat
                    has_input=true
                    send_chat_callback=self.link.callback(Message::SendChat)
                    />
                <canvas ref=self.graphics_canvas_ref.clone()
                    width=self.dim.width height=self.dim.height
                    style=graphics_style
                    />
                <canvas ref=self.overlay_canvas_ref.clone()
                    width=self.dim.width height=self.dim.height
                    style=overlay_style
                    />
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let gl = self.graphics_canvas();
        self.render = Some(Render::new(gl));

        if first_render {
            self.schedule_render();
        }
    }
}

pub enum Message {
    WsStatus(WebSocketStatus),
    WsReceive(anyhow::Result<Vec<u8>>),
    WindowResize(WindowDimensions),
    Render(f64),
    SendChat(String),
}

#[derive(Clone, Debug, Properties)]
pub struct Properties {
    pub addr: String,
    pub port: u16,
    pub allow_insecure: bool,
    pub identity: Vec<u8>,
    pub name: String,
    pub error_hook: Callback<Option<String>>,
}

impl Properties {
    fn addr_string(&self, secure: bool) -> String {
        debug_assert!(secure || self.allow_insecure);

        format!(
            "{}://{}:{}",
            if secure { "wss" } else { "ws" },
            &self.addr,
            self.port,
        )
    }

    fn hashed_identity(&self) -> [u8; 512 / 8] {
        let mut sha = sha2::Sha512::new();
        sha.update(&self.identity[..]);
        sha.update(self.addr.as_bytes());
        sha.update(&self.port.to_le_bytes());
        let array = sha.finalize();
        array.as_slice().try_into().expect("512 / 8 == 64")
    }
}

#[derive(Debug, Clone, Copy)]
enum Step {
    SecureConnect,
    InsecureConnect,
    VersionHandshake,
    AcceptReject,
    Download,
    Game,
}
