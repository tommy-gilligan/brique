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

// TODO: use something better
pub fn to_char(digit: u32) -> char {
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

pub fn write_time(
    rtc: &mut impl crate::Rtc,
    seconds: bool
) -> heapless::String<8> {
    let now = chrono::DateTime::<chrono::Utc>::from_timestamp(rtc.timestamp(), 0).unwrap();
    let mut text = heapless::String::new();

    text.push(to_char(now.hour() / 10)).unwrap();
    text.push(to_char(now.hour() % 10)).unwrap();
    text.push(':').unwrap();
    text.push(to_char(now.minute() / 10)).unwrap();
    text.push(to_char(now.minute() % 10)).unwrap();

    if seconds {
        text.push(':').unwrap();
        text.push(to_char(now.second() / 10)).unwrap();
        text.push(to_char(now.second() % 10)).unwrap();
    }
    text
}
