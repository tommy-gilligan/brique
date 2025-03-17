#![no_std]

use chrono::Timelike;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::PrimitiveStyle,
    text::{Alignment, Text},
};
use shared::Application;

pub struct Clock;

impl Clock {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self
    }
}

// TODO: use something better
fn to_char(digit: u32) -> char {
    match digit {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => '?',
    }
}

impl Application for Clock {
    async fn run(
        &mut self,
        device: &mut impl shared::Device,
        _system_response: Option<[u8; 64]>,
    ) -> Result<Option<shared::SystemRequest>, ()> {
        let fill = PrimitiveStyle::with_fill(BinaryColor::On);
        device
            .bounding_box()
            .into_styled(fill)
            .draw(device)
            .unwrap();

        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::Off);
        if let Ok(timestamp) = device.timestamp() {
            let now = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp, 0).unwrap();
            let mut text: heapless::String<8> = heapless::String::new();

            text.push(to_char(now.hour() / 10)).unwrap();
            text.push(to_char(now.hour() % 10)).unwrap();
            text.push(':').unwrap();
            text.push(to_char(now.minute() / 10)).unwrap();
            text.push(to_char(now.minute() % 10)).unwrap();
            text.push(':').unwrap();
            text.push(to_char(now.second() / 10)).unwrap();
            text.push(to_char(now.second() % 10)).unwrap();

            Text::with_alignment(
                &text,
                device.bounding_box().center() + Point::new(0, 6),
                character_style,
                Alignment::Center,
            )
            .draw(device)
            .unwrap();
        }
        embassy_time::Timer::after_millis(10).await;

        Ok(None)
    }
}
