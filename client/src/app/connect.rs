use std::cell::RefCell;
use std::rc::Rc;


pub struct Connect {
    link: ComponentLink<Self>,
    props: Properties,
    status_log: Vec<String>,
}

impl Component for Connect {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Properties, link: ComponentLink<Self>) -> Self {
        let addr = format!("wss://{}:{}", props.addr, props.port);


        Self {
            link,
            props,
            status_log: vec![format!("Connecting to {}", addr)],
            ws,
            step: Step::SecureConnect,
        }
    }

    fn update(&mut self, msg: Message) -> ShouldRender {
        match msg {
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        unreachable!()
    }

    fn view(&self) -> Html {
        html! {
            <div style="max-width: 640px; margin: 0 auto;">
                <h1>{ "rust-web-game-skeleton" }</h1>
                <h2>{ "Connecting\u{2026}" }</h2>
                <ul>{ for self.status_log.iter().map(|status| html! {
                    <li>{ status }</li>
                }) } </ul>
            </div>
        }
    }
}

pub enum Message {
    WsStatus(WebSocketStatus),
    WsReceive(anyhow::Result<Vec<u8>>),
}

#[derive(Clone, Properties)]
pub struct Properties {
    pub ready_hook: Callback<super::GameArgs>,
    pub error_hook: Callback<Option<String>>,
}

impl Properties {
}

