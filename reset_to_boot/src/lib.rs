#![no_std]

use core::fmt::Debug;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::PrimitiveStyle,
    text::{Alignment, Text},
};
use shared::Application;

pub struct ResetToBoot;

impl ResetToBoot {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ResetToBoot {
    fn default() -> Self {
        Self
    }
}

impl Application for ResetToBoot {
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
        None

    }
}
