use crate::Status;

#[derive(Clone, PartialEq)]
pub struct BuzzerTest<'a>(
    shared::console::Console<'a>,
    shared::confirmation::Confirmation,
);

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
    pub async fn run(&mut self, device: &mut impl shared::Device) -> Status {
        device.unmute();
        device.set_frequency(440);
        match self.1.run(device).await {
            None => Status::InProgress,
            Some(true) => {
                device.mute();
                Status::Passed
            }
            Some(false) => {
                device.mute();
                Status::Failed
            }
        }
    }
}
