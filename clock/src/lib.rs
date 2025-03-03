#![no_std]

use core::fmt::Debug;

use chrono::Timelike;
use embedded_graphics::{
    draw_target::DrawTarget,
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
    async fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        _vibration_motor: &mut impl shared::VibrationMotor,
        _buzzer: &mut impl shared::Buzzer,
        display: &mut D,
        _keypad: &mut impl shared::Keypad,
        rtc: &mut impl shared::Rtc,
        _backlight: &mut impl shared::Backlight,
        _system_response: Option<[u8; 64]>,
    ) -> Option<shared::SystemRequest>
    where
        <D as DrawTarget>::Error: Debug,
    {
        let fill = PrimitiveStyle::with_fill(BinaryColor::On);
        display
            .bounding_box()
            .into_styled(fill)
            .draw(display)
            .unwrap();

        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::Off);
        let now = chrono::DateTime::<chrono::Utc>::from_timestamp(rtc.timestamp(), 0).unwrap();
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
            display.bounding_box().center() + Point::new(0, 6),
            character_style,
            Alignment::Center,
        )
        .draw(display)
        .unwrap();
        embassy_time::Timer::after_millis(10).await;

        None
    }
}
