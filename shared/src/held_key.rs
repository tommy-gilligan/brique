use embassy_time::Timer;
use futures::{future, future::Either, pin_mut};

use crate::{Key, KeyEvent};

#[derive(Debug, PartialEq)]
pub enum Event {
    Down(Key),
    Delay(Key),
    Repeat(Key),
}

pub struct HeldKey {
    down: Option<Key>,
    timer: Option<Timer>,
    delay_duration: u64,
    repeat_period: u64,
    repeating: bool,
}

impl HeldKey {
    pub fn new(delay_duration: u64, repeat_period: u64) -> Self {
        Self {
            down: None,
            timer: None,
            delay_duration,
            repeat_period,
            repeating: false,
        }
    }

    fn timeout_event(&mut self) -> Option<Event> {
        let result = self.down.clone();

        if self.repeating {
            self.timer = Some(embassy_time::Timer::after_millis(self.repeat_period));
            result.map(Event::Repeat)
        } else {
            self.repeating = true;
            result.map(Event::Delay)
        }
    }

    fn key_event(&mut self, key_event: KeyEvent) -> Option<Event> {
        match key_event {
            KeyEvent::Down(key) => {
                self.timer = Some(embassy_time::Timer::after_millis(self.delay_duration));
                self.down = Some(key.clone());
                Some(Event::Down(key))
            }
            KeyEvent::Up(_key) => {
                self.timer = None;
                self.down = None;
                self.repeating = false;
                None
            }
        }
    }

    pub async fn event<KEYPAD>(&mut self, keypad: &mut KEYPAD) -> Option<Event>
    where
        KEYPAD: crate::Keypad,
    {
        let event_future = keypad.event();
        pin_mut!(event_future);

        if let Some(timer) = &mut self.timer {
            match future::select(timer, event_future).await {
                Either::Left((..)) => self.timeout_event(),
                Either::Right((e, _)) => self.key_event(e),
            }
        } else {
            self.key_event(event_future.await)
        }
    }
}
