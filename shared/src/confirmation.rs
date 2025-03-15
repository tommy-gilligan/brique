use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Primitive, Size},
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};
use embedded_text::{
    TextBox,
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
};

use crate::KeyEvent;

#[derive(Clone, PartialEq)]
pub struct Confirmation(&'static str, &'static str, &'static str, bool, Option<bool>);

impl Confirmation {
    pub fn new(
        message: &'static str,
        r#true: &'static str,
        r#false: &'static str,
        selected: bool,
    ) -> Self {
        Self(message, r#true, r#false, selected, None)
    }

    pub async fn run(&mut self, device: &mut impl crate::Device) -> Option<bool> {
        if let Some(result) = self.4 {
            return Some(result);
        }
        let _ = device.clear(BinaryColor::On);

        let fill = PrimitiveStyle::with_fill(BinaryColor::Off);
        if self.3 {
            Rectangle::new(Point::new(0, 36), Size::new(42, 12))
                .into_styled(fill)
                .draw(device)
                .unwrap();
        } else {
            Rectangle::new(Point::new(42, 36), Size::new(42, 12))
                .into_styled(fill)
                .draw(device)
                .unwrap();
        }

        let true_style = if self.3 {
            MonoTextStyle::new(&FONT_6X10, BinaryColor::On)
        } else {
            MonoTextStyle::new(&FONT_6X10, BinaryColor::Off)
        };
        Text::with_alignment(self.1, Point::new(21, 44), true_style, Alignment::Center)
            .draw(device)
            .unwrap();
        let false_style = if self.3 {
            MonoTextStyle::new(&FONT_6X10, BinaryColor::Off)
        } else {
            MonoTextStyle::new(&FONT_6X10, BinaryColor::On)
        };
        Text::with_alignment(self.2, Point::new(63, 44), false_style, Alignment::Center)
            .draw(device)
            .unwrap();

        TextBox::with_textbox_style(
            self.0,
            Rectangle::new(Point::zero(), Size::new(84, 48)),
            MonoTextStyle::new(&FONT_6X10, BinaryColor::Off),
            TextBoxStyleBuilder::new()
                .height_mode(HeightMode::FitToText)
                .alignment(HorizontalAlignment::Left)
                .paragraph_spacing(6)
                .build(),
        )
        .draw(device)
        .unwrap();

        match device.event().await {
            KeyEvent::Down(crate::Key::Up | crate::Key::Down) => {
                self.3 = !self.3;
                None
            }
            KeyEvent::Down(crate::Key::Select) => {
                self.4 = Some(self.3);
                self.4
            },
            _ => None,
        }
    }
}
