#![no_std]
#![feature(ascii_char_variants)]
#![feature(ascii_char)]

use shared::Application;

pub struct Keyboard<'a>(shared::textbox::Textbox<'a>);

use core::fmt::Debug;
use core::ascii::Char;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Rectangle},
};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::text::Text;
use embedded_graphics::text::Alignment;

impl <'a>Keyboard<'a> {
    pub fn new<D: DrawTarget<Color = BinaryColor>>(draw_target: &mut D, buffer: &'a mut [u8]) -> Self where <D as DrawTarget>::Error: Debug {
        Self(
            shared::textbox::Textbox::new(draw_target, buffer),
        )
    }
}
use usbd_hid::descriptor::KeyboardUsage;

fn build_report(c: Char) -> usbd_hid::descriptor::KeyboardReport {
    let keycode = match c {
        Char::Space => KeyboardUsage::KeyboardSpacebar,
        Char::Digit0 => KeyboardUsage::Keyboard0CloseParens,
        Char::Digit1 => KeyboardUsage::Keyboard1Exclamation,
        Char::Digit2 => KeyboardUsage::Keyboard2At,
        Char::Digit3 => KeyboardUsage::Keyboard3Hash,
        Char::Digit4 => KeyboardUsage::Keyboard4Dollar,
        Char::Digit5 => KeyboardUsage::Keyboard5Percent,
        Char::Digit6 => KeyboardUsage::Keyboard6Caret,
        Char::Digit7 => KeyboardUsage::Keyboard7Ampersand,
        Char::Digit8 => KeyboardUsage::Keyboard8Asterisk,
        Char::Digit9 => KeyboardUsage::Keyboard9OpenParens,
        Char::CapitalA => KeyboardUsage::KeyboardAa,
        Char::CapitalB => KeyboardUsage::KeyboardBb,
        Char::CapitalC => KeyboardUsage::KeyboardCc,
        Char::CapitalD => KeyboardUsage::KeyboardDd,
        Char::CapitalE => KeyboardUsage::KeyboardEe,
        Char::CapitalF => KeyboardUsage::KeyboardFf,
        Char::CapitalG => KeyboardUsage::KeyboardGg,
        Char::CapitalH => KeyboardUsage::KeyboardHh,
        Char::CapitalI => KeyboardUsage::KeyboardIi,
        Char::CapitalJ => KeyboardUsage::KeyboardJj,
        Char::CapitalK => KeyboardUsage::KeyboardKk,
        Char::CapitalL => KeyboardUsage::KeyboardLl,
        Char::CapitalM => KeyboardUsage::KeyboardMm,
        Char::CapitalN => KeyboardUsage::KeyboardNn,
        Char::CapitalO => KeyboardUsage::KeyboardOo,
        Char::CapitalP => KeyboardUsage::KeyboardPp,
        Char::CapitalQ => KeyboardUsage::KeyboardQq,
        Char::CapitalR => KeyboardUsage::KeyboardRr,
        Char::CapitalS => KeyboardUsage::KeyboardSs,
        Char::CapitalT => KeyboardUsage::KeyboardTt,
        Char::CapitalU => KeyboardUsage::KeyboardUu,
        Char::CapitalV => KeyboardUsage::KeyboardVv,
        Char::CapitalW => KeyboardUsage::KeyboardWw,
        Char::CapitalX => KeyboardUsage::KeyboardXx,
        Char::CapitalY => KeyboardUsage::KeyboardYy,
        Char::CapitalZ => KeyboardUsage::KeyboardZz,
        Char::SmallA => KeyboardUsage::KeyboardAa,
        Char::SmallB => KeyboardUsage::KeyboardBb,
        Char::SmallC => KeyboardUsage::KeyboardCc,
        Char::SmallD => KeyboardUsage::KeyboardDd,
        Char::SmallE => KeyboardUsage::KeyboardEe,
        Char::SmallF => KeyboardUsage::KeyboardFf,
        Char::SmallG => KeyboardUsage::KeyboardGg,
        Char::SmallH => KeyboardUsage::KeyboardHh,
        Char::SmallI => KeyboardUsage::KeyboardIi,
        Char::SmallJ => KeyboardUsage::KeyboardJj,
        Char::SmallK => KeyboardUsage::KeyboardKk,
        Char::SmallL => KeyboardUsage::KeyboardLl,
        Char::SmallM => KeyboardUsage::KeyboardMm,
        Char::SmallN => KeyboardUsage::KeyboardNn,
        Char::SmallO => KeyboardUsage::KeyboardOo,
        Char::SmallP => KeyboardUsage::KeyboardPp,
        Char::SmallQ => KeyboardUsage::KeyboardQq,
        Char::SmallR => KeyboardUsage::KeyboardRr,
        Char::SmallS => KeyboardUsage::KeyboardSs,
        Char::SmallT => KeyboardUsage::KeyboardTt,
        Char::SmallU => KeyboardUsage::KeyboardUu,
        Char::SmallV => KeyboardUsage::KeyboardVv,
        Char::SmallW => KeyboardUsage::KeyboardWw,
        Char::SmallX => KeyboardUsage::KeyboardXx,
        Char::SmallY => KeyboardUsage::KeyboardYy,
        Char::SmallZ => KeyboardUsage::KeyboardZz,
        _ => KeyboardUsage::KeyboardZz
    };

    usbd_hid::descriptor::KeyboardReport {
        keycodes: [keycode as u8, 0, 0, 0, 0, 0],
        leds: 0,
        modifier: 0,
        reserved: 0,
    }
}

impl Application for Keyboard<'_> {
    async fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        _vibration_motor: &mut impl shared::VibrationMotor,
        _buzzer: &mut impl shared::Buzzer,
        display: &mut D,
        keypad: &mut impl shared::Keypad,
        rtc: &mut impl shared::Rtc,
        _backlight: &mut impl shared::Backlight,
        _system_response: Option<[u8; 64]>,
    ) -> Option<shared::SystemRequest>
    where
        <D as DrawTarget>::Error: Debug,
    {
        if let Some(c) = self.0.process(display, keypad).await {
            Some(
                shared::SystemRequest::UsbTx(
                    shared::UsbTx::HidChar(build_report(c))
                )
            )
        } else {
            None
        }
    }
}
