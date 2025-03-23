use embassy_rp::{
    gpio::{Level, Output},
    peripherals::PIN_28,
};
use shared::PowerButton;

pub struct Button<'a>(Output<'a>);

impl Button<'_> {
    pub fn new(pin: PIN_28) -> Self {
        Self(Output::new(pin, Level::Low))
    }
}

impl PowerButton for Button<'_> {
    fn clear(&mut self) {
        self.0.set_high();
    }
}
