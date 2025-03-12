#![no_std]

use core::fmt::Debug;

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};
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
        _display: &mut D,
        _keypad: &mut impl shared::Keypad,
        _rtc: &mut impl shared::Rtc,
        _backlight: &mut impl shared::Backlight,
        _system_response: Option<[u8; 64]>,
    ) -> Option<shared::SystemRequest>
    where
        <D as DrawTarget>::Error: Debug,
    {
        None
    }
}
