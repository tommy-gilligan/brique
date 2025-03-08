use core::fmt::Debug;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};

use crate::Status;

#[derive(Clone, PartialEq)]
pub struct BuzzerTest<'a>(shared::console::Console<'a>, shared::confirmation::Confirmation);

impl BuzzerTest<'_> {
    pub fn new() -> Self {
        Self(
            shared::console::Console::new(),
            shared::confirmation::Confirmation::new(
                "Is the device making a tone?",
                "Yes",
                "No",
                false,
            ),
        )
    }
}

impl Default for BuzzerTest<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl BuzzerTest<'_> {
    pub async fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        keypad: &mut impl shared::Keypad,
        buzzer: &mut impl shared::Buzzer,
        draw_target: &mut D,
    ) -> Status
    where
        <D as DrawTarget>::Error: Debug,
    {
        buzzer.unmute();
        buzzer.set_frequency(440);
        match self.1.run(keypad, draw_target).await {
            None => Status::InProgress,
            Some(true) => {
                buzzer.mute();
                Status::Passed
            }
            Some(false) => {
                buzzer.mute();
                Status::Failed
            }
        }
    }
}
