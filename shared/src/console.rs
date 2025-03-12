use core::fmt::Debug;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle,
};
use embedded_text::{
    TextBox,
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyle, TextBoxStyleBuilder},
};

#[derive(Clone, PartialEq)]
pub struct Console<'a>(Rectangle, MonoTextStyle<'a, BinaryColor>, TextBoxStyle);

impl Default for Console<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Console<'a> {
    pub fn new() -> Self {
        Self(
            Rectangle::new(Point::zero(), Size::new(84, 48)),
            MonoTextStyle::new(&FONT_6X10, BinaryColor::Off),
            TextBoxStyleBuilder::new()
                .height_mode(HeightMode::FitToText)
                .alignment(HorizontalAlignment::Left)
                .paragraph_spacing(6)
                .build(),
        )
    }

    pub fn draw<D: DrawTarget<Color = BinaryColor>>(&self, draw_target: &mut D, text: &'a str)
    where
        <D as DrawTarget>::Error: Debug,
    {
        draw_target.clear(BinaryColor::On).unwrap();
        TextBox::with_textbox_style(text, self.0, self.1, self.2)
            .draw(draw_target)
            .unwrap();
    }
}
