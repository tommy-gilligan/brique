use core::fmt::Debug;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};

use crate::Status;

#[derive(Clone, PartialEq)]
pub struct VibrationTest<'a>(
    shared::console::Console<'a>,
    shared::confirmation::Confirmation,
);

impl VibrationTest<'_> {
    pub fn new() -> Self {
        Self(
            shared::console::Console::new(),
            shared::confirmation::Confirmation::new("Is the device vibrating?", "Yes", "No", false),
        )
    }
}

impl Default for VibrationTest<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl VibrationTest<'_> {
    pub async fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        keypad: &mut impl shared::Keypad,
        vibration: &mut impl shared::VibrationMotor,
        draw_target: &mut D,
    ) -> Status
    where
        <D as DrawTarget>::Error: Debug,
    {
        vibration.start();
        match self.1.run(keypad, draw_target).await {
            None => Status::InProgress,
            Some(true) => {
                vibration.stop();
                Status::Passed
            }
            Some(false) => {
                vibration.stop();
                Status::Failed
            }
        }
    }
}
