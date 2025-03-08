use core::fmt::Debug;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};

use crate::Status;

#[derive(Clone, PartialEq)]
pub struct BacklightTest<'a>(shared::console::Console<'a>, shared::confirmation::Confirmation);

impl BacklightTest<'_> {
    pub fn new() -> Self {
        Self(
            shared::console::Console::new(),
            shared::confirmation::Confirmation::new("Is the backlight on?", "Yes", "No", false),
        )
    }
}

impl Default for BacklightTest<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl BacklightTest<'_> {
    pub async fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        keypad: &mut impl shared::Keypad,
        backlight: &mut impl shared::Backlight,
        draw_target: &mut D,
    ) -> Status
    where
        <D as DrawTarget>::Error: Debug,
    {
        backlight.on();
        match self.1.run(keypad, draw_target).await {
            None => {
                return Status::InProgress;
            }
            Some(true) => {
                backlight.off();
                return Status::Passed;
            }
            Some(false) => {
                backlight.off();
                return Status::Failed;
            }
        }

        Status::InProgress
    }
}
