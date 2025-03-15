use embassy_rp::{
    peripherals::{PIN_21, PWM_SLICE2},
    pwm::{Config, Pwm, SetDutyCycle},
};
use shared::Buzzer;

pub struct Beeper<'a>(Pwm<'a>, u16);

impl Beeper<'_> {
    pub fn new(slice: PWM_SLICE2, pin: PIN_21) -> Self {
        Self(Pwm::new_output_b(slice, pin, Config::default()), 0)
    }

    fn update(&mut self) {
        let mut c: embassy_rp::pwm::Config = Default::default();
        if self.1 == 0 {
            self.0.set_duty_cycle_percent(0).unwrap();
        } else {
            let divider = 16u8;
            let period =
                (embassy_rp::clocks::clk_sys_freq() / (self.1 as u32 * divider as u32)) as u16 - 1;

            c.top = period;
            c.divider = divider.into();

            self.0.set_config(&c);
            self.0.set_duty_cycle_percent(90).unwrap();
        }
    }
}

impl Buzzer for Beeper<'_> {
    fn mute(&mut self) {
        self.0.set_duty_cycle_percent(0).unwrap();
    }

    fn unmute(&mut self) {
        self.0.set_duty_cycle_percent(90).unwrap();
    }

    fn set_volume(&mut self, _volume: u8) {
        // self.0.set_duty_cycle_percent(volume).unwrap();
    }

    fn set_frequency(&mut self, frequency: u16) {
        self.1 = frequency;
        self.update();
    }
}
