use js_sys::Date;
use shared::Rtc;

impl Rtc for super::Device {
    fn timestamp(&mut self) -> i64 {
        (Date::now() / 1000.0) as i64 + self.offset
    }
}
