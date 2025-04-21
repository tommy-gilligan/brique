use embedded_graphics::{mock_display::MockDisplay, pixelcolor::BinaryColor};

mod held_key;
mod keypad;

pub struct Device {
    pub display: MockDisplay<BinaryColor>,
}

impl Device {
    pub fn new() -> Self {
        let mut display = MockDisplay::new();
        display.set_allow_out_of_bounds_drawing(false);
        display.set_allow_overdraw(true);

        Self { display }
    }
}

impl shared::Keypad for Device {
    async fn event(&mut self) -> shared::KeyEvent {
        shared::KeyEvent::Down(shared::Key::Down)
    }

    fn last_pressed(&mut self) -> Option<embassy_time::Duration> {
        None
    }
}
