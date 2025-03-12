use core::cell::RefCell;
use std::rc::Rc;

use embassy_time::Timer;

pub struct DomPower {
    power: Rc<RefCell<super::DomB>>,
}

impl DomPower {
    pub fn new(power_id: &'static str) -> Self {
        Self {
            power: crate::DomB::new(power_id),
        }
    }
}

unsafe impl Send for DomPower {}

impl shared::PowerButton for DomPower {
    async fn was_pressed(&mut self) -> bool {
        Timer::after_millis(30).await;
        (*self.power).borrow_mut().check() == Some(crate::Event::Down)
    }
}
