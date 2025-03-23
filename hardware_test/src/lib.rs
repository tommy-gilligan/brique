#![feature(let_chains)]
#![feature(ascii_char)]
#![no_std]

use enum_iterator::Sequence;
use shared::Application;

mod keypad;
use keypad::*;
mod vibration;
use vibration::*;
mod buzzer;
use buzzer::*;
mod backlight;
use backlight::*;
mod cdc;
use cdc::*;
mod hid;
use hid::*;

#[derive(Clone, PartialEq)]
pub enum Status {
    Passed,
    Failed,
    InProgress(Option<shared::SystemRequest>),
}

#[derive(Clone, Sequence, PartialEq)]
enum Test<'a> {
    Keypad(KeypadTest<'a>),
    Vibration(VibrationTest<'a>),
    Buzzer(BuzzerTest<'a>),
    Backlight(BacklightTest<'a>),
    Cdc(CdcTest<'a>),
    Hid(HidTest<'a>),
}

pub struct HardwareTest<'a>(Status, shared::console::Console<'a>, Test<'a>);

impl HardwareTest<'_> {
    pub fn new(test: Status) -> Self {
        Self(
            test,
            shared::console::Console::new(),
            Test::Keypad(Default::default()),
        )
    }

    pub fn next(&mut self) {
        match self.2 {
            Test::Keypad(_) => {
                self.2 = Test::Vibration(Default::default());
            }
            Test::Vibration(_) => {
                self.2 = Test::Buzzer(Default::default());
            }
            Test::Buzzer(_) => {
                self.2 = Test::Backlight(Default::default());
            }
            Test::Backlight(_) => {
                self.2 = Test::Cdc(Default::default());
            }
            Test::Cdc(_) => {
                self.2 = Test::Hid(Default::default());
            }
            Test::Hid(_) => {
                self.2 = Test::Keypad(Default::default());
            }
        }
    }
}

impl Default for HardwareTest<'_> {
    fn default() -> Self {
        Self::new(Status::InProgress(None))
    }
}

impl Application for HardwareTest<'_> {
    async fn run(
        &mut self,
        device: &mut impl shared::Device,
        system_response: Option<[u8; 64]>,
    ) -> Result<Option<shared::SystemRequest>, ()> {
        match self.0.clone() {
            Status::InProgress(_) => match self.2 {
                Test::Keypad(ref mut test) => match test.run(device, system_response).await {
                    Status::Passed => {
                        log::info!("Passed keypad");
                        self.next();
                        Ok(None)
                    }
                    Status::Failed => {
                        self.0 = Status::Failed;
                        Ok(None)
                    }
                    _ => Ok(None),
                },
                Test::Vibration(ref mut test) => match test.run(device, system_response).await {
                    Status::Passed => {
                        log::info!("Passed vibration");
                        self.next();
                        Ok(None)
                    }
                    Status::Failed => {
                        self.0 = Status::Failed;
                        Ok(None)
                    }
                    _ => Ok(None),
                },
                Test::Buzzer(ref mut test) => match test.run(device, system_response).await {
                    Status::Passed => {
                        log::info!("Passed buzzer");
                        self.next();
                        Ok(None)
                    }
                    Status::Failed => {
                        self.0 = Status::Failed;
                        Ok(None)
                    }
                    _ => Ok(None),
                },
                Test::Backlight(ref mut test) => match test.run(device, system_response).await {
                    Status::Passed => {
                        log::info!("Passed backlight");
                        self.next();
                        Ok(None)
                    }
                    Status::Failed => {
                        self.0 = Status::Failed;
                        Ok(None)
                    }
                    _ => Ok(None),
                },
                Test::Cdc(ref mut test) => match test.run(device, system_response).await {
                    Status::Passed => {
                        log::info!("Passed CDC");
                        self.next();
                        Ok(None)
                    }
                    Status::Failed => {
                        self.0 = Status::Failed;
                        Ok(None)
                    }
                    Status::InProgress(system_request) => Ok(system_request),
                },
                Test::Hid(ref mut test) => match test.run(device, system_response).await {
                    Status::Passed => {
                        log::info!("Passed HID");
                        self.0 = Status::Passed;
                        Ok(None)
                    }
                    Status::Failed => {
                        self.0 = Status::Failed;
                        Ok(None)
                    }
                    Status::InProgress(system_request) => Ok(system_request),
                },
            },
            Status::Passed => {
                log::info!("Passed all tests");
                self.1.draw(device, "Passed");
                embassy_time::Timer::after_millis(10).await;
                Ok(None)
            }
            Status::Failed => {
                log::info!("Failed");
                self.1.draw(device, "Failed");
                Ok(None)
            }
        }
    }
}
