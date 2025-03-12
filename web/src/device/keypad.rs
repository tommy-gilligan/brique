use embassy_time::Timer;
use shared::{Key, Keypad};

unsafe impl Send for super::Device {}

impl Keypad for super::Device {
    async fn event(&mut self) -> shared::KeyEvent {
        loop {
            Timer::after_millis(30).await;
            if (*self.cancel).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Cancel);
            } else if (*self.select).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Select);
            } else if (*self.up).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Up);
            } else if (*self.down).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Down);
            } else if (*self.one).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::One);
            } else if (*self.two).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Two);
            } else if (*self.three).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Three);
            } else if (*self.four).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Four);
            } else if (*self.five).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Five);
            } else if (*self.six).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Six);
            } else if (*self.seven).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Seven);
            } else if (*self.eight).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Eight);
            } else if (*self.nine).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Nine);
            } else if (*self.asterisk).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Asterisk);
            } else if (*self.zero).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Zero);
            } else if (*self.hash).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Hash);
            }
        }
    }
}
