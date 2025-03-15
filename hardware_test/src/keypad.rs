use enum_iterator::{first, last, next};
use shared::{Key, KeyEvent};

use crate::Status;

#[derive(Clone, PartialEq)]
pub struct KeypadTest<'a>(Key, shared::console::Console<'a>);

impl KeypadTest<'_> {
    pub fn new(key: Key) -> Self {
        Self(key, shared::console::Console::new())
    }
}

impl Default for KeypadTest<'_> {
    fn default() -> Self {
        Self::new(first().unwrap())
    }
}

impl KeypadTest<'_> {
    pub async fn run(
        &mut self,
        device: &mut impl shared::Device,
        _system_response: Option<[u8; 64]>,
    ) -> Status {
        self.1.draw(device, self.0.clone().into());
        match device.event().await {
            KeyEvent::Down(key) if key == self.0 => {
                if self.0 == last::<Key>().unwrap() {
                    return Status::Passed;
                } else {
                    self.0 = next(&self.0).unwrap();
                }
                Status::InProgress(None)
            }
            KeyEvent::Down(_key) => Status::Failed,
            _ => Status::InProgress(None),
        }
    }
}
