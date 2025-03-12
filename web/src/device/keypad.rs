use embassy_time::Timer;
use shared::{Key, Keypad};

unsafe impl Send for super::Device {}

impl Keypad for super::Device {
    async fn event(&mut self) -> shared::KeyEvent {
        loop {
            Timer::after_millis(30).await;
            if let Some(e) = (*self.cancel).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Cancel)
                } else {
                    shared::KeyEvent::Up(Key::Cancel)
                };
            } else if let Some(e) = (*self.select).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Select)
                } else {
                    shared::KeyEvent::Up(Key::Select)
                };
            } else if let Some(e) = (*self.up).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Up)
                } else {
                    shared::KeyEvent::Up(Key::Up)
                };
            } else if let Some(e) = (*self.down).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Down)
                } else {
                    shared::KeyEvent::Up(Key::Down)
                };
            } else if let Some(e) = (*self.one).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::One)
                } else {
                    shared::KeyEvent::Up(Key::One)
                };
            } else if let Some(e) = (*self.two).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Two)
                } else {
                    shared::KeyEvent::Up(Key::Two)
                };
            } else if let Some(e) = (*self.three).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Three)
                } else {
                    shared::KeyEvent::Up(Key::Three)
                };
            } else if let Some(e) = (*self.four).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Four)
                } else {
                    shared::KeyEvent::Up(Key::Four)
                };
            } else if let Some(e) = (*self.five).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Five)
                } else {
                    shared::KeyEvent::Up(Key::Five)
                };
            } else if let Some(e) = (*self.six).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Six)
                } else {
                    shared::KeyEvent::Up(Key::Six)
                };
            } else if let Some(e) = (*self.seven).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Seven)
                } else {
                    shared::KeyEvent::Up(Key::Seven)
                };
            } else if let Some(e) = (*self.eight).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Eight)
                } else {
                    shared::KeyEvent::Up(Key::Eight)
                };
            } else if let Some(e) = (*self.nine).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Nine)
                } else {
                    shared::KeyEvent::Up(Key::Nine)
                };
            } else if let Some(e) = (*self.asterisk).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Asterisk)
                } else {
                    shared::KeyEvent::Up(Key::Asterisk)
                };
            } else if let Some(e) = (*self.zero).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Zero)
                } else {
                    shared::KeyEvent::Up(Key::Zero)
                };
            } else if let Some(e) = (*self.hash).borrow_mut().check() {
                return if e == crate::Event::Down {
                    shared::KeyEvent::Down(Key::Hash)
                } else {
                    shared::KeyEvent::Up(Key::Hash)
                };
            }
        }
    }
}
