use embassy_time::Timer;
use shared::{Key, Keypad};

unsafe impl Send for super::Device {}

impl super::Device {
    async fn base_event(&mut self) -> shared::KeyEvent {
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
            } else if let Some(e) = (*self.keyboard).borrow_mut().check() {
                return match e {
                    crate::KeyEvent::Down('0') => shared::KeyEvent::Down(Key::Zero),
                    crate::KeyEvent::Down('1') => shared::KeyEvent::Down(Key::One),
                    crate::KeyEvent::Down('2') => shared::KeyEvent::Down(Key::Two),
                    crate::KeyEvent::Down('3') => shared::KeyEvent::Down(Key::Three),
                    crate::KeyEvent::Down('4') => shared::KeyEvent::Down(Key::Four),
                    crate::KeyEvent::Down('5') => shared::KeyEvent::Down(Key::Five),
                    crate::KeyEvent::Down('6') => shared::KeyEvent::Down(Key::Six),
                    crate::KeyEvent::Down('7') => shared::KeyEvent::Down(Key::Seven),
                    crate::KeyEvent::Down('8') => shared::KeyEvent::Down(Key::Eight),
                    crate::KeyEvent::Down('9') => shared::KeyEvent::Down(Key::Nine),
                    crate::KeyEvent::Down('*') => shared::KeyEvent::Down(Key::Asterisk),
                    crate::KeyEvent::Down('#') => shared::KeyEvent::Down(Key::Hash),
                    crate::KeyEvent::Down('u') => shared::KeyEvent::Down(Key::Up),
                    crate::KeyEvent::Down('d') => shared::KeyEvent::Down(Key::Down),
                    crate::KeyEvent::Down('E') => shared::KeyEvent::Down(Key::Select),
                    crate::KeyEvent::Down('e') => shared::KeyEvent::Down(Key::Cancel),
                    crate::KeyEvent::Up('0') => shared::KeyEvent::Up(Key::Zero),
                    crate::KeyEvent::Up('1') => shared::KeyEvent::Up(Key::One),
                    crate::KeyEvent::Up('2') => shared::KeyEvent::Up(Key::Two),
                    crate::KeyEvent::Up('3') => shared::KeyEvent::Up(Key::Three),
                    crate::KeyEvent::Up('4') => shared::KeyEvent::Up(Key::Four),
                    crate::KeyEvent::Up('5') => shared::KeyEvent::Up(Key::Five),
                    crate::KeyEvent::Up('6') => shared::KeyEvent::Up(Key::Six),
                    crate::KeyEvent::Up('7') => shared::KeyEvent::Up(Key::Seven),
                    crate::KeyEvent::Up('8') => shared::KeyEvent::Up(Key::Eight),
                    crate::KeyEvent::Up('9') => shared::KeyEvent::Up(Key::Nine),
                    crate::KeyEvent::Up('*') => shared::KeyEvent::Up(Key::Asterisk),
                    crate::KeyEvent::Up('#') => shared::KeyEvent::Up(Key::Hash),
                    crate::KeyEvent::Up('u') => shared::KeyEvent::Up(Key::Up),
                    crate::KeyEvent::Up('d') => shared::KeyEvent::Up(Key::Down),
                    crate::KeyEvent::Up('E') => shared::KeyEvent::Up(Key::Select),
                    crate::KeyEvent::Up('e') => shared::KeyEvent::Up(Key::Cancel),

                    _ => shared::KeyEvent::Down(Key::Select),
                };
            }
        }
    }
}

impl Keypad for super::Device {
    async fn event(&mut self) -> shared::KeyEvent {
        let result = self.base_event().await;
        self.last_time_pressed.replace(embassy_time::Instant::now());
        result
    }

    fn last_pressed(&mut self) -> Option<embassy_time::Duration> {
        if let Some(last_time_pressed) = self.last_time_pressed {
            return Some(embassy_time::Instant::now() - last_time_pressed);
        }
        None
    }
}
