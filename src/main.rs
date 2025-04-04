mod api;
mod app;
mod error;
mod message;

use crate::app::{update, view};

fn main() -> iced::Result {
    iced::run("Cryograph", update, view)
}
