use crate::Status;

#[derive(Clone, PartialEq)]
pub struct CdcTest<'a>(
    shared::console::Console<'a>,
    shared::confirmation::Confirmation,
    bool,
    bool,
);

impl CdcTest<'_> {
    pub fn new() -> Self {
        Self(
            shared::console::Console::new(),
            shared::confirmation::Confirmation::new(
                "Perform optional test of USB CDC?",
                "Yes",
                "No",
                false,
            ),
            false,
            false,
        )
    }
}

impl Default for CdcTest<'_> {
    fn default() -> Self {
        Self::new()
    }
}

const SECRET: &str = "abxvn";
const CDC_PROMPT: &str = "Enter secret: abxvn\n";

impl CdcTest<'_> {
    pub async fn run(
        &mut self,
        device: &mut impl shared::Device,
        system_response: Option<[u8; 64]>,
    ) -> Status {
        match (self.1.run(device).await, self.2, self.3) {
            (None, ..) => Status::InProgress(None),
            (Some(true), false, _) => {
                let mut buf = [0; 64];
                for (a, b) in buf.iter_mut().zip(CDC_PROMPT.as_bytes()) {
                    *a = *b;
                }
                self.2 = true;
                self.0.draw(device, "Waiting for secret from host.\n");

                Status::InProgress(Some(shared::SystemRequest::UsbTx(
                    shared::UsbTx::CdcBuffer(buf),
                )))
            }
            (Some(true), true, false) => {
                embassy_time::Timer::after_millis(100).await;
                if let Some(buf) = system_response {
                    let c_str = core::ffi::CStr::from_bytes_until_nul(&buf)
                        .unwrap()
                        .to_str()
                        .unwrap();
                    self.0.draw(device, c_str);

                    if c_str == SECRET {
                        self.3 = true;
                    }
                }
                Status::InProgress(None)
            }
            (Some(true), true, true) => Status::Passed,
            (Some(false), ..) => Status::Passed,
        }
    }
}
