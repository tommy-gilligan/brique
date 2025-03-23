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
    pub async fn run(
        &mut self,
        device: &mut impl shared::Device,
        _system_response: Option<[u8; 64]>,
    ) -> Status {
        if let Err(_e) = device.unmute_buzzer() {
            return Status::Failed;
        }
        if let Err(_e) = device.set_frequency(440) {
            return Status::Failed;
        }
        match self.1.run(device).await {
            None => Status::InProgress(None),
            Some(true) => {
                if let Err(_e) = device.mute_buzzer() {
                    return Status::Failed;
                }
                Status::Passed
            }
            Some(false) => {
                if let Err(_e) = device.mute_buzzer() {
                    return Status::Failed;
                }
                Status::Failed
            }
        }
    }
}
