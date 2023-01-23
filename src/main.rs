use cli_clipboard as clipboard;
use device_query::{DeviceState, DeviceQuery, MouseState};
use image::{io::Reader as ImageReader, GenericImageView, ImageError};
use rgb::{RGB, RGB8};
use screenshots::Screen;
use std::io::Cursor;

mod gui;
mod cli;

struct App {
    color: Option<RGB<u8>>,
}

fn main() {
    //set_dpi(1.0);

    cli::run();
    //gui::iced::run();
    
    /*let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "pickme",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );*/
}

impl uMousePosition {
    fn get_color(self, screen: Screen) -> Result<RGB<u8>, ImageError> {
        let mut point = self;

        let screenshot = loop {
            if point.x < 0 || point.y < 0 {
                point = DeviceState::new().get_mouse().position().unsigned();
                continue;
            }

            let image = screen.capture_area(point.x, point.y, 1, 1);

            if let Some(ss) = image {
                break ss;
            }
        };

        let buffer = screenshot.buffer();

        let img = ImageReader::new(Cursor::new(buffer))
            .with_guessed_format()?
            .decode()?;

        let pixel = img.get_pixel(0, 0).0;
        let pixel = RGB {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
        };

        Ok(pixel)
    }
}

trait ToUnsigned {
    type Output;

    fn unsigned(self) -> Self::Output;
}

impl ToUnsigned for i32 {
    type Output = i32;

    fn unsigned(self) -> Self::Output {
        if self < 0 {
            return 0;
        } else {
            return self as Self::Output;
        }
    }
}

impl ToUnsigned for iMousePosition {
    type Output = uMousePosition;

    fn unsigned(self) -> Self::Output {
        Self::Output {
            x: self.x.unsigned(),
            y: self.y.unsigned(),
        }
    }
}

fn set_dpi(scale: f32) {
    use winapi::um::shellscalingapi::SetProcessDpiAwareness;

    let dpi = scale * 100.0;

    unsafe {
        SetProcessDpiAwareness(dpi as u32);
    }
}

impl App {
    fn new() -> Self {
        Self {
            color: None,
        }
    }

    fn update_color(&mut self, color: RGB8) -> Option<RGB8> {
        if let Some(previous) = self.color {
            if color != previous {
                self.color = Some(color);
            } else {
                return None;
            }
        } else {
            self.color = Some(color);
        }

        self.color
    }
}

#[derive(Debug)]
struct MousePosition<T> {
    x: T,
    y: T,
}

#[allow(non_camel_case_types)]
type iMousePosition = MousePosition<i32>;

#[allow(non_camel_case_types)]
type uMousePosition = MousePosition<i32>;

trait GetMousePosition {
    fn position(&self) -> iMousePosition;
}

impl GetMousePosition for MouseState {
    fn position(&self) -> iMousePosition {
        iMousePosition {
            x: self.coords.0,
            y: self.coords.1,
        }
    }
}