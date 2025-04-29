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
    pub async fn run(&mut self, device: &mut impl shared::Device) -> Status {
        device.start_vibrating();
        match self.1.run(device).await {
            None => Status::InProgress,
            Some(true) => {
                device.stop_vibrating();
                Status::Passed
            }
            Some(false) => {
                device.stop_vibrating();
                Status::Failed
            }
        }
    }
}
