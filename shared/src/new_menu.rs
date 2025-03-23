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
use embedded_graphics::Drawable;

pub struct NewMenu<'a, D> where D: Drawable<Color = BinaryColor> {
    items: &'a [&'a D],
    index: usize,
}

impl<'a, D> NewMenu<'a, D> where D: Drawable<Color = BinaryColor> {
    pub fn new(items: &'a [&'a D]) -> Self {
        Self { items, index: 0 }
    }

    fn draw(&mut self, draw_target: &mut impl crate::Device, text: &str) {
        draw_target.clear(BinaryColor::On).unwrap();
        let mut clipped = draw_target.clipped(&Rectangle::new(Point::new(0, 0), Size::new(84, 40)));
        for (index, item) in self.items.into_iter().enumerate() {
            let mut translated = clipped.translated(Point::new(0, 10 * (index as i32)));
            item.draw(&mut translated);
        }
    }

    fn down(&mut self) {
    }

    fn up(&mut self) {
    }

    pub async fn process(&mut self, device: &mut impl crate::Device, text: &str) -> Option<usize> {
        None
    }
}
