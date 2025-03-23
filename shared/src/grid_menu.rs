use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};
use embedded_graphics::mono_font::ascii::FONT_6X9;

use crate::held_key::HeldKey;

const ROW_LENGTH: usize = 8;

pub struct GridMenu<'a> {
    items: &'a [&'a str],
    row_index: usize,
    column_index: usize,
    held_key: HeldKey,
}

impl<'a> GridMenu<'a> {
    pub fn new(items: &'a [&'a str]) -> Self {
        Self {
            items,
            column_index: 0,
            row_index: 0,
            held_key: HeldKey::new(1500, 500),
        }
    }

    pub fn draw<D>(&mut self, draw_target: &mut D, text: &str)
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let top_left = draw_target.bounding_box().top_left;
        let _ = draw_target.clear(BinaryColor::On);
        let _ = Text::with_alignment(
            text,
            Point::new(42, 45),
            MonoTextStyle::new(&FONT_6X9, BinaryColor::Off),
            Alignment::Center,
        )
        .draw(draw_target);

        let mut clipped = draw_target.clipped(&Rectangle::new(Point::new(0, 0), Size::new(84, 40)));

        for (row_index, row) in self.items.chunks(ROW_LENGTH).enumerate() {
            for (column_index, cell) in row.iter().enumerate() {
                let y_offset: i32 = (row_index * 10).try_into().unwrap();
                let x_offset: i32 = (column_index * 10).try_into().unwrap();

                if self.row_index == row_index && self.column_index == column_index {
                    let _ = Rectangle::new(
                        top_left + Point::new(x_offset, y_offset) + Point::new(2, 3),
                        Size::new(9, 10),
                    )
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
                    .draw(&mut clipped);
                    let _ = Text::with_alignment(
                        cell,
                        top_left + Point::new(6, 10) + Point::new(x_offset, y_offset),
                        MonoTextStyle::new(&FONT_6X10, BinaryColor::On),
                        Alignment::Center,
                    )
                    .draw(&mut clipped);
                } else {
                    let _ = Text::with_alignment(
                        cell,
                        top_left + Point::new(6, 10) + Point::new(x_offset, y_offset),
                        MonoTextStyle::new(&FONT_6X10, BinaryColor::Off),
                        Alignment::Center,
                    )
                    .draw(&mut clipped);
                }
            }
        }
    }

    fn down(&mut self) {
        if (self.column_index + self.row_index * ROW_LENGTH) == self.items.len() - 1 {
            return;
        }
        if self.column_index == ROW_LENGTH - 1 {
            self.row_index += 1;
            self.column_index = 0;
        } else {
            self.column_index += 1;
        }
    }

    fn up(&mut self) {
        if self.column_index == 0 && self.row_index == 0 {
            return;
        }
        if self.column_index == 0 {
            self.row_index -= 1;
            self.column_index = ROW_LENGTH - 1;
        } else {
            self.column_index -= 1;
        }
    }

    pub async fn run(&mut self, device: &mut impl crate::Device, text: &str) -> &'a str {
        loop {
            self.draw(device, text);

            match self.held_key.event(device).await {
                Some(
                    super::held_key::Event::Down(super::Key::Down)
                    | super::held_key::Event::Delay(super::Key::Down)
                    | super::held_key::Event::Repeat(super::Key::Down),
                ) => {
                    self.down();
                }
                Some(
                    super::held_key::Event::Down(super::Key::Up)
                    | super::held_key::Event::Delay(super::Key::Up)
                    | super::held_key::Event::Repeat(super::Key::Up),
                ) => {
                    self.up();
                }
                Some(super::held_key::Event::Down(super::Key::Select)) => {
                    return self.items[self.row_index * ROW_LENGTH + self.column_index];
                }
                _ => {}
            }
        }
    }
}
