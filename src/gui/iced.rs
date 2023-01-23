#![cfg(feature = "iced")]

use iced::application::StyleSheet;
use iced::theme::Palette;
use iced::{Element, Application, Settings, executor, Command, Color, Subscription, subscription};
use iced_native::window::Action as WindowAction;
use iced_native::widget::Button;
use iced::widget::container;
use iced_native::command::Action as CommandAction;
use iced_native::event::Event;
use iced::window::Position;
use rgb::RGB8;
use crate::From;
use screenshots::Screen;
use iced::Theme;
use crate::{set_dpi, MousePosition};

struct App {
    color: Option<RGB8>,
    screen: Screen,
    theme: Theme
}

impl App {
    fn update_color(&mut self, color: RGB8) {
        if let Some(previous) = self.color {
            if color != previous {
                self.color = Some(color);
            }
        } else {
            self.color = Some(color);
        }
    }
}

pub fn run() -> iced::Result {
    let mut settings: Settings<()> = Settings::default();
    settings.window.size = (200, 200);
    settings.window.position = Position::Specific(100, 100);

    App::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    LostFocus,
    MouseMoved,
}

impl From<RGB8> for Color {
    fn rgb(value: RGB8) -> Self {
        Color::from_rgb8(
            value.r,
            value.g,
            value.b,
        )
    }
}

impl iced::Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        crate::set_dpi(1.5);

        let screen = Screen::all().unwrap()[0];

        (
            Self {
                color: None,
                screen,
                theme: Theme::default()
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        "pickme".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        if let Some(color) = self.color {
            let mut palette = Palette::LIGHT;
            palette.background = Color::rgb(color);
    
            let theme = Theme::custom(palette);

            self.style();
        }

        match message {
            Message::LostFocus => {
                //return iced::Command::single(iced_native::command::Action::Window(iced_native::window::Action::GainFocus))
            }
            /*Message::MouseMoved => {
                return Command::single(
                    let color = MousePosition::get_mouse_position()
                    .unwrap()
                    .get_color(self.screen)
                    .unwrap();
        
                    self.update_color(color);
                )
            }*/
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let hex = match self.color {
            Some(color) => format!("#{}", hex::encode(color)),
            None => "#000000".to_string()
        };

        let text = iced::widget::text(hex);

        container(
            text
        )
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        match event {
            Event::Window(window::Event::Unfocused)
            => subscription::run("sus", Message::LostFocus),

            Event::Mouse(mouse::Event::CursorMoved { position: _ })
                | Event::Mouse(mouse::Event::CursorLeft)
                | Event::Mouse(mouse::Event::CursorEntered)
            => Some(Message::MouseMoved),
    
            _ => Subscription::none()
        }
    }

    fn run(settings: Settings<Self::Flags>) -> iced::Result
        where
            Self: 'static, {
        
    }
}

use iced::event::Status;

fn lost_focus(event: iced::Event, _status: Status) -> Option<Message> {
    use iced::{window, mouse};

    match event {
        Event::Window(window::Event::Unfocused) => Some(Message::LostFocus),

        Event::Mouse(mouse::Event::CursorMoved { position: _ })
            | Event::Mouse(mouse::Event::CursorLeft)
            | Event::Mouse(mouse::Event::CursorEntered)
        => Some(Message::MouseMoved),

        _ => None
    }
}