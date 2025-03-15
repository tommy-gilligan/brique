use embassy_rp::{
    gpio::{Level, Output},
    peripherals::PIN_2,
};
use shared::VibrationMotor;

pub struct Motor<'a>(Output<'a>);

impl Motor<'_> {
    pub fn new(pin: PIN_2) -> Self {
        Self(Output::new(pin, Level::Low))
    }
}

impl VibrationMotor for Motor<'_> {
    fn start(&mut self) {
        self.0.set_high();
    }

    fn stop(&mut self) {
        self.0.set_low();
    }
}
