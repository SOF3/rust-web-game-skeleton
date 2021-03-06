#![allow(clippy::unwrap_used)]

use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::services::websocket::WebSocketTask;

macro_rules! css {
    ($($key:literal : $value:expr);* $(;)?) => {
        format!(
            concat!(
                $($key, ": {}; ",)*
            ),
            $($value,)*
        )
    }
}

mod chat;
mod game;
mod menu;

pub struct Lifecycle {
    link: ComponentLink<Self>,
    state: State,
    config: MenuConfig,
}

impl Component for Lifecycle {
    type Message = Message;
    type Properties = Properties;

    fn create((): Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: State::Menu { err: None },
            config: read_menu_config(),
        }
    }
    fn update(&mut self, msg: Message) -> ShouldRender {
        match &self.state {
            State::Menu { .. } => match msg {
                Message::StartGame(args) => {
                    self.config.addr = args.addr.clone();
                    self.config.port = args.port;
                    self.config.allow_insecure = args.allow_insecure;
                    self.config.name = args.name.clone();
                    write_menu_config(&self.config);
                    self.state = State::Game { args };
                    true
                }
                Message::GameError(err) => {
                    self.state = State::Menu { err };
                    true
                }
            },
            State::Game { .. } => match msg {
                Message::GameError(err) => {
                    self.state = State::Menu { err };
                    true
                }
                _ => unreachable!(),
            },
        }
    }

    fn change(&mut self, (): Self::Properties) -> ShouldRender {
        unreachable!()
    }

    fn view(&self) -> Html {
        match &self.state {
            State::Menu { err } => html! {
                <menu::Menu
                    addr=&self.config.addr
                    port=self.config.port
                    allow_insecure=self.config.allow_insecure
                    name=&self.config.name
                    err=err
                    connect_hook=self.link.callback(Message::StartGame) />
            },
            State::Game { args } => html! {
                <game::Game
                    addr=&args.addr port=args.port allow_insecure=args.allow_insecure
                    name=&args.name
                    identity=&self.config.identity
                    error_hook=self.link.callback(Message::GameError)
                    />
            },
        }
    }
}

enum State {
    Menu { err: Option<String> },
    Game { args: GameArgs },
}

pub struct ClientArgs {}

#[derive(Debug, Clone)]
pub struct GameArgs {
    addr: String,
    port: u16,
    allow_insecure: bool,
    name: String,
}

pub enum Message {
    StartGame(GameArgs),
    GameError(Option<String>),
}

// #[derive(Clone, Default, Properties)]
// pub struct Properties {}
type Properties = ();

const MENU_CONFIG_STORAGE_KEY: &str = "rust-web-game-skeleton:menu_config";

#[derive(serde::Serialize, serde::Deserialize)]
struct MenuConfig {
    identity: Vec<u8>,
    addr: String,
    port: u16,
    allow_insecure: bool,
    name: String,
}

fn read_menu_config() -> MenuConfig {
    let storage = StorageService::new(Area::Local).unwrap();
    let Json(value): Json<anyhow::Result<MenuConfig>> = storage.restore(MENU_CONFIG_STORAGE_KEY);
    match value {
        Ok(config) => config,
        _ => {
            let identity: Vec<u8> = rand::thread_rng()
                .sample_iter(rand::distributions::Standard)
                .take(512 / 8)
                .collect();
            let addr = "localhost".to_owned();
            let port = common::DEFAULT_PORT;
            let allow_insecure = false;
            let name = "Player".to_owned();
            let config = MenuConfig {
                identity,
                addr,
                port,
                allow_insecure,
                name,
            };
            write_menu_config(&config);
            config
        }
    }
}

fn write_menu_config(config: &MenuConfig) {
    let mut storage = StorageService::new(Area::Local).unwrap();
    storage.store(MENU_CONFIG_STORAGE_KEY, Json(&config));
}
