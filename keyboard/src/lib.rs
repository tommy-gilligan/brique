#![no_std]

use core::fmt::Debug;

use embedded_graphics::{
    draw_target::DrawTarget,
    // mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::PrimitiveStyle,
    // text::{Alignment, Text},
};
use shared::{Application, Key, KeyEvent};

pub struct Keyboard;

impl Keyboard {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self
    }
}

impl Application for Keyboard {
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
        let fill = PrimitiveStyle::with_fill(BinaryColor::Off);
        display
            .bounding_box()
            .into_styled(fill)
            .draw(display)
            .unwrap();

        // let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::Off);
        // let now = chrono::DateTime::<chrono::Utc>::from_timestamp(rtc.timestamp(), 0).unwrap();
        // let mut text: heapless::String<8> = heapless::String::new();
        // Text::with_alignment(
        //     &text,
        //     display.bounding_box().center() + Point::new(0, 6),
        //     character_style,
        //     Alignment::Center,
        // )
        // .draw(display)
        // .unwrap();

        match keypad.event().await {
            KeyEvent::Down(Key::Down) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('d'))),
            KeyEvent::Down(Key::Up) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('u'))),
            KeyEvent::Down(Key::One) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('1'))),
            KeyEvent::Down(Key::Two) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('2'))),
            KeyEvent::Down(Key::Four) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('4'))),
            KeyEvent::Down(Key::Five) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('5'))),
            KeyEvent::Down(Key::Six) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('6'))),
            KeyEvent::Down(Key::Eight) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('8'))),
            KeyEvent::Down(Key::Seven) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('7'))),
            KeyEvent::Down(Key::Nine) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('9'))),
            KeyEvent::Down(Key::Three) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('3'))),
            KeyEvent::Down(Key::Select) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('s'))),
            KeyEvent::Down(Key::Cancel) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('c'))),
            KeyEvent::Down(Key::Asterisk) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('*'))),
            KeyEvent::Down(Key::Zero) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('0'))),
            KeyEvent::Down(Key::Hash) => Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar('#'))),
            KeyEvent::Up(_) => None,
        }
    }
}
