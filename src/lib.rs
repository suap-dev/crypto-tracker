use iced::{Application, Command, Element, Theme, executor, widget};

pub struct App;

impl Application for App {
    type Executor = executor::Default;

    type Message = ();

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("Cryograph")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        Element::from(widget::Text::new("Cryograph"))
    }
}
