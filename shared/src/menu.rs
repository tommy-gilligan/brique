use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};

use super::Keypad;

pub struct Menu<'a> {
    items: &'a [&'a str],
    index: usize,
}

impl<'a> Menu<'a> {
    pub fn new(items: &'a [&'a str]) -> Self {
        Self { items, index: 0 }
    }

    fn draw(&mut self, draw_target: &mut impl crate::Device) {
        let bounding_box = draw_target.bounding_box();
        let top_left = bounding_box.top_left;

        let _ = bounding_box
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(draw_target);

        for (index, item) in self.items.iter().enumerate() {
            let y_offset: i32 = (index * 10).try_into().unwrap();
            if self.index == index {
                let _ = Rectangle::new(
                    top_left + Point::new(0, y_offset + 2),
                    Size::new(draw_target.bounding_box().size.width, 11),
                )
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
                .draw(draw_target);

                let _ = Text::with_alignment(
                    item,
                    top_left + Point::new(2, 10) + Point::new(0, y_offset),
                    MonoTextStyle::new(&FONT_6X10, BinaryColor::On),
                    Alignment::Left,
                )
                .draw(draw_target);
            } else {
                let _ = Text::with_alignment(
                    item,
                    top_left + Point::new(2, 10) + Point::new(0, y_offset),
                    MonoTextStyle::new(&FONT_6X10, BinaryColor::Off),
                    Alignment::Left,
                )
                .draw(draw_target);
            }
        }
    }

    fn down(&mut self) {
        self.index = (self.index + 1) % self.items.len()
    }

    fn up(&mut self) {
        if self.index == 0 {
            self.index = self.items.len() - 1;
        } else {
            self.index -= 1;
        }
    }

    pub async fn process(&mut self, device: &mut impl crate::Device) -> usize {
        loop {
            self.draw(device);
            match device.event().await {
                super::KeyEvent::Down(super::Key::Down) => {
                    self.down();
                }
                super::KeyEvent::Down(super::Key::Up) => {
                    self.up();
                }
                super::KeyEvent::Down(super::Key::Select) => {
                    return self.index;
                }
                _ => {}
            }
        }
    }
}
