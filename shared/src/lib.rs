#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(trivial_bounds)]
#![feature(let_chains)]
#![feature(associated_type_defaults)]
#![no_std]

pub mod confirmation;
pub mod console;
pub mod grid_menu;
pub mod held_key;
pub mod lock_screen;
pub mod menu;
pub mod multitap;
pub mod textbox;
pub mod time;
pub mod abstract_menu;

use core::{ascii::Char, future::Future};

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
    fn run(
        &mut self,
        device: &mut impl Device,
        system_response: Option<[u8; 64]>,
    ) -> impl Future<Output = Option<SystemRequest>>;
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

pub trait Device:
    VibrationMotor + Buzzer + Keypad + Rtc + Backlight + DrawTarget<Color = BinaryColor, Error = ()>
{
}

// decide your time budgets
// 'trust' application takes at most 750ms
// force pre-emption at 1500ms
// how do you progress things inside app that take longer than 750?
// special kind of timer?
// forced pre-emption should be signalled back to application + print log entry
#[allow(clippy::too_many_arguments)]
pub async fn run_app(
    mut app: impl Application,
    device: &mut impl Device,
    power: &mut impl PowerButton,
    // just usb rx for now
    system_response: Option<[u8; 64]>,
    system_request_handler: &mut impl SystemRequestHandler,
) {
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    device
        .bounding_box()
        .into_styled(fill)
        .draw(device)
        .unwrap();
    device.mute();
    device.stop();

    loop {
        match embassy_time::with_timeout(
            embassy_time::Duration::from_millis(2000),
            app.run(device, system_response),
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
            device
                .bounding_box()
                .into_styled(fill)
                .draw(device)
                .unwrap();
            device.mute();
            device.stop();
            return;
        }
    }
}
