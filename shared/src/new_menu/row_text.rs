use embedded_graphics::mono_font::{
    MonoTextStyle,
    ascii::{FONT_4X6, FONT_6X9, FONT_10X20},
};
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::text::Text;
use embedded_graphics_core::{
    Drawable,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle
};
use embedded_graphics::draw_target::DrawTargetExt;
use embedded_graphics::prelude::Primitive;
use embedded_layout::View;
use crate::new_menu::SelectableView;
use embedded_graphics::mono_font::MonoFont;

#[derive(Clone)]
pub struct RowText<'a, T> where T: core::convert::AsRef<str> {
    bounds: Rectangle,
    pub item: T,
    inverted: bool,
    font: &'a MonoFont<'a>
}

impl <'a, T>View for RowText<'a, T> where T: core::convert::AsRef<str> {
    #[inline]
    fn translate_impl(&mut self, by: Point) {
        embedded_graphics::prelude::Transform::translate_mut(&mut self.bounds, by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}

impl <'a, T>Drawable for RowText<'a, T> where T: core::convert::AsRef<str> {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D) -> Result<(), D::Error> {
        display.bounding_box().into_styled(PrimitiveStyle::with_fill(
            if self.inverted {
                BinaryColor::Off
            } else {
                BinaryColor::On
            }
        )).draw(&mut display.clipped(&self.bounds()));

        Text::new(
            self.item.as_ref(),
            self.bounds().top_left + Point::new(0, (self.bounds().size.height * 3 / 4).try_into().unwrap()),
            MonoTextStyle::new(
                self.font,
                if self.inverted {
                    BinaryColor::On
                } else {
                    BinaryColor::Off
                }
            )
        ).draw(&mut display.clipped(&self.bounds()));

        Ok(())
    }
}

impl <'a, T>RowText<'a, T> where T: core::convert::AsRef<str> {
    pub fn new<D: DrawTarget<Color = BinaryColor>>(
        item: T,
        display: &mut D,
        font: &'a MonoFont<'a>
    ) -> Self {

        Self {
            bounds: Rectangle::new(
                Point::zero(),
                Size::new(
                    display.bounding_box().size.width,
                    font.character_size.height
                )
            ),
            item,
            inverted: false,
            font
        }
    }

    pub fn borrow_mut(&mut self) -> &mut T {
        &mut self.item
    }
}

impl <'a, T>SelectableView for RowText<'a, T> where T: core::convert::AsRef<str> {
    fn deselect(&mut self) {
        self.inverted = false;
    }

    fn select(&mut self) {
        self.inverted = true;
    }

    fn is_selected(&self) -> bool {
        self.inverted
    }
}
