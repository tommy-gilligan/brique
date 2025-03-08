#![no_std]

use shared::Application;

pub struct Ringtones(Textbox);

use core::fmt::Debug;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Rectangle},
};
use embedded_graphics::text::renderer::TextRenderer;
// use embedded_graphics::geometry::point::Point;

#[derive(Clone, PartialEq)]
pub struct Textbox {
    bounding_box: Rectangle,
    cursor: Point
}

impl Textbox {
    pub fn new<D: DrawTarget<Color = BinaryColor>>(draw_target: &mut D) -> Self where <D as DrawTarget>::Error: Debug {
        draw_target.clear(BinaryColor::On).unwrap();
        Self {
            bounding_box: Rectangle::new(Point::new(0,0), Size::new(84, 48)),
            cursor: Point::new(0, 0)
        }
    }

    pub fn push<D: DrawTarget<Color = BinaryColor>>(&mut self, draw_target: &mut D, character: char)
    where
        <D as DrawTarget>::Error: Debug,
    {
        let renderer = MonoTextStyle::new(&FONT_6X10, BinaryColor::Off);
        let mut b = [0; 4];

        if let Ok(g) = renderer.draw_string(
            character.encode_utf8(&mut b),
            self.cursor,
            embedded_graphics::text::Baseline::Top,
            draw_target
        ) {
            if g.x > 84 {
                self.cursor = Point::new(0, g.y + 10);
            } else {
                self.cursor = g;
            }
        }
    }
}

const MISSION: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl Ringtones {
    pub fn new<D: DrawTarget<Color = BinaryColor>>(draw_target: &mut D) -> Self where <D as DrawTarget>::Error: Debug {
        Self(Textbox::new(draw_target))
    }
}

impl Application for Ringtones {
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
        if self.0.cursor == Point::new(0, 0) {
            for character in MISSION.chars() {
                self.0.push(display, character);
            }
        }
        None
    }
}
