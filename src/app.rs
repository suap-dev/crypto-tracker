use iced::widget::{button, column, text};
use iced::{Element, Task};

use crate::api::get_data;
use crate::message::Message;

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Increment => {
            state.counter = state.counter.wrapping_add(1);
            Task::none()
        }
        Message::Decrement => {
            state.counter = state.counter.wrapping_sub(1);
            Task::none()
        }
        Message::RequestData => Task::perform(get_data(), Message::Update),
        Message::Update(data) => match data {
            Ok(message) => {
                state.stuff = message;
                Task::none()
            }
            Err(err) => {
                state.stuff = format!("{:?}", err);
                Task::none()
            }
        },
    }
}

pub fn view(state: &State) -> Element<Message> {
    column![
        button(text('+')).on_press(Message::Increment),
        text(state.counter),
        button(text('-')).on_press(Message::Decrement),
        text(state.stuff.clone()),
        button(text("Request")).on_press(Message::RequestData)
    ]
    .into()
}

#[derive(Default, Clone)]
pub struct State {
    counter: i8,
    stuff: String,
}
