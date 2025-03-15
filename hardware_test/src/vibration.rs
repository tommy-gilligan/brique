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
    pub async fn run(
        &mut self,
        device: &mut impl shared::Device,
        _system_response: Option<[u8; 64]>,
    ) -> Status {
        device.start();
        match self.1.run(device).await {
            None => Status::InProgress(None),
            Some(true) => {
                device.stop();
                Status::Passed
            }
            Some(false) => {
                device.stop();
                Status::Failed
            }
        }
    }
}
