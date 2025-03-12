use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::{FONT_4X6, FONT_6X9, FONT_6X13}},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};

use core::{future::Future};
use futures::{future, future::Either, pin_mut};
use embedded_graphics::image::ImageRawBE;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::image::Image;
use embedded_graphics::primitives::Line;

use super::Keypad;

pub struct LockScreen<'a> {
    locked: bool,
    menu_open: bool,
    items: &'a [&'a str],
    index: usize,
}

impl <'a>LockScreen<'a> {
    pub fn new(items: &'a [&'a str]) -> Self {
        Self {
            locked: true,
            menu_open: false,
            items,
            index: 0
        }
    }

    fn draw_scrollbar<D>(&mut self, draw_target: &mut D)
    where
        D: DrawTarget<Color = BinaryColor>, <D as embedded_graphics::draw_target::DrawTarget>::Error: core::fmt::Debug
    {
        Line::new(Point::new(81, 7), Point::new(81, 37))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
            .draw(draw_target).unwrap();

        let top: f32 = 7.0;
        let bottom: f32 = 36.0;
        let factor: f32 = (bottom - top) / (self.items.len() as f32);
        let actual_top: i32 = (factor * (self.index as f32) + top) as i32;

        Line::new(Point::new(83, actual_top + 1), Point::new(83, actual_top + 6))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
            .draw(draw_target).unwrap();
        Line::new(Point::new(81, actual_top + 1), Point::new(81, actual_top + 6))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(draw_target).unwrap();
        Line::new(Point::new(81, actual_top), Point::new(83, actual_top))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
            .draw(draw_target).unwrap();
        Line::new(Point::new(81, actual_top + 7), Point::new(83, actual_top + 7))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
            .draw(draw_target).unwrap();
    }

    fn draw_index<D>(&mut self, draw_target: &mut D)
    where
        D: DrawTarget<Color = BinaryColor>, <D as embedded_graphics::draw_target::DrawTarget>::Error: core::fmt::Debug
    {
        let mut text: heapless::String<2> = heapless::String::new();
        let major: u32 = ((self.index + 1) / 10).try_into().unwrap();
        if major != 0 {
            text.push(crate::time::to_char(major)).unwrap();
        }
        text.push(crate::time::to_char(((self.index + 1) % 10).try_into().unwrap())).unwrap();

        Text::with_alignment(
            &text,
            Point::new(84, 5),
            MonoTextStyle::new(&FONT_4X6, BinaryColor::Off),
            Alignment::Right,
        )
        .draw(draw_target)
        .unwrap();
    }

    pub async fn process<D>(&mut self, rtc: &mut impl crate::Rtc, draw_target: &mut D, keypad: &mut impl crate::Keypad) -> Option<usize>
    where
        D: DrawTarget<Color = BinaryColor>, <D as embedded_graphics::draw_target::DrawTarget>::Error: core::fmt::Debug
    {
        draw_target.clear(BinaryColor::On);

        let bounding_box = draw_target.bounding_box();
        let top_left = bounding_box.top_left;
        let text = crate::time::write_time(rtc, false);

        Text::with_alignment(
            &text,
            Point::new(60, 5),
            MonoTextStyle::new(&FONT_4X6, BinaryColor::Off),
            Alignment::Left,
        )
        .draw(draw_target)
        .unwrap();

        let antenna = [
            0b00000111,
            0b01010111,
            0b10001111,
            0b11011111,
            0b11011111,
            0b11011111,
        ];
        let raw: ImageRawBE<BinaryColor> = ImageRaw::new(&antenna, 8);
        let image = Image::new(&raw, Point::new(0, 34));
        image.draw(draw_target).unwrap();

        let battery = [
            0b10011111,
            0b00001111,
            0b01101111,
            0b01101111,
            0b01101111,
            0b00001111,
        ];
        let raw: ImageRawBE<BinaryColor> = ImageRaw::new(&battery, 8);
        let image = Image::new(&raw, Point::new(80, 34));
        image.draw(draw_target).unwrap();

        if self.locked {
            Text::with_alignment(
                "Unlock",
                Point::new(42, 47),
                MonoTextStyle::new(&FONT_6X9, BinaryColor::Off),
                Alignment::Center,
            )
            .draw(draw_target)
            .unwrap();

            let key = [
                0b11111100,  0b01111111,
                0b11111001,  0b00111111,
                0b00000001,  0b00111111,
                0b00000001,  0b00111111,
                0b00111001,  0b00111111,
                0b11111100,  0b01111111,
            ];
            let raw: ImageRawBE<BinaryColor> = ImageRaw::new(&key, 16);
            let image = Image::new(&raw, Point::zero());
            image.draw(draw_target).unwrap();

            if keypad.event().await == crate::KeyEvent::Down(crate::Key::Select) {
                let event_future = async {
                    loop {
                        if keypad.event().await == crate::KeyEvent::Down(crate::Key::Asterisk) {
                            return;
                        }
                    }
                };
                let timeout_future = embassy_time::Timer::after_millis(1000);
                pin_mut!(event_future);
                pin_mut!(timeout_future);

                match future::select(timeout_future, event_future).await {
                    Either::Left((..)) => {
                        return None;
                    }
                    Either::Right((..)) => {
                        self.locked = false;
                        return None;
                    }
                }
            }
        } else {
            if self.menu_open {
                draw_target.clear(BinaryColor::On);
                Text::with_alignment(
                    self.items[self.index],
                    Point::new(42, 16),
                    MonoTextStyle::new(&FONT_6X13, BinaryColor::Off),
                    Alignment::Center,
                )
                .draw(draw_target)
                .unwrap();
                Text::with_alignment(
                    "Select",
                    Point::new(42, 47),
                    MonoTextStyle::new(&FONT_6X9, BinaryColor::Off),
                    Alignment::Center,
                )
                .draw(draw_target)
                .unwrap();
                self.draw_index(draw_target);
                self.draw_scrollbar(draw_target);

                match keypad.event().await {
                    crate::KeyEvent::Down(crate::Key::Cancel) => {
                        self.menu_open = false;
                    }
                    crate::KeyEvent::Down(crate::Key::Down) => {
                        self.index = (self.index + 1) % self.items.len()
                    }
                    crate::KeyEvent::Down(crate::Key::Up) => {
                        if self.index == 0 {
                            self.index = self.items.len() - 1;
                        } else {
                            self.index -= 1;
                        }
                    }
                    crate::KeyEvent::Down(crate::Key::Select) => {
                        return Some(self.index);
                    }
                    _ => {
                    }
                }
            } else {
                Text::with_alignment(
                    "Menu",
                    Point::new(42, 47),
                    MonoTextStyle::new(&FONT_6X9, BinaryColor::Off),
                    Alignment::Center,
                )
                .draw(draw_target)
                .unwrap();

                if keypad.event().await == crate::KeyEvent::Down(crate::Key::Select) {
                    self.menu_open = true;

                    let event_future = async {
                        loop {
                            if keypad.event().await == crate::KeyEvent::Down(crate::Key::Asterisk) {
                                return;
                            }
                        }
                    };
                    let timeout_future = embassy_time::Timer::after_millis(1000);
                    pin_mut!(event_future);
                    pin_mut!(timeout_future);

                    match future::select(timeout_future, event_future).await {
                        Either::Left((..)) => {
                            return None;
                        }
                        Either::Right((..)) => {
                            self.locked = true;
                            self.menu_open = false;
                            return None;
                        }
                    }
                }
            }
        }
        embassy_time::Timer::after_millis(10).await;
        None
    }
}
