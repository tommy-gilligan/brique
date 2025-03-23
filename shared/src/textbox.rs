use core::{ascii::Char, fmt::Debug};

use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRaw, ImageRawBE},
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::renderer::TextRenderer,
};

use crate::multitap::Case;

pub struct Textbox<'a> {
    cursor: Point,
    multitap: crate::multitap::MultiTap,
    buffer: &'a mut [u8],
    index: usize,
    first_draw: bool,
    grid_menu: Option<crate::grid_menu::GridMenu<'a>>
}

impl<'a> Textbox<'a> {
    pub fn new<D: DrawTarget<Color = BinaryColor>>(
        _draw_target: &mut D,
        buffer: &'a mut [u8],
    ) -> Self
    where
        <D as DrawTarget>::Error: Debug,
    {
        Self {
            cursor: Point::new(0, 9),
            multitap: crate::multitap::MultiTap::new(1500),
            buffer,
            index: 0,
            first_draw: true,
            grid_menu: None
        }
    }

    // should return emitted characters
    pub async fn process(&mut self, device: &mut impl crate::Device) -> Option<Char> {
        if self.first_draw {
            self.first_draw = false;
            device.clear(BinaryColor::On).unwrap();
            self.draw_border(device);
        }

        if let Some(grid_menu) = &mut self.grid_menu {
            let c = grid_menu.run(device, "Use").await.chars().next().unwrap();
            log::info!("{}", c);
            self.grid_menu = None;
            self.push(device, c);
            self.first_draw = true;
        } else {
            match self.multitap.event(device).await {

                Some(crate::multitap::Event::Tentative(c)) => {
                    if c == Char::Backspace {
                        self.backspace(device);
                        return Some(c);
                    } else {
                        self.push_tentative(device, c.into());
                    }
                }
                Some(crate::multitap::Event::Decided(c)) => {
                    if c != Char::Backspace {
                        self.push(device, c.into());
                        return Some(c);
                    } else {
                        self.backspace(device);
                        return Some(c);
                    }
                }
                Some(crate::multitap::Event::Case(c)) => {
                    self.draw_case_icon(device, c);
                }
                Some(crate::multitap::Event::ShowSpecialCharacters) => {
                    self.grid_menu = Some(crate::grid_menu::GridMenu::new(&[
                        "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?", "@",    "[", "\\", "^", "_", "`",    "{", "|", "}", "~",
                    ]));
                }
                None => {}

            }
        }
        None
    }

    fn draw_border<D: DrawTarget<Color = BinaryColor>>(&mut self, draw_target: &mut D)
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
            0b11111110,
            0b1100_1111,
            0b11111111,
            0b0000_1111,
        ];
        let raw: ImageRawBE<BinaryColor> = ImageRaw::new(&pencil, 16);
        let image = Image::new(&raw, Point::zero());
        image.draw(draw_target).unwrap();
    }

    fn draw_case_icon<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        draw_target: &mut D,
        c: crate::multitap::Case,
    ) where
        <D as DrawTarget>::Error: Debug,
    {
        let icon = match c {
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

    fn push<D: DrawTarget<Color = BinaryColor>>(&mut self, draw_target: &mut D, character: char)
    where
        <D as DrawTarget>::Error: Debug,
    {
        self.buffer[self.index] = character as u8;
        self.index += 1;

        let renderer = MonoTextStyle::new(&FONT_6X9, BinaryColor::Off);
        let mut b = [0; 4];

        Rectangle::new(self.cursor, Size::new(6, 9))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(draw_target)
            .unwrap();

        if let Ok(g) = renderer.draw_string(
            character.encode_utf8(&mut b),
            self.cursor,
            embedded_graphics::text::Baseline::Top,
            draw_target,
        ) {
            if g.x >= 80 {
                self.cursor = Point::new(0, g.y + 9);
            } else {
                self.cursor = g;
            }
        }
    }

    fn backspace<D: DrawTarget<Color = BinaryColor>>(&mut self, draw_target: &mut D)
    where
        <D as DrawTarget>::Error: Debug,
    {
        if self.index > 0 {
            self.index -= 1;
            self.buffer[self.index] = 0;
        }

        self.cursor -= Point::new(6, 0);
        if self.cursor.x < 0 {
            self.cursor.y -= 9;
            self.cursor.x = 78;
        }
        if self.cursor.x <= 0 && self.cursor.y <= 9 {
            self.cursor.x = 0;
            self.cursor.y = 9;
        }

        Rectangle::new(self.cursor, Size::new(6, 9))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(draw_target)
            .unwrap();
    }

    fn push_tentative<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        draw_target: &mut D,
        character: char,
    ) where
        <D as DrawTarget>::Error: Debug,
    {
        let renderer = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
        let mut b = [0; 4];

        Rectangle::new(self.cursor, Size::new(6, 9))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(draw_target)
            .unwrap();

        let _ = renderer.draw_string(
            character.encode_utf8(&mut b),
            self.cursor,
            embedded_graphics::text::Baseline::Top,
            draw_target,
        );
    }
}
