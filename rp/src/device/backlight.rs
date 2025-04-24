use embassy_rp::{
    Peri,
    gpio::{Level, Output},
    peripherals::PIN_15,
};
use shared::Backlight;

pub struct Light<'a>(Output<'a>);

impl<'a> Light<'a> {
    pub fn new(pin: Peri<'a, PIN_15>) -> Self {
        Self(Output::new(pin, Level::Low))
    }
}

impl Backlight for Light<'_> {
    fn on(&mut self) {
        self.0.set_high();
    }

    fn off(&mut self) {
        self.0.set_low();
    }
}
