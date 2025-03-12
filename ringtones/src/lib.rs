#![no_std]
#![feature(ascii_char_variants)]
#![feature(ascii_char)]

use shared::Application;

pub struct Ringtones<'a>(shared::textbox::Textbox<'a>);

use core::fmt::Debug;

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};

impl<'a> Ringtones<'a> {
    pub fn new<D: DrawTarget<Color = BinaryColor>>(
        draw_target: &mut D,
        buffer: &'a mut [u8],
    ) -> Self
    where
        <D as DrawTarget>::Error: Debug,
    {
        Self(shared::textbox::Textbox::new(draw_target, buffer))
    }
}

impl Application for Ringtones<'_> {
    async fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        _vibration_motor: &mut impl shared::VibrationMotor,
        _buzzer: &mut impl shared::Buzzer,
        display: &mut D,
        keypad: &mut impl shared::Keypad,
        _rtc: &mut impl shared::Rtc,
        _backlight: &mut impl shared::Backlight,
        _system_response: Option<[u8; 64]>,
    ) -> Option<shared::SystemRequest>
    where
        <D as DrawTarget>::Error: Debug,
    {
        self.0.process(display, keypad).await;
        None
    }
}
