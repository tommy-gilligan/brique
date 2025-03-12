#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(trivial_bounds)]
#![feature(let_chains)]
#![no_std]

pub mod confirmation;
pub mod console;
pub mod lock_screen;
pub mod menu;
pub mod multitap;
pub mod textbox;
pub mod time;

use core::{ascii::Char, fmt::Debug, future::Future};

use embedded_graphics::{Drawable, prelude::Primitive, primitives::PrimitiveStyle};
use embedded_graphics_core::{draw_target::DrawTarget, pixelcolor::BinaryColor};
use enum_iterator::Sequence;
use strum_macros::IntoStaticStr;

pub trait Backlight {
    fn on(&mut self);
    fn off(&mut self);
}

pub trait VibrationMotor {
    fn start(&mut self);
    fn stop(&mut self);
}

pub trait Buzzer {
    fn set_frequency(&mut self, frequency: u16);
    fn set_volume(&mut self, volume: u8);
    fn mute(&mut self);
    fn unmute(&mut self);
}

pub enum ButtonEvent {
    Up,
    Down,
}

pub trait PowerButton {
    fn was_pressed(&mut self) -> impl core::future::Future<Output = bool> + core::marker::Send;
}

pub trait Rtc {
    fn timestamp(&mut self) -> i64;
}

#[derive(Clone, IntoStaticStr, Sequence, PartialEq)]
pub enum Key {
    Select,
    Cancel,
    Up,
    Down,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Asterisk,
    Zero,
    Hash,
}

impl From<Key> for Char {
    fn from(k: Key) -> Char {
        match k {
            Key::Select => Char::CarriageReturn,
            Key::Cancel => Char::Backspace,
            Key::Up => Char::Digit1,
            Key::Down => Char::Digit1,
            Key::One => Char::Digit1,
            Key::Two => Char::CapitalA,
            Key::Three => Char::CapitalD,
            Key::Four => Char::CapitalG,
            Key::Five => Char::CapitalJ,
            Key::Six => Char::CapitalM,
            Key::Seven => Char::CapitalP,
            Key::Eight => Char::CapitalT,
            Key::Nine => Char::CapitalW,
            Key::Asterisk => Char::Asterisk,
            Key::Zero => Char::Space,
            Key::Hash => Char::NumberSign,
        }
    }
}

#[derive(PartialEq)]
pub enum KeyEvent {
    Up(Key),
    Down(Key),
}

pub trait Keypad {
    fn event(&mut self) -> impl core::future::Future<Output = KeyEvent> + core::marker::Send;
}

pub trait Application {
    // should record:
    // how long this takes
    // how long between calls
    #[allow(clippy::too_many_arguments)]
    fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        vibration_motor: &mut impl VibrationMotor,
        buzzer: &mut impl Buzzer,
        display: &mut D,
        keypad: &mut impl Keypad,
        rtc: &mut impl Rtc,
        backlight: &mut impl Backlight,
        system_response: Option<[u8; 64]>,
    ) -> impl Future<Output = Option<SystemRequest>>
    where
        <D as DrawTarget>::Error: Debug;
}

pub type UsbRx = [u8; 64];
pub enum UsbTx {
    CdcBuffer([u8; 64]),
    HidChar(usbd_hid::descriptor::KeyboardReport),
}

pub enum SystemRequest {
    UsbTx(UsbTx),
    ResetToBoot,
}

pub trait SystemRequestHandler {
    fn handle_request(
        &mut self,
        system_request: SystemRequest,
    ) -> impl core::future::Future<Output = ()>;
}

// decide your time budgets
// 'trust' application takes at most 750ms
// force pre-emption at 1500ms
// how do you progress things inside app that take longer than 750?
// special kind of timer?
// forced pre-emption should be signalled back to application + print log entry
#[allow(clippy::too_many_arguments)]
pub async fn run_app<D: DrawTarget<Color = BinaryColor>>(
    mut app: impl Application,
    vibration_motor: &mut impl VibrationMotor,
    buzzer: &mut impl Buzzer,
    display: &mut D,
    keypad: &mut impl Keypad,
    rtc: &mut impl Rtc,
    light: &mut impl Backlight,
    power: &mut impl PowerButton,
    // just usb rx for now
    system_response: Option<[u8; 64]>,
    system_request_handler: &mut impl SystemRequestHandler,
) where
    <D as DrawTarget>::Error: Debug,
{
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    display
        .bounding_box()
        .into_styled(fill)
        .draw(display)
        .unwrap();
    buzzer.mute();
    vibration_motor.stop();

    loop {
        match embassy_time::with_timeout(
            embassy_time::Duration::from_millis(2000),
            app.run(
                vibration_motor,
                buzzer,
                display,
                keypad,
                rtc,
                light,
                system_response,
            ),
        )
        .await
        {
            Ok(None) => {}
            Ok(Some(e)) => {
                system_request_handler.handle_request(e).await;
            }
            Err(embassy_time::TimeoutError) => {
                log::info!("timed out");
            }
        }

        if power.was_pressed().await {
            let fill = PrimitiveStyle::with_fill(BinaryColor::On);
            display
                .bounding_box()
                .into_styled(fill)
                .draw(display)
                .unwrap();
            buzzer.mute();
            vibration_motor.stop();
            return;
        }
    }
}
