use embassy_time::Timer;
use futures::{future, future::Either, pin_mut};

use crate::{Key, KeyEvent};

pub enum Event {
    Down(Key),
    Held(Key),
}

pub struct HeldKey {
    down: Option<Key>,
    timer: Option<Timer>,
    held_duration: u64,
}

impl HeldKey {
    pub fn new(held_duration: u64) -> Self {
        Self {
            down: None,
            timer: None,
            held_duration,
        }
    }

    fn set_key_down(&mut self, key: Key) {
        self.timer = Some(embassy_time::Timer::after_millis(self.held_duration));
        self.down = Some(key.clone());
    }

    fn clear_key_down(&mut self) {
        self.timer = None;
        self.down = None;
    }

    fn timeout_event(&mut self) -> Option<Event> {
        let result = self.down.clone();
        self.clear_key_down();
        result.map(Event::Held)
    }

    fn key_event(&mut self, key_event: KeyEvent) -> Option<Event> {
        match key_event {
            KeyEvent::Down(key) => {
                self.set_key_down(key.clone());
                Some(Event::Down(key))
            }
            KeyEvent::Up(key) => {
                self.clear_key_down();
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
