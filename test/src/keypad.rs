pub struct Keypad<'a>(
    core::slice::Iter<'a, shared::KeyEvent>,
    Option<embassy_time::Instant>,
    bool,
);

impl<'a> Keypad<'a> {
    pub fn new(events: &'a [shared::KeyEvent]) -> Self {
        Self(events.iter(), None, false)
    }

    pub fn pending(&mut self) {
        self.2 = true;
    }
}

impl<'a> shared::Keypad for Keypad<'a> {
    async fn event(&mut self) -> shared::KeyEvent {
        if self.2 {
            core::future::pending().await
        } else {
            let result = self.0.next().unwrap().clone();
            self.1.replace(embassy_time::Instant::now());

            result
        }
    }

    fn last_pressed(&mut self) -> Option<embassy_time::Duration> {
        if let Some(last_time_pressed) = self.1 {
            return Some(embassy_time::Instant::now() - last_time_pressed);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use futures_executor::block_on;
    use shared::Keypad;

    use super::*;

    #[test]
    fn test_keypad() {
        let f = async {
            let driver = embassy_time::MockDriver::get();
            driver.reset();
            let mut keypad = super::Keypad::new(&[
                shared::KeyEvent::Down(shared::Key::Two),
                shared::KeyEvent::Down(shared::Key::Three),
            ]);
            assert_eq!(keypad.last_pressed(), None);
            assert_eq!(
                keypad.event().await,
                shared::KeyEvent::Down(shared::Key::Two)
            );
            assert_eq!(
                keypad.last_pressed(),
                Some(embassy_time::Duration::from_secs(0))
            );
            let instant = embassy_time::Instant::now();
            assert_eq!(
                keypad.event().await,
                shared::KeyEvent::Down(shared::Key::Three)
            );
            driver.advance(embassy_time::Duration::from_secs(1));
            assert_eq!(
                keypad.last_pressed(),
                Some(embassy_time::Duration::from_secs(1))
            );
        };
        block_on(f);
    }
}
