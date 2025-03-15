use crate::Status;

#[derive(Clone, PartialEq)]
pub struct HidTest<'a>(
    shared::console::Console<'a>,
    shared::confirmation::Confirmation,
    bool,
    bool,
    HidPrinter<'a>,
);

impl HidTest<'_> {
    pub fn new() -> Self {
        Self(
            shared::console::Console::new(),
            shared::confirmation::Confirmation::new(
                "Perform optional test of USB HID?",
                "Yes",
                "No",
                false,
            ),
            false,
            false,
            HidPrinter::new(SECRET),
        )
    }
}

#[derive(Clone)]
struct HidPrinter<'a>(core::str::Chars<'a>, bool);

impl<'a> HidPrinter<'a> {
    fn new(s: &'a str) -> Self {
        Self(s.chars(), false)
    }
}

impl<'a> PartialEq for HidPrinter<'a> {
    fn eq(&self, _: &HidPrinter<'a>) -> bool {
        true
    }
}

impl Iterator for HidPrinter<'_> {
    type Item = shared::SystemRequest;

    fn next(&mut self) -> Option<shared::SystemRequest> {
        if !self.1 {
            match self.0.next() {
                Some(c) => {
                    self.1 = true;
                    Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar(
                        shared::build_report(c.as_ascii().unwrap()),
                    )))
                }
                None => None,
            }
        } else {
            self.1 = false;
            let key_up = usbd_hid::descriptor::KeyboardReport {
                keycodes: [0, 0, 0, 0, 0, 0],
                leds: 0,
                modifier: 0,
                reserved: 0,
            };
            Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar(key_up)))
        }
    }
}

impl Default for HidTest<'_> {
    fn default() -> Self {
        Self::new()
    }
}

const SECRET: &str = "oevdhr";

impl HidTest<'_> {
    pub async fn run(
        &mut self,
        device: &mut impl shared::Device,
        _system_response: Option<[u8; 64]>,
    ) -> Status {
        match (self.1.run(device).await, self.2, self.3) {
            (None, ..) => Status::InProgress(None),
            (Some(true), false, _) => {
                self.0.draw(device, "Printing.\n");
                match self.4.next() {
                    None => {
                        self.2 = true;
                        Status::InProgress(None)
                    }
                    Some(request) => Status::InProgress(Some(request)),
                }
            }
            (Some(true), true, false) => {
                self.0.draw(device, "Printed.\n");
                embassy_time::Timer::after_millis(100).await;
                // if let Some(buf) = system_response {
                //     let c_str = core::ffi::CStr::from_bytes_until_nul(&buf).unwrap().to_str().unwrap();
                //     self.0.draw(device, c_str);

                //     if c_str == SECRET {
                //         self.3 = true;
                //     }
                // }
                Status::InProgress(None)
            }
            (Some(true), true, true) => Status::Passed,
            (Some(false), ..) => Status::Passed,
        }
    }
}
