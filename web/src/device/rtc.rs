use js_sys::Date;
use shared::Rtc;

impl Rtc for super::Device {
    type Error = ();

    fn timestamp(&mut self) -> Result<i64, ()> {
        Ok((Date::now() / 1000.0) as i64 + self.offset)
    }

    fn set_timestamp(&mut self, time: i64) {
        self.offset = time - ((Date::now() / 1000.0) as i64);
    }
}
