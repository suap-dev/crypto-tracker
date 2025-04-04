use iced::widget::{button, column, text};
use iced::{Element, Task};
use std::time::Instant;

fn main() -> iced::Result {
    iced::run("Cryograph", update, view)
}

const MIN_API: &str = "https://min-api.CRYPTOCOMPARE.com";
const GET_BTC_EUR: &str = "/data/v2/histominute?fsym=BTC&tsym=EUR&limit=1000";
fn update(state: &mut State, message: Message) -> Task<Message> {
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

async fn get_data() -> Result<String, CryoError> {
    let now = Instant::now();

    let response = reqwest::get(format!("{} {}", MIN_API, GET_BTC_EUR))
        .await
        .map_err(|e| CryoError::FetchError(format!("{:?}", e)))?;

    let raw_data = response
        .text()
        .await
        .map_err(|e| CryoError::ParseError(format!("{:?}", e)))?;

    let elapsed = now.elapsed();

    Ok(format!(
        "Got response with {} characters in {} miliseconds",
        raw_data.len(),
        elapsed.as_millis()
    ))
}

fn view(state: &State) -> Element<Message> {
    column![
        button(text('+')).on_press(Message::Increment),
        text(state.counter),
        button(text('-')).on_press(Message::Decrement),
        text(state.stuff.clone()),
        button(text("Request")).on_press(Message::RequestData)
    ]
    .into()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    RequestData,
    Update(Result<String, CryoError>),
}

#[derive(Default, Clone)]
struct State {
    counter: i8,
    stuff: String,
}

#[derive(Debug, Clone)]
pub enum CryoError {
    FetchError(String),
    ParseError(String),
}
