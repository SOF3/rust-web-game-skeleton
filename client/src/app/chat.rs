use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use yew::prelude::*;

pub struct ChatComp {
    link: ComponentLink<Self>,
    props: Properties,
    input_ref: NodeRef,
}

impl Component for ChatComp {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Message) -> ShouldRender {
        match msg {
            Message::InputKeyDown(event) => match event.code().as_str() {
                "Enter" => {
                    let msg = self.input_ref.cast::<web_sys::HtmlInputElement>().expect("Input element has incorrect type");
                    self.props.send_chat_callback.emit(msg.value());
                    msg.set_value("");
                    true
                }
                _ => false,
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div style=&self.props.style>
                { for self.props.messages.messages().map(|chat| html! {
                    <div>
                        { "<" }
                        <span>{ chat.speaker.as_str() }</span>
                        { "> " }
                        <span>{ chat.content.as_str() }</span>
                    </div>
                }) }
                { for self.props.has_input.then(|| html! {
                    <div>
                        <input
                            type="text"
                            placeholder="Enter chat here"
                            focused=true
                            onkeydown=self.link.callback(Message::InputKeyDown)
                            ref=self.input_ref.clone()
                            />
                    </div>
                }) }
            </div>
        }
    }
}

pub enum Message {
    InputKeyDown(KeyboardEvent),
}

#[derive(Clone, Properties)]
pub struct Properties {
    pub style: String,
    pub messages: List,
    pub has_input: bool,
    pub send_chat_callback: Callback<String>,
}

pub struct Chat {
    pub speaker: String,
    pub content: String,
}

#[derive(Clone)]
pub struct List {
    pub deque: Rc<RefCell<VecDeque<Chat>>>,
    pub size: usize,
}

impl List {
    pub fn new(size: usize) -> Self {
        Self {
            deque: Rc::default(),
            size,
        }
    }

    pub fn push(&self, message: Chat) {
        let deque = &mut *self.deque.borrow_mut();
        deque.push_back(message);
    }

    pub fn push_system(&self, content: String) {
        self.push(Chat {
            speaker: String::from("Traffloat"),
            content,
        })
    }

    pub fn messages(&self) -> impl Iterator<Item = std::cell::Ref<'_, Chat>> {
        use std::cell::Ref;

        let borrow = self.deque.borrow();
        let size = borrow.len();
        (0..size).map(move |index| {
            Ref::map(Ref::clone(&borrow), |deque| {
                deque.get(index).expect("index < size")
            })
        })
    }
}
