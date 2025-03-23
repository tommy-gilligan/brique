#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(trivial_bounds)]
#![feature(let_chains)]
#![feature(associated_type_defaults)]
#![no_std]

pub mod abstract_menu;
pub mod confirmation;
pub mod console;
pub mod grid;
pub mod grid_menu;
pub mod held_key;
pub mod lock_screen;
pub mod menu;
pub mod multitap;
pub mod textbox;
pub mod time;
pub mod new_menu;

use core::{ascii::Char, future::Future};

use embassy_time::Duration;
use embedded_graphics_core::{draw_target::DrawTarget, pixelcolor::BinaryColor};
use enum_iterator::Sequence;
use strum_macros::IntoStaticStr;
use usbd_hid::descriptor::KeyboardUsage;

pub trait SystemResponse {
    fn take(&mut self) -> Option<[u8; 64]>;
}

pub trait Backlight {
    fn on(&mut self);
    fn off(&mut self);
}

pub trait VibrationMotor {
    fn start_vibrating(&mut self);
    fn stop_vibrating(&mut self);
}

pub trait Buzzer {
    type Error: core::fmt::Debug;

    fn set_frequency(&mut self, frequency: u16) -> Result<(), Self::Error>;
    fn set_volume(&mut self, volume: u8);
    fn mute_buzzer(&mut self) -> Result<(), Self::Error>;
    fn unmute_buzzer(&mut self) -> Result<(), Self::Error>;
}

pub enum ButtonEvent {
    Up,
    Down,
}

pub trait PowerButton {
    fn clear(&mut self);
}

pub trait Rtc {
    type Error: core::fmt::Debug;

    fn timestamp(&mut self) -> Result<i64, Self::Error>;
    fn set_timestamp(&mut self, time: i64);
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

    fn last_pressed(&mut self) -> Option<embassy_time::Duration>;
}

pub trait Application {
    // should record:
    // how long this takes
    // how long between calls
    fn run(
        &mut self,
        device: &mut impl Device,
        system_response: Option<[u8; 64]>,
    ) -> impl Future<Output = Result<Option<SystemRequest>, ()>>;
}

pub type UsbRx = [u8; 64];
#[derive(Clone, PartialEq)]
pub enum UsbTx {
    CdcBuffer([u8; 64]),
    HidChar(usbd_hid::descriptor::KeyboardReport),
}

#[derive(Clone, PartialEq)]
pub enum SystemRequest {
    UsbTx(UsbTx),
    ResetToBoot,
    SetTime(i64),
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
    fn start_watchdog(&mut self, duration: Duration);
    fn feed_watchdog(&mut self);
}

fn prepare_for_app(device: &mut impl Device) {
    log::debug!("Preparing device for app");
    device.clear(BinaryColor::On).unwrap();
    device.mute_buzzer().unwrap();
    device.stop_vibrating();
    device.on();
}

pub async fn run_app(
    mut app: impl Application,
    device: &mut impl Device,
    power: &mut impl PowerButton,
    system_response: &mut impl SystemResponse,
    system_request_handler: &mut impl SystemRequestHandler,
) {
    prepare_for_app(device);

    loop {
        device.start_watchdog(Duration::from_millis(2200));
        match device.last_pressed() {
            Some(last_pressed) if last_pressed > embassy_time::Duration::from_secs(5) => {
                device.off();
            }
            _ => device.on(),
        }
        match device.last_pressed() {
            Some(last_pressed) if last_pressed > embassy_time::Duration::from_secs(15) => {
                power.clear();
                device.off();
            }
            _ => {}
        }
        match embassy_time::with_timeout(
            embassy_time::Duration::from_millis(2000),
            app.run(device, system_response.take()),
        )
        .await
        {
            Ok(Ok(None)) => {}
            Ok(Ok(Some(e))) => {
                log::debug!("Handling system request");
                system_request_handler.handle_request(e).await;
            }
            Ok(Err(_)) => {}
            Err(embassy_time::TimeoutError) => {
                log::debug!("Timed out while waiting for app to return");
            }
        }

        // if power.was_pressed().await {
        //     prepare_for_app(device);
        //     device.feed_watchdog();
        //     return;
        // } else {
        //     device.feed_watchdog();
        // }
    }
}

