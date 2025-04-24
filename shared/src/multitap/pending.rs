#[derive(Debug)]
pub struct Pending<EVENT> {
    event: Option<EVENT>,
}

impl<EVENT> Default for Pending<EVENT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<EVENT> Pending<EVENT> {
    pub fn new() -> Self {
        Self { event: None }
    }

    pub fn enqueue(&mut self, event: EVENT) {
        self.event.replace(event);
    }

    pub fn dequeue(&mut self) -> Option<EVENT> {
        self.event.take()
    }
}
