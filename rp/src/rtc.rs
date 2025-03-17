use embassy_rp::peripherals::{I2C1, PIN_46, PIN_47};
use shared::Rtc;
use unofficial_piicodev::Driver;

pub struct Clock<'a>(
    unofficial_piicodev::p19::P19<embassy_rp::i2c::I2c<'a, I2C1, embassy_rp::i2c::Blocking>>,
);

impl Clock<'_> {
    pub fn new(i2c: I2C1, sda: PIN_46, scl: PIN_47) -> Result<Self, unofficial_piicodev::OutOfRange> {
        Ok(
            Self(
                unofficial_piicodev::p19::P19::new(
                    embassy_rp::i2c::I2c::new_blocking(
                        i2c,
                        scl,
                        sda,
                        embassy_rp::i2c::Config::default(),
                    ),
                    0x52,
                )?
            )
        )
    }
}

impl Rtc for Clock<'_> {
    type Error = embassy_rp::i2c::Error;

    fn timestamp(&mut self) -> Result<i64, Self::Error> {
        Ok(self.0.get_unix_time()?.into())
    }
}
