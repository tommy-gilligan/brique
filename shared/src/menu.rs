use embedded_graphics::{
    draw_target::DrawTargetExt,
    mono_font::{
        MonoTextStyle,
        ascii::{FONT_6X9, FONT_6X10},
    },
    prelude::*,
    primitives::PrimitiveStyle,
    text::{Text, renderer::TextRenderer},
};
use embedded_graphics_core::{pixelcolor::BinaryColor, primitives::Rectangle};

use crate::held_key::HeldKey;

pub struct Menu<'a, T>
where
    T: AsRef<str>,
{
    items: &'a mut [T],
    index: usize,
    start_of_page_index: usize,
    bottom_visible_index: usize,
    page_size: usize,
    held_key: HeldKey,
    select_label: Option<&'a str>,
}

impl<'a, T> Menu<'a, T>
where
    T: AsRef<str>,
{
    pub fn new(items: &'a mut [T], select_label: Option<&'a str>) -> Self {
        assert!(!items.is_empty());
        Self {
            items,
            index: 0,
            start_of_page_index: 0,
            bottom_visible_index: 0,
            page_size: 0,
            held_key: HeldKey::new(750, 250),
            select_label,
        }
    }

    fn down(&mut self) {
        if self.index < self.items.len() - 1 {
            self.index += 1;
        }
        if self.index > self.bottom_visible_index {
            self.page_size = self.index - self.start_of_page_index;
            self.start_of_page_index = self.index;
        }
    }

    fn up(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
        if self.index < self.start_of_page_index {
            self.start_of_page_index = self.index.saturating_sub(self.page_size - 1);
        }
    }

    fn text_style(&self, selected: bool) -> MonoTextStyle<'_, BinaryColor> {
        embedded_graphics::mono_font::MonoTextStyleBuilder::new()
            .text_color(if selected {
                BinaryColor::On
            } else {
                BinaryColor::Off
            })
            .font(&FONT_6X10)
            .build()
    }
}

pub fn row_render(
    draw_target: &mut impl DrawTarget<Color = BinaryColor>,
    selected: bool,
    thing: &str,
    point: Point,
    text_style: MonoTextStyle<'_, BinaryColor>,
) -> Option<Point> {
    let text_bounding_box = text_style
        .measure_string(thing, point, embedded_graphics::text::Baseline::Top)
        .bounding_box;

    if draw_target
        .bounding_box()
        .contains(Point::new(0, text_bounding_box.bottom_right().unwrap().y))
    {
        let _ = Rectangle::new(
            point,
            Size::new(
                draw_target.bounding_box().size.width,
                text_bounding_box.size.height,
            ),
        )
        .into_styled(PrimitiveStyle::with_fill(if selected {
            BinaryColor::Off
        } else {
            BinaryColor::On
        }))
        .draw(draw_target);

        let _ = Text::with_baseline(
            thing,
            point,
            text_style,
            embedded_graphics::text::Baseline::Top,
        )
        .draw(draw_target);

        Some(Point::new(
            point.x,
            point.y + text_bounding_box.size.height as i32,
        ))
    } else {
        None
    }
}

impl<'a, T> embedded_graphics::Drawable for Menu<'a, T>
where
    T: AsRef<str>,
{
    type Color = BinaryColor;
    type Output = usize;

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let _ = target
            .bounding_box()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(target);

        let mut target = if let Some(select_label) = self.select_label {
            let text_bounding_box = embedded_graphics::mono_font::MonoTextStyleBuilder::new()
                .text_color(BinaryColor::Off)
                .font(&FONT_6X9)
                .build()
                .measure_string(
                    select_label,
                    Point::new(42, 47),
                    embedded_graphics::text::Baseline::Bottom,
                )
                .bounding_box;

            let text_style = embedded_graphics::text::TextStyleBuilder::new()
                .alignment(embedded_graphics::text::Alignment::Center)
                .baseline(embedded_graphics::text::Baseline::Bottom)
                .build();

            let _ = Text::with_text_style(
                select_label,
                Point::new(42, 47),
                embedded_graphics::mono_font::MonoTextStyleBuilder::new()
                    .text_color(BinaryColor::Off)
                    .font(&FONT_6X9)
                    .build(),
                text_style,
            )
            .draw(target);

            target.clipped(&Rectangle::new(
                Point::zero(),
                Size::new(
                    target.bounding_box().size.width,
                    target.bounding_box().size.height - text_bounding_box.size.height,
                ),
            ))
        } else {
            target.clipped(&Rectangle::new(Point::zero(), target.bounding_box().size))
        };

        let mut last_drawn_index = 0;
        let mut point = Point::zero();
        for (index, a) in self.items[self.start_of_page_index..].iter().enumerate() {
            if let Some(p) = row_render(
                &mut target.clipped(&target.bounding_box()),
                (self.index - self.start_of_page_index) == index,
                a.as_ref(),
                point,
                self.text_style((self.index - self.start_of_page_index) == index),
            ) {
                point = p;
                last_drawn_index = index;
            }
        }

        Ok(last_drawn_index + self.start_of_page_index)
    }
}

impl<'a, T> Menu<'a, T>
where
    T: AsRef<str>,
    Menu<'a, T>: embedded_graphics::Drawable<Color = BinaryColor, Output = usize>,
{
    pub async fn process(&mut self, device: &mut impl crate::Device) -> Option<&mut T> {
        loop {
            if let Ok(point) = self.draw(device) {
                self.bottom_visible_index = point;
                match self.held_key.event(device).await {
                    Some(
                        crate::held_key::Event::Down(crate::Key::Down)
                        | crate::held_key::Event::Delay(crate::Key::Down)
                        | crate::held_key::Event::Repeat(crate::Key::Down),
                    ) => {
                        self.down();
                    }
                    Some(
                        crate::held_key::Event::Down(crate::Key::Up)
                        | crate::held_key::Event::Delay(crate::Key::Up)
                        | crate::held_key::Event::Repeat(crate::Key::Up),
                    ) => {
                        self.up();
                    }
                    Some(crate::held_key::Event::Down(crate::Key::Cancel)) => {
                        return None;
                    }
                    Some(crate::held_key::Event::Down(crate::Key::Select)) => {
                        return Some(&mut self.items[self.index]);
                    }
                    _ => {}
                }
            }
        }
    }
}
