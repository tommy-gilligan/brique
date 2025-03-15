use core::cell::RefCell;

use embassy_rp::peripherals::{
    PIN_2, PIN_4, PIN_5, PIN_6, PIN_7, PIN_8, PIN_9, PIN_10, PIN_11, PIN_12, PIN_13, PIN_14,
    PIN_15, PIN_16, PIN_17, PIN_18, PIN_19, PIN_20, PIN_21, PIN_33, PIN_36, PIN_37, PWM_SLICE2,
    SPI0,
};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embedded_graphics_core::{
    Pixel,
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget},
    primitives::Rectangle,
};
mod backlight;
mod buzzer;
mod display;
mod keypad;
mod vibration_motor;

pub struct Device<'a> {
    keypad: keypad::ContactKeypad<'a>,
    backlight: backlight::Light<'a>,
    vibration_motor: vibration_motor::Motor<'a>,
    buzzer: buzzer::Beeper<'a>,
    display: display::Display<'a>,
}

unsafe impl Send for Device<'_> {}

impl<'a> Device<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pin_2: PIN_2,
        pin_4: PIN_4,
        pin_5: PIN_5,
        pin_6: PIN_6,
        pin_7: PIN_7,
        pin_8: PIN_8,
        pin_9: PIN_9,
        pin_10: PIN_10,
        pin_11: PIN_11,
        pin_12: PIN_12,
        pin_13: PIN_13,
        pin_14: PIN_14,
        pin_15: PIN_15,
        pin_16: PIN_16,
        pin_17: PIN_17,
        pin_18: PIN_18,
        pin_19: PIN_19,
        pin_20: PIN_20,
        pin_21: PIN_21,
        pin_33: PIN_33,
        pin_36: PIN_36,
        pin_37: PIN_37,
        pwm_slice2: PWM_SLICE2,
        spi_bus: &'a embassy_sync::blocking_mutex::Mutex<
            NoopRawMutex,
            RefCell<embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>>,
        >,
    ) -> Self {
        Self {
            keypad: keypad::ContactKeypad::new(
                pin_16, pin_12, pin_9, pin_8, pin_17, pin_13, pin_7, pin_18, pin_14, pin_6, pin_19,
                pin_11, pin_5, pin_20, pin_10, pin_4,
            ),
            backlight: backlight::Light::new(pin_15),
            vibration_motor: vibration_motor::Motor::new(pin_2),
            buzzer: buzzer::Beeper::new(pwm_slice2, pin_21),
            display: display::Display::new(spi_bus, pin_37, pin_36, pin_33),
        }
    }
}

impl shared::Device for Device<'_> {}
use shared::{Backlight, Buzzer, Keypad, Rtc, VibrationMotor};

impl Backlight for Device<'_> {
    fn on(&mut self) {
        self.backlight.on();
    }

    fn off(&mut self) {
        self.backlight.off();
    }
}

impl Buzzer for Device<'_> {
    fn mute(&mut self) {
        self.buzzer.mute();
    }

    fn unmute(&mut self) {
        self.buzzer.unmute();
    }

    fn set_volume(&mut self, volume: u8) {
        self.buzzer.set_volume(volume);
    }

    fn set_frequency(&mut self, frequency: u16) {
        self.buzzer.set_frequency(frequency);
    }
}

impl VibrationMotor for Device<'_> {
    fn start(&mut self) {
        self.vibration_motor.start();
    }

    fn stop(&mut self) {
        self.vibration_motor.stop();
    }
}

impl Keypad for Device<'_> {
    async fn event(&mut self) -> shared::KeyEvent {
        self.keypad.event().await
    }
}

impl DrawTarget for Device<'_> {
    type Color = BinaryColor;

    type Error = ();

    fn draw_iter<I: IntoIterator<Item = Pixel<<Self as DrawTarget>::Color>>>(
        &mut self,
        i: I,
    ) -> Result<(), <Self as DrawTarget>::Error> {
        let _ = self.display.draw_iter(i);
        Ok(())
    }
}

impl Dimensions for Device<'_> {
    fn bounding_box(&self) -> Rectangle {
        self.display.bounding_box()
    }
}

pub struct Handler;

impl shared::SystemRequestHandler for Handler {
    async fn handle_request(&mut self, _request: shared::SystemRequest) {}
}

pub struct CdcSend;

impl shared::SystemResponse for CdcSend {
    fn take(&mut self) -> Option<[u8; 64]> {
        None
    }
}

impl Rtc for Device<'_> {
    fn timestamp(&mut self) -> i64 {
        0
    }
}
