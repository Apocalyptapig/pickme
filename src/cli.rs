#![cfg(feature = "cli")]

use rgb::RGB8;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;
use device_query::{DeviceState, DeviceQuery};
use screenshots::Screen;
use cli_clipboard as clipboard;
use clap::Parser;
use crate::{App, GetMousePosition};
use serde::{Serialize, Deserialize};

pub fn run() {
    let mut app = App::new();
    let device_state = DeviceState::new();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let config_file = std::fs::read_to_string("pickme.toml");

    let config = match config_file {
        Ok(file) => toml::from_str(&file).unwrap(),
        Err(_) => Config::default(),
    };

    loop {
        let new = app.update_color(
            device_state.get_mouse().position()
                .get_color(Screen::all().unwrap()[0])
                .unwrap()
        );

        if device_state.get_mouse().button_pressed[3] {
            break;
        }

        if let Some(new) = new {
            let hex = format!("#{}", hex::encode(new));

            stdout.set_color(
                &config.clone().normal.color_spec(new)
            ).unwrap();

            write!(&mut stdout, "{hex}").unwrap();

            stdout.set_color(
                &ColorSpec::new()
            ).unwrap();

            write!(&mut stdout, "\n").unwrap();
        }
    };

    let color = app.color.unwrap();
    let hex = format!("#{}", hex::encode(color));

    stdout.set_color(
        &config.selected.color_spec(color)
    ).unwrap();

    writeln!(&mut stdout, "> {hex} <").unwrap();

    clipboard::set_contents(hex).unwrap()
}

trait From<T>: Sized {
    fn from_rgb8(value: T) -> Self;
}

impl From<RGB8> for Color {
    fn from_rgb8(value: RGB8) -> Self {
        Self::Rgb(value.r, value.g, value.b)
    }
}

trait Into<T>: Sized {
    fn into_termcolor(self) -> T;
}

impl Into<Color> for RGB8 {
    fn into_termcolor(self) -> Color {
        Color::from_rgb8(self)
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    stop_buttons: Vec<usize>,
    normal: FormatMode,
    selected: FormatMode,
    selected_formatting: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            normal: FormatMode::default(),
            selected: {
                let mut format_mode = FormatMode::default();
                format_mode.markdown = vec!["bold".to_string()];
                format_mode
            },
            stop_buttons: vec![3, 4, 5],
            selected_formatting: "> {} <".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct FormatMode {
    foreground: String,
    background: String,
    markdown: Vec<String>
}

impl Default for FormatMode {
    fn default() -> Self {
        Self {
            foreground: "bright".to_string(),
            background: "none".to_string(),
            markdown: vec![]
        }
    }
}

impl FormatMode {
    fn color_spec(self, color: RGB8) -> ColorSpec {
        let mut color_spec = ColorSpec::new();

        match self.foreground.as_str() {
            "bright" => { color_spec.set_fg( Some(color.into_termcolor()) ); }
            "dark" => { color_spec.set_fg( Some(color.darken().into_termcolor()) ); }
            _ => (),
        };

        match self.background.as_str() {
            "bright" => { color_spec.set_bg( Some(color.into_termcolor()) ); }
            "dark" => { color_spec.set_bg( Some(color.darken().into_termcolor()) ); }
            _ => (),
        }

        color_spec.set_bold(
            self.markdown.contains(&"bold".to_string()) 
        );

        color_spec.set_italic(
            self.markdown.contains(&"italic".to_string()) 
        );

        color_spec
    }
}

trait Darken {
    fn darken(&self) -> Self;
}

impl Darken for RGB8 {
    fn darken(&self) -> Self {
        Self {
            r: self.r / 3,
            g: self.g / 3,
            b: self.b / 3,
        }
    }
}