use core::fmt::Debug;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};
use enum_iterator::{first, last, next};
use shared::{Key, KeyEvent};

use crate::Status;

#[derive(Clone, PartialEq)]
pub struct KeypadTest<'a>(Key, shared::console::Console<'a>);

impl KeypadTest<'_> {
    pub fn new(key: Key) -> Self {
        Self(key, shared::console::Console::new())
    }
}

impl Default for KeypadTest<'_> {
    fn default() -> Self {
        Self::new(first().unwrap())
    }
}

impl KeypadTest<'_> {
    pub async fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        keypad: &mut impl shared::Keypad,
        draw_target: &mut D,
    ) -> Status
    where
        <D as DrawTarget>::Error: Debug,
    {
        self.1.draw(draw_target, self.0.clone().into());
        if let KeyEvent::Down(key) = keypad.event().await
            && key == self.0
        {
            if self.0 == last::<Key>().unwrap() {
                return Status::Passed;
            } else {
                self.0 = next(&self.0).unwrap();
            }
        } else {
            return Status::Failed;
        }

        Status::InProgress
    }
}
