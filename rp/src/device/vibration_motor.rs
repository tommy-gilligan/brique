use embassy_rp::{
    gpio::{Level, Output},
    peripherals::PIN_2,
};
use shared::VibrationMotor;

pub struct Motor<'a>(Output<'a>);

use embassy_rp::Peri;

impl<'a> Motor<'a> {
    pub fn new(pin: Peri<'a, PIN_2>) -> Self {
        Self(Output::new(pin, Level::Low))
    }
}

impl VibrationMotor for Motor<'_> {
    fn start_vibrating(&mut self) {
        self.0.set_high();
    }

    fn stop_vibrating(&mut self) {
        self.0.set_low();
    }
}
