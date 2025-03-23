use embedded_graphics::{
    mono_font::{
        MonoTextStyle,
        ascii::{FONT_6X9, FONT_6X10},
    },
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};

pub struct Menu<'a> {
    items: &'a [&'a str],
    index: usize,
}

impl<'a> Menu<'a> {
    pub fn new(items: &'a [&'a str]) -> Self {
        Self { items, index: 0 }
    }

    fn draw(&mut self, draw_target: &mut impl crate::Device, text: &str) {
        draw_target.clear(BinaryColor::On).unwrap();
        let _ = Text::with_alignment(
            text,
            Point::new(42, 45),
            MonoTextStyle::new(&FONT_6X9, BinaryColor::Off),
            Alignment::Center,
        )
        .draw(draw_target);

        let mut clipped = draw_target.clipped(&Rectangle::new(Point::new(0, 0), Size::new(84, 40)));

        for (index, item) in self
            .items
            .iter()
            .skip(self.index & 0xfffffffc)
            .take(4)
            .enumerate()
        {
            let y_offset: i32 = (index * 10).try_into().unwrap();

            if self.index == index {
                let _ = Rectangle::new(Point::new(0, y_offset), Size::new(84, 10))
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
                    .draw(&mut clipped);

                let _ = Text::with_alignment(
                    item,
                    Point::new(2, 7 + y_offset),
                    MonoTextStyle::new(&FONT_6X10, BinaryColor::On),
                    Alignment::Left,
                )
                .draw(&mut clipped);
            } else {
                let _ = Rectangle::new(Point::new(0, y_offset), Size::new(84, 10))
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(&mut clipped);
                let _ = Text::with_alignment(
                    item,
                    Point::new(2, 7 + y_offset),
                    MonoTextStyle::new(&FONT_6X10, BinaryColor::Off),
                    Alignment::Left,
                )
                .draw(&mut clipped);
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

    pub async fn process(&mut self, device: &mut impl crate::Device, text: &str) -> Option<usize> {
        self.draw(device, text);
        match device.event().await {
            super::KeyEvent::Down(super::Key::Down) => {
                self.down();
            }
            super::KeyEvent::Down(super::Key::Up) => {
                self.up();
            }
            super::KeyEvent::Down(super::Key::Select) => {
                return Some(self.index);
            }
            _ => {}
        }
        None
    }
}
