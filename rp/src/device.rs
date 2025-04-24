use core::cell::RefCell;

use embassy_rp::{
    peripherals::{
        PIN_2, PIN_4, PIN_5, PIN_6, PIN_7, PIN_8, PIN_9, PIN_10, PIN_11, PIN_12, PIN_13, PIN_14,
        PIN_15, PIN_16, PIN_17, PIN_18, PIN_19, PIN_20, PIN_21, PIN_33, PIN_36, PIN_37, PWM_SLICE2,
        SPI0,
    },
    pwm::PwmError,
    watchdog::Watchdog,
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
    watchdog: Watchdog,
}
use embassy_rp::Peri;

unsafe impl Send for Device<'_> {}

impl<'a> Device<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        watchdog: embassy_rp::watchdog::Watchdog,
        pin_2: Peri<'a, PIN_2>,
        pin_4: Peri<'a, PIN_4>,
        pin_5: Peri<'a, PIN_5>,
        pin_6: Peri<'a, PIN_6>,
        pin_7: Peri<'a, PIN_7>,
        pin_8: Peri<'a, PIN_8>,
        pin_9: Peri<'a, PIN_9>,
        pin_10: Peri<'a, PIN_10>,
        pin_11: Peri<'a, PIN_11>,
        pin_12: Peri<'a, PIN_12>,
        pin_13: Peri<'a, PIN_13>,
        pin_14: Peri<'a, PIN_14>,
        pin_15: Peri<'a, PIN_15>,
        pin_16: Peri<'a, PIN_16>,
        pin_17: Peri<'a, PIN_17>,
        pin_18: Peri<'a, PIN_18>,
        pin_19: Peri<'a, PIN_19>,
        pin_20: Peri<'a, PIN_20>,
        pin_21: Peri<'a, PIN_21>,
        pin_33: Peri<'a, PIN_33>,
        pin_36: Peri<'a, PIN_36>,
        pin_37: Peri<'a, PIN_37>,
        pwm_slice2: Peri<'a, PWM_SLICE2>,
        spi_bus: &'a embassy_sync::blocking_mutex::Mutex<
            NoopRawMutex,
            RefCell<embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>>,
        >,
    ) -> Result<Self, display_interface::DisplayError> {
        Ok(Self {
            keypad: keypad::ContactKeypad::new(
                pin_16, pin_12, pin_9, pin_8, pin_17, pin_13, pin_7, pin_18, pin_14, pin_6, pin_19,
                pin_11, pin_5, pin_20, pin_10, pin_4,
            ),
            backlight: backlight::Light::new(pin_15),
            vibration_motor: vibration_motor::Motor::new(pin_2),
            buzzer: buzzer::Beeper::new(pwm_slice2, pin_21),
            display: display::Display::new(spi_bus, pin_37, pin_36, pin_33)?,
            watchdog,
        })
    }
}

impl shared::Device for Device<'_> {
    fn start_watchdog(&mut self, duration: embassy_time::Duration) {
        self.watchdog.start(duration);
    }

    fn feed_watchdog(&mut self) {
        self.watchdog.feed();
    }
}
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
    type Error = PwmError;

    fn mute_buzzer(&mut self) -> Result<(), Self::Error> {
        self.buzzer.mute_buzzer()
    }

    fn unmute_buzzer(&mut self) -> Result<(), Self::Error> {
        self.buzzer.unmute_buzzer()
    }

    fn set_volume(&mut self, volume: u8) {
        self.buzzer.set_volume(volume);
    }

    fn set_frequency(&mut self, frequency: u16) -> Result<(), Self::Error> {
        self.buzzer.set_frequency(frequency)
    }
}

impl VibrationMotor for Device<'_> {
    fn start_vibrating(&mut self) {
        self.vibration_motor.start_vibrating();
    }

    fn stop_vibrating(&mut self) {
        self.vibration_motor.stop_vibrating();
    }
}

impl Keypad for Device<'_> {
    async fn event(&mut self) -> shared::KeyEvent {
        self.keypad.event().await
    }
    fn last_pressed(&mut self) -> Option<embassy_time::Duration> {
        todo!()
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

pub struct SystemRequestHandler;

impl shared::SystemRequestHandler for SystemRequestHandler {
    async fn handle_request(&mut self, _request: shared::SystemRequest) {}
}

pub struct CdcSend;

impl shared::SystemResponse for CdcSend {
    fn take(&mut self) -> Option<[u8; 64]> {
        None
    }
}

impl Rtc for Device<'_> {
    type Error = ();

    fn timestamp(&mut self) -> Result<i64, ()> {
        Ok(0)
    }

    fn set_timestamp(&mut self, _: i64) {
        todo!()
    }
}
