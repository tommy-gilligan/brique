#![no_std]
#![feature(ascii_char_variants)]
#![feature(ascii_char)]

use core::fmt::Debug;

use embedded_graphics::{
    Drawable,
    draw_target::{DrawTarget, DrawTargetExt},
    image::{Image, ImageRaw, ImageRawBE},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle,
};
use shared::{Application, multitap, multitap::Case};

pub struct Keyboard<'a, const N: usize> {
    textbox: shared::textbox::Textbox<'a, heapless::String<N>>,
    case: crate::multitap::Case,
}

impl<'a, const N: usize> Keyboard<'a, N> {
    pub fn new(buffer: heapless::String<N>) -> Self {
        Self {
            textbox: shared::textbox::Textbox::new(buffer),
            case: crate::multitap::Case::Upper,
        }
    }

    fn draw_titlebar<D: DrawTarget<Color = BinaryColor>>(&mut self, draw_target: &mut D)
    where
        <D as DrawTarget>::Error: Debug,
    {
        let pencil = [
            0b11101010,
            0b1011_1111,
            0b11110101,
            0b0101_1111,
            0b11111010,
            0b1010_1111,
            0b11111101,
            0b0110_1111,
            0b11111111,
            0b0000_1111,
        ];
        let raw: ImageRawBE<BinaryColor> = ImageRaw::new(&pencil, 16);
        let image = Image::new(&raw, Point::zero());
        image.draw(draw_target).unwrap();
        let icon = match self.case {
            Case::Upper => [
                0b1000_1100,
                0b0011_1000,
                0b0010_0100,
                0b1001_0011,
                0b0010_0100,
                0b0011_0011,
                0b0000_0100,
                0b1001_0011,
                0b0010_0100,
                0b1001_0011,
                0b0010_0100,
                0b0011_1000,
            ],
            Case::Lower => [
                0b1111_1100,
                0b1111_1111,
                0b1111_1100,
                0b1111_1111,
                0b1000_0100,
                0b0011_1000,
                0b0010_0100,
                0b1001_0011,
                0b0010_0100,
                0b1001_0011,
                0b1000_0100,
                0b0011_1000,
            ],
            Case::Number => [
                0b1011_1011,
                0b0001_1111,
                0b0011_0101,
                0b1101_1111,
                0b1011_1101,
                0b1101_1111,
                0b1011_1101,
                0b1011_1111,
                0b1011_1011,
                0b1101_1111,
                0b0001_0001,
                0b0011_1111,
            ],
        };
        let raw: ImageRawBE<BinaryColor> = ImageRaw::new(&icon, 16);
        let image = Image::new(&raw, Point::new(16, 0));
        image.draw(draw_target).unwrap();
    }
}

impl<const N: usize> Application for Keyboard<'_, N> {
    async fn run(&mut self, device: &mut impl shared::Device) -> Result<(), ()> {
        let mut multitap = crate::multitap::MultiTap::new(1500);
        if let Some(crate::multitap::Event::Case(case)) = multitap.event(device).await {
            self.case = case;
            self.draw_titlebar(device);
        }

        let mut translated = device.translated(Point::new(0, 20));
        let mut clipped = translated.clipped(&Rectangle::new(Point::zero(), Size::new(84, 20)));
        self.textbox.draw(&mut clipped, None, false);

        loop {
            // needs to be just waiting on keypad
            // does not need to be waiting on display as well
            match multitap.event(device).await {
                Some(crate::multitap::Event::ShowSpecialCharacters) => {
                    let char_str = shared::character_select::process(device).await.unwrap();
                    let the_char = char_str.chars().next().unwrap();

                    // stack is necessary here?
                    // or at least clear
                    // + redrawing decoration
                    // + redrawing all of buffer
                    let _ = device.clear(BinaryColor::On);
                    let mut translated = device.translated(Point::new(0, 20));
                    let mut clipped =
                        translated.clipped(&Rectangle::new(Point::zero(), Size::new(84, 20)));

                    self.textbox.push(&mut clipped, the_char, false);
                    self.textbox.draw(&mut clipped, None, false);
                }
                Some(crate::multitap::Event::Case(case)) => {
                    self.case = case;
                    self.draw_titlebar(device);
                }
                Some(crate::multitap::Event::Decided(core::ascii::Char::Backspace)) => {
                    let mut translated = device.translated(Point::new(0, 20));
                    let mut clipped =
                        translated.clipped(&Rectangle::new(Point::zero(), Size::new(84, 20)));
                    self.textbox.backspace(&mut clipped);
                }
                Some(crate::multitap::Event::Decided(c)) => {
                    let mut translated = device.translated(Point::new(0, 20));
                    let mut clipped =
                        translated.clipped(&Rectangle::new(Point::zero(), Size::new(84, 20)));
                    self.textbox.push(&mut clipped, c.into(), false);
                }
                Some(crate::multitap::Event::Tentative(c)) => {
                    let mut translated = device.translated(Point::new(0, 20));
                    let mut clipped =
                        translated.clipped(&Rectangle::new(Point::zero(), Size::new(84, 20)));
                    self.textbox.push(&mut clipped, c.into(), true);
                }
                _ => {}
            }
        }
    }
}
