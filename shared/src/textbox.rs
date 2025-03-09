#![no_std]

use core::fmt::Debug;
use core::ascii::Char;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Rectangle},
};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::text::Text;
use embedded_graphics::text::Alignment;

use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawBE},
};

pub enum Case {
    Upper,
    Lower,
    Number
}

pub struct Textbox<'a> {
    bounding_box: Rectangle,
    cursor: Point,
    multitap: crate::multitap::MultiTap,
    buffer: &'a mut [u8],
    index: usize,
    first_draw: bool,
    case: Case
}

impl <'a>Textbox<'a> {
    pub fn new<D: DrawTarget<Color = BinaryColor>>(draw_target: &mut D, buffer: &'a mut [u8]) -> Self where <D as DrawTarget>::Error: Debug {

        Self {
            bounding_box: Rectangle::new(Point::new(0,0), Size::new(84, 48)),
            cursor: Point::new(0, 9),
            multitap: crate::multitap::MultiTap::new(),
            buffer,
            index: 0,
            first_draw: true,
            case: Case::Lower
        }
    }

    // should return emitted characters
    pub async fn process<D: DrawTarget<Color = BinaryColor>>(&mut self, display: &mut D, keypad: &mut impl crate::Keypad) -> Option<Char> where <D as DrawTarget>::Error: Debug {
        if self.first_draw {
            self.first_draw = false;
            display.clear(BinaryColor::On).unwrap();
            self.draw_border(display);
            self.draw_case_icon(display);
        }

        match self.multitap.event(keypad, embassy_time::Timer::after_millis(1500), &self.case).await {
            crate::multitap::Event::Tentative(c) => {
                if c == Char::Backspace {
                    self.backspace(display);
                    return Some(c);
                } else {

                    self.push_tentative(display, c.into());
                }
            },
            crate::multitap::Event::Decided(c) => {
                if c != Char::Backspace {
                    self.push(display, c.into());
                    return Some(c);
                }
            }
            crate::multitap::Event::Case => {
                self.case = match self.case {
                    Case::Upper => Case::Lower,
                    Case::Lower => Case::Upper,
                    _ => Case::Lower
                };
                self.draw_case_icon(display);
            }
        }
        None
    }

    fn draw_border<D: DrawTarget<Color = BinaryColor>>(&mut self, draw_target: &mut D)
    where
        <D as DrawTarget>::Error: Debug {
        let pencil = [
            0b11101010,  0b1011_1111,
            0b11110101,  0b0101_1111,
            0b11111010,  0b1010_1111,
            0b11111101,  0b0110_1111,
            0b11111110,  0b1100_1111,
            0b11111111,  0b0000_1111,
        ];
        let raw: ImageRawBE<BinaryColor> = ImageRaw::new(&pencil, 16);
        let image = Image::new(&raw, Point::zero());
        image.draw(draw_target).unwrap();

    }

    fn draw_case_icon<D: DrawTarget<Color = BinaryColor>>(&mut self, draw_target: &mut D)
    where
        <D as DrawTarget>::Error: Debug {
        let icon = match self.case {
            Case::Upper => [
                0b1000_1100,0b0011_1000,
                0b0010_0100,0b1001_0011,
                0b0010_0100,0b0011_0011,
                0b0000_0100,0b1001_0011,
                0b0010_0100,0b1001_0011,
                0b0010_0100,0b0011_1000,
            ],
            Case::Lower => [
                0b1111_1100,0b1111_1111,
                0b1111_1100,0b1111_1111,
                0b1000_0100,0b0011_1000,
                0b0010_0100,0b1001_0011,
                0b0010_0100,0b1001_0011,
                0b1000_0100,0b0011_1000,
            ],
            Case::Number => [
                0b1111_1100,0b1111_1111,
                0b1111_1100,0b1111_1111,
                0b1000_0100,0b0011_1000,
                0b0010_0100,0b1001_0011,
                0b0010_0100,0b1001_0011,
                0b1000_0100,0b0011_1000,
            ]
        };
        let raw: ImageRawBE<BinaryColor> = ImageRaw::new(&icon, 16);
        let image = Image::new(&raw, Point::new(16, 0));
        image.draw(draw_target).unwrap();
    }

    fn push<D: DrawTarget<Color = BinaryColor>>(&mut self, draw_target: &mut D, character: char)
    where
        <D as DrawTarget>::Error: Debug,
    {
        self.buffer[self.index] = character as u8;
        self.index += 1;

        let renderer = MonoTextStyle::new(&FONT_6X9, BinaryColor::Off);
        let mut b = [0; 4];

        Rectangle::new(
            self.cursor, Size::new(6, 9)
        ).into_styled(PrimitiveStyle::with_fill(BinaryColor::On)).draw(draw_target).unwrap();

        if let Ok(g) = renderer.draw_string(
            character.encode_utf8(&mut b),
            self.cursor,
            embedded_graphics::text::Baseline::Top,
            draw_target
        ) {
            if g.x >= 80 {
                self.cursor = Point::new(0, g.y + 9);
            } else {
                self.cursor = g;
            }
        }
    }

    fn backspace<D: DrawTarget<Color = BinaryColor>>(&mut self, draw_target: &mut D) where <D as DrawTarget>::Error: Debug {
        if self.index > 0 {
            self.index -= 1;
            self.buffer[self.index] = 0;
        }

        self.cursor = self.cursor - Point::new(6, 0);
        if self.cursor.x < 0 {
            self.cursor.y -= 9;
            self.cursor.x = 78;
        }
        if self.cursor.x <= 0 && self.cursor.y <= 9 {
            self.cursor.x = 0;
            self.cursor.y = 9;
        }

        Rectangle::new(
            self.cursor, Size::new(6, 9)
        ).into_styled(PrimitiveStyle::with_fill(BinaryColor::On)).draw(draw_target).unwrap();
    }

    fn push_tentative<D: DrawTarget<Color = BinaryColor>>(&mut self, draw_target: &mut D, character: char)
    where
        <D as DrawTarget>::Error: Debug,
    {
        let renderer = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
        let mut b = [0; 4];

        Rectangle::new(
            self.cursor, Size::new(6, 9)
        ).into_styled(PrimitiveStyle::with_fill(BinaryColor::Off)).draw(draw_target).unwrap();

        if let Ok(g) = renderer.draw_string(
            character.encode_utf8(&mut b),
            self.cursor,
            embedded_graphics::text::Baseline::Top,
            draw_target
        ) {
        }
    }
}
