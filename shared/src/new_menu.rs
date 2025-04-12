use embedded_graphics::{
    Drawable,
    pixelcolor::BinaryColor,
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use embedded_layout::{layout::linear::LinearLayout, prelude::*};
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_layout::view_group::ViewGroup;
use crate::new_menu::row_text::RowText;
use crate::KeyEvent;
use crate::Key;

trait SelectableView: View {
    fn is_selected(&self) -> bool;
    fn select(&mut self);
    fn deselect(&mut self);
}
pub mod row_text;

pub struct MenuItems<'a, T> where T: core::convert::AsRef<str> {
  bounds: Rectangle,
  menu_items: core::cell::RefCell<&'a mut [RowText<'a, T>]>,
  index: usize,
}

impl <'a, T>View for MenuItems<'a, T> where T: core::convert::AsRef<str> {
    #[inline]
    fn translate_impl(&mut self, by: Point) {
        embedded_graphics::prelude::Transform::translate_mut(&mut self.bounds, by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}

impl <'a, T>Drawable for MenuItems<'a, T> where T: core::convert::AsRef<str> {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D) -> Result<(), D::Error> {
        let mut binding = self.menu_items.borrow_mut();

        let views = Views::new(&mut (*binding)[0..]);
        let view_group = LinearLayout::vertical(views).with_alignment(horizontal::Left).arrange();
        view_group.draw(&mut display.clipped(&self.bounds()));

        // rect.draw(&mut (device.color_converted::<Inverted>()).color_converted());

        Ok(())
    }
}

impl <'a, T>MenuItems<'a, T> where T: core::convert::AsRef<str> {
    fn first_invisible_index<D: DrawTarget<Color = BinaryColor>>(
        &self,
        display: &mut D,
        menu_items: &mut [RowText<'a, T>]
    ) -> usize {
        let len = menu_items.len();
        let views = Views::new(menu_items);
        let view_group = LinearLayout::vertical(views).with_alignment(horizontal::Left).arrange();
        (0..len).find(|i|
            !display.bounding_box().contains(view_group.bounds_of(*i).bottom_right().unwrap())
        ).unwrap()
    }

    pub fn new(menu_items: &'a mut [RowText<'a, T>]) -> Self {
        Self {
            bounds: Rectangle::new(
                Point::new(0, 0),
                Size::new(84, 40)
            ),
            menu_items: core::cell::RefCell::new(menu_items),
            index: 0,
        }
    }

    fn down<D: DrawTarget<Color = BinaryColor>>(&mut self, display: &mut D) {
        let mut binding = self.menu_items.borrow_mut();

        for menu_item in (*binding).iter_mut() {
            menu_item.deselect();
        }
        if self.index < (*binding).len() - 1 {
            self.index = self.index + 1;
        }

        (*binding)[self.index].select();
    }

    fn up<D: DrawTarget<Color = BinaryColor>>(&mut self, display: &mut D) {
        let mut binding = self.menu_items.borrow_mut();

        for menu_item in (*binding).iter_mut() {
            menu_item.deselect();
        }

        if self.index > 0 {
            self.index = self.index - 1;
        }

        (*binding)[self.index].select();
    }

    fn selection(&mut self) -> usize {
        let mut binding = self.menu_items.borrow_mut();
        (*binding).iter_mut().enumerate().find(|(i, e)| e.is_selected()).unwrap().0
    }
}

pub struct NewMenu<'a, T> where T: core::convert::AsRef<str> {
    menu_items: MenuItems<'a, T>
}

impl <'a, T>NewMenu<'a, T> where T: core::convert::AsRef<str> {
    pub fn new(menu_items: &'a mut [RowText<'a, T>]) -> Self {
        Self {
            menu_items: MenuItems::new(menu_items)
        }
    }

    pub async fn run(
        &mut self,
        device: &mut impl crate::Device,
        select_label: &str
    ) -> usize {
        device.bounding_box().into_styled(PrimitiveStyle::with_fill(BinaryColor::On)).draw(device);
        let mut text = Text::with_alignment(
            select_label,
            Point::zero(),
            MonoTextStyle::new(&FONT_6X9, BinaryColor::Off),
            embedded_graphics::text::Alignment::Center
        );

        loop {
            self.menu_items.draw(
                &mut device.clipped(
                    &Rectangle::new(
                        Point::zero(),
                        Size::new(
                            device.bounding_box().size.width,
                            device.bounding_box().size.height - text.bounds().size.height
                        )
                    )
                )
            );

            embedded_graphics::prelude::Transform::translate(&text, Point::new(
                device.bounding_box().center().x,
                (<u32 as TryInto<i32>>::try_into(device.bounding_box().size.height).unwrap() - 3i32)
            )).draw(device);

            match device.event().await {
                KeyEvent::Down(Key::Down) => {
                    self.menu_items.down(device)
                },
                KeyEvent::Down(Key::Up) => {
                    self.menu_items.up(device)
                },
                KeyEvent::Down(Key::Select) => {
                    return self.menu_items.selection();
                },
                _ => {
                },
            }
        }

        // menu_items: core::cell::RefCell<&'a mut [RowText<'a, T>]>,
    }
}