pub fn build_report(c: Char) -> usbd_hid::descriptor::KeyboardReport {
    let keycode = match c {
        Char::Space => KeyboardUsage::KeyboardSpacebar,
        Char::Digit0 => KeyboardUsage::Keyboard0CloseParens,
        Char::Digit1 => KeyboardUsage::Keyboard1Exclamation,
        Char::Digit2 => KeyboardUsage::Keyboard2At,
        Char::Digit3 => KeyboardUsage::Keyboard3Hash,
        Char::Digit4 => KeyboardUsage::Keyboard4Dollar,
        Char::Digit5 => KeyboardUsage::Keyboard5Percent,
        Char::Digit6 => KeyboardUsage::Keyboard6Caret,
        Char::Digit7 => KeyboardUsage::Keyboard7Ampersand,
        Char::Digit8 => KeyboardUsage::Keyboard8Asterisk,
        Char::Digit9 => KeyboardUsage::Keyboard9OpenParens,
        Char::CapitalA => KeyboardUsage::KeyboardAa,
        Char::CapitalB => KeyboardUsage::KeyboardBb,
        Char::CapitalC => KeyboardUsage::KeyboardCc,
        Char::CapitalD => KeyboardUsage::KeyboardDd,
        Char::CapitalE => KeyboardUsage::KeyboardEe,
        Char::CapitalF => KeyboardUsage::KeyboardFf,
        Char::CapitalG => KeyboardUsage::KeyboardGg,
        Char::CapitalH => KeyboardUsage::KeyboardHh,
        Char::CapitalI => KeyboardUsage::KeyboardIi,
        Char::CapitalJ => KeyboardUsage::KeyboardJj,
        Char::CapitalK => KeyboardUsage::KeyboardKk,
        Char::CapitalL => KeyboardUsage::KeyboardLl,
        Char::CapitalM => KeyboardUsage::KeyboardMm,
        Char::CapitalN => KeyboardUsage::KeyboardNn,
        Char::CapitalO => KeyboardUsage::KeyboardOo,
        Char::CapitalP => KeyboardUsage::KeyboardPp,
        Char::CapitalQ => KeyboardUsage::KeyboardQq,
        Char::CapitalR => KeyboardUsage::KeyboardRr,
        Char::CapitalS => KeyboardUsage::KeyboardSs,
        Char::CapitalT => KeyboardUsage::KeyboardTt,
        Char::CapitalU => KeyboardUsage::KeyboardUu,
        Char::CapitalV => KeyboardUsage::KeyboardVv,
        Char::CapitalW => KeyboardUsage::KeyboardWw,
        Char::CapitalX => KeyboardUsage::KeyboardXx,
        Char::CapitalY => KeyboardUsage::KeyboardYy,
        Char::CapitalZ => KeyboardUsage::KeyboardZz,
        Char::SmallA => KeyboardUsage::KeyboardAa,
        Char::SmallB => KeyboardUsage::KeyboardBb,
        Char::SmallC => KeyboardUsage::KeyboardCc,
        Char::SmallD => KeyboardUsage::KeyboardDd,
        Char::SmallE => KeyboardUsage::KeyboardEe,
        Char::SmallF => KeyboardUsage::KeyboardFf,
        Char::SmallG => KeyboardUsage::KeyboardGg,
        Char::SmallH => KeyboardUsage::KeyboardHh,
        Char::SmallI => KeyboardUsage::KeyboardIi,
        Char::SmallJ => KeyboardUsage::KeyboardJj,
        Char::SmallK => KeyboardUsage::KeyboardKk,
        Char::SmallL => KeyboardUsage::KeyboardLl,
        Char::SmallM => KeyboardUsage::KeyboardMm,
        Char::SmallN => KeyboardUsage::KeyboardNn,
        Char::SmallO => KeyboardUsage::KeyboardOo,
        Char::SmallP => KeyboardUsage::KeyboardPp,
        Char::SmallQ => KeyboardUsage::KeyboardQq,
        Char::SmallR => KeyboardUsage::KeyboardRr,
        Char::SmallS => KeyboardUsage::KeyboardSs,
        Char::SmallT => KeyboardUsage::KeyboardTt,
        Char::SmallU => KeyboardUsage::KeyboardUu,
        Char::SmallV => KeyboardUsage::KeyboardVv,
        Char::SmallW => KeyboardUsage::KeyboardWw,
        Char::SmallX => KeyboardUsage::KeyboardXx,
        Char::SmallY => KeyboardUsage::KeyboardYy,
        Char::SmallZ => KeyboardUsage::KeyboardZz,
        _ => KeyboardUsage::KeyboardZz,
    };

    usbd_hid::descriptor::KeyboardReport {
        keycodes: [keycode as u8, 0, 0, 0, 0, 0],
        leds: 0,
        modifier: 0,
        reserved: 0,
    }
}
