use iced::widget::{button, column, text};
use iced::Application;

struct App {
    value: i32,
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::theme::Theme;

    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (App { value: 42 }, iced::Command::none())
    }

    fn title(&self) -> String {
        "Firefly Emulator".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let widget = column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),
            // We show the value of the counter here
            text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ];
        widget.into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

fn main() -> iced::Result {
    let settings = iced::Settings {
        antialiasing: true,
        ..Default::default()
    };
    App::run(settings)
}
