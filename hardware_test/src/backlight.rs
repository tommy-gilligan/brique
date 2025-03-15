use crate::Status;

#[derive(Clone, PartialEq)]
pub struct BacklightTest<'a>(
    shared::console::Console<'a>,
    shared::confirmation::Confirmation,
);

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
    pub async fn run(&mut self, device: &mut impl shared::Device, system_response: Option<[u8; 64]>) -> Status {
        device.on();
        match self.1.run(device).await {
            None => Status::InProgress(None),
            Some(true) => {
                device.off();
                Status::Passed
            }
            Some(false) => {
                device.off();
                Status::Failed
            }
        }
    }
}
