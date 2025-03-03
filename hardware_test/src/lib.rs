#![no_std]

use core::fmt::Debug;

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle},
};
use shared::{Application, Key, KeyEvent};

pub struct HardwareTest(i32);

impl HardwareTest {
    pub fn new(yoffset: i32) -> Self {
        Self(yoffset)
    }
}

impl Default for HardwareTest {
    fn default() -> Self {
        Self::new(10)
    }
}

impl Application for HardwareTest {
    async fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        vibration_motor: &mut impl shared::VibrationMotor,
        buzzer: &mut impl shared::Buzzer,
        display: &mut D,
        keypad: &mut impl shared::Keypad,
        _rtc: &mut impl shared::Rtc,
        backlight: &mut impl shared::Backlight,
        _system_response: Option<[u8; 64]>,
    ) -> Option<shared::SystemRequest>
    where
        <D as DrawTarget>::Error: Debug,
    {
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::Off, 1);
        let border_stroke = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::Off)
            .stroke_width(3)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let fill = PrimitiveStyle::with_fill(BinaryColor::Off);

        display
            .bounding_box()
            .into_styled(border_stroke)
            .draw(display)
            .unwrap();

        match keypad.event().await {
            KeyEvent::Down(Key::Down) => {
                self.0 -= 1;
            }
            KeyEvent::Down(Key::Up) => {
                self.0 += 1;
            }
            KeyEvent::Down(Key::One) => {
                buzzer.unmute();
            }
            KeyEvent::Down(Key::Two) => {
                buzzer.mute();
            }
            KeyEvent::Down(Key::Four) => {
                buzzer.set_frequency(440);
            }
            KeyEvent::Down(Key::Five) => {
                buzzer.set_frequency(660);
            }
            KeyEvent::Down(Key::Six) => {
                buzzer.set_frequency(880);
            }
            KeyEvent::Down(Key::Eight) => {
                vibration_motor.start();
            }
            KeyEvent::Down(Key::Seven) => {
                vibration_motor.stop();
            }
            KeyEvent::Down(Key::Nine) => {
                backlight.on();
            }
            KeyEvent::Down(Key::Three) => {
                backlight.off();
            }
            KeyEvent::Up(_) => {}
            KeyEvent::Down(Key::Select) => {
            }
            KeyEvent::Down(Key::Cancel) => {
            }
            KeyEvent::Down(Key::Asterisk) => {
            }
            KeyEvent::Down(Key::Zero) => {
            }
            KeyEvent::Down(Key::Hash) => {
            }
        }

        Triangle::new(
            Point::new(16, 16 + self.0),
            Point::new(16 + 16, 16 + self.0),
            Point::new(16 + 8, self.0),
        )
        .into_styled(thin_stroke)
        .draw(display)
        .unwrap();

        Rectangle::new(Point::new(52, self.0), Size::new(16, 16))
            .into_styled(fill)
            .draw(display)
            .unwrap();

        None
    }
}
