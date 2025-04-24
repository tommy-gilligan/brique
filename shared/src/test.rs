pub struct Keypad<'a>(
    core::slice::Iter<'a, crate::KeyEvent>,
    Option<embassy_time::Instant>,
    bool,
);

#[cfg(test)]
impl<'a> Keypad<'a> {
    pub fn new(events: &'a [crate::KeyEvent]) -> Self {
        let driver = embassy_time::MockDriver::get();
        driver.reset();
        Self(events.iter(), None, false)
    }

    pub fn pending(&mut self) {
        self.2 = true;
    }

    pub fn advance_by_millis(&self, millis: u64) {
        embassy_time::MockDriver::get().advance(embassy_time::Duration::from_millis(millis));
    }
}

impl<'a> crate::Keypad for Keypad<'a> {
    async fn event(&mut self) -> crate::KeyEvent {
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

    use crate::Keypad;

    #[test]
    fn test_keypad() {
        block_on(async {
            let mut keypad = super::Keypad::new(&[
                crate::KeyEvent::Down(crate::Key::Two),
                crate::KeyEvent::Down(crate::Key::Three),
            ]);

            assert_eq!(keypad.last_pressed(), None);
            assert_eq!(keypad.event().await, crate::KeyEvent::Down(crate::Key::Two));
            assert_eq!(
                keypad.last_pressed(),
                Some(embassy_time::Duration::from_secs(0))
            );
            assert_eq!(
                keypad.event().await,
                crate::KeyEvent::Down(crate::Key::Three)
            );
            keypad.advance_by_millis(1000);
            assert_eq!(
                keypad.last_pressed(),
                Some(embassy_time::Duration::from_millis(1000))
            );
        });
    }
}
