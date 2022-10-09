use iced::widget::{column, text, text_input};
use iced::{executor, Application, Command, Element, Settings, Theme};

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello {
    text_value: String,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
}

impl Application for Hello {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (
            Self {
                theme: match dark_light::detect() {
                    dark_light::Mode::Dark => Theme::Dark,
                    dark_light::Mode::Light => Theme::Light,
                },
                text_value: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InputChanged(value) => self.text_value = value,
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        column![
            text("Hello, world!"),
            text_input(
                "Type something to continue...",
                &self.text_value,
                Message::InputChanged,
            )
            .padding(10)
        ]
        .spacing(10)
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme
    }
}
