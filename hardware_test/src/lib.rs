#![feature(let_chains)]
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

#[derive(Clone, PartialEq)]
pub enum Status {
    Passed,
    Failed,
    InProgress,
}

#[derive(Clone, Sequence, PartialEq)]
enum Test<'a> {
    Keypad(KeypadTest<'a>),
    Vibration(VibrationTest<'a>),
    Buzzer(BuzzerTest<'a>),
    Backlight(BacklightTest<'a>),
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
                self.2 = Test::Keypad(Default::default());
            }
        }
    }
}

impl Default for HardwareTest<'_> {
    fn default() -> Self {
        Self::new(Status::InProgress)
    }
}

impl Application for HardwareTest<'_> {
    async fn run(
        &mut self,
        device: &mut impl shared::Device,
        _system_response: Option<[u8; 64]>,
    ) -> Option<shared::SystemRequest> {
        match self.0 {
            Status::InProgress => match self.2 {
                Test::Keypad(ref mut test) => match test.run(device).await {
                    Status::Passed => self.next(),
                    Status::Failed => {
                        self.0 = Status::Failed;
                    }
                    _ => {}
                },
                Test::Vibration(ref mut test) => match test.run(device).await {
                    Status::Passed => self.next(),
                    Status::Failed => {
                        self.0 = Status::Failed;
                    }
                    _ => {}
                },
                Test::Buzzer(ref mut test) => match test.run(device).await {
                    Status::Passed => self.next(),
                    Status::Failed => {
                        self.0 = Status::Failed;
                    }
                    _ => {}
                },
                Test::Backlight(ref mut test) => match test.run(device).await {
                    Status::Passed => {
                        self.0 = Status::Passed;
                    }
                    Status::Failed => {
                        self.0 = Status::Failed;
                    }
                    _ => {}
                },
            },
            Status::Passed => {
                self.1.draw(device, "Passed");
            }
            Status::Failed => {
                self.1.draw(device, "Failed");
            }
        }

        None
    }
}
