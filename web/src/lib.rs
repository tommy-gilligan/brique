#![allow(unexpected_cfgs)]
mod backlight;
mod buzzer;
mod display;
mod keypad;
mod power;
mod rtc;
mod vibration_motor;

use embassy_executor::Spawner;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    wasm_logger::init(wasm_logger::Config::default());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let svg = document.get_element_by_id("svg1").unwrap();
    let mut vibration_motor = vibration_motor::Motor::new(svg);

    let svg = document.get_element_by_id("nokia").unwrap();
    let mut buzzer = buzzer::Buzzer::new(svg);
    let mut rtc = rtc::Clock::new();

    let svg = document.get_element_by_id("display").unwrap();
    let mut display = display::Display::new(svg);

    let svg = document.get_element_by_id("backlight").unwrap();
    let mut light = backlight::Light::new(svg);
    let mut power = power::DomPower::new("power");

    let items = ["Clock", "Hardware Test", "Keyboard", "Reboot to USB"];
    let mut menu = shared::menu::Menu::new(&items);
    let mut keypad = keypad::DomKeypad::new(
        "cancel", "select", "up", "down", "one", "two", "three", "four", "five", "six", "seven",
        "eight", "nine", "asterisk", "zero", "hash",
    );
    loop {
            let result = match menu.process(&mut keypad, &mut display).await {
                0 => {
                    let clock = clock::Clock;
                    shared::run_app(
                        clock,
                        &mut vibration_motor,
                        &mut buzzer,
                        &mut display,
                        &mut keypad,
                        &mut rtc,
                        &mut light,
                        &mut power,
                        None,
                    ).await
                }
                1 => {
                    let hardware_test = hardware_test::HardwareTest::default();
                    shared::run_app(
                        hardware_test,
                        &mut vibration_motor,
                        &mut buzzer,
                        &mut display,
                        &mut keypad,
                        &mut rtc,
                        &mut light,
                        &mut power,
                        None,
                    ).await
                }
                2 => {
                    let keyboard = keyboard::Keyboard;
                    shared::run_app(
                        keyboard,
                        &mut vibration_motor,
                        &mut buzzer,
                        &mut display,
                        &mut keypad,
                        &mut rtc,
                        &mut light,
                        &mut power,
                        None,
                    ).await
                }
                _ => {
                    let reset = reset_to_boot::ResetToBoot;
                    shared::run_app(
                        reset,
                        &mut vibration_motor,
                        &mut buzzer,
                        &mut display,
                        &mut keypad,
                        &mut rtc,
                        &mut light,
                        &mut power,
                        None,
                    ).await
                }
            };
            match result {
                _ => unimplemented!()
            }
    }
}

use core::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{JsCast, closure::Closure};

struct DomB {
    was_clicked: bool,
}

impl DomB {
    #[allow(clippy::too_many_arguments)]
    fn new(id: &'static str) -> Rc<RefCell<Self>> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let s = Self { was_clicked: false };
        let r = Rc::new(RefCell::new(s));
        let g = r.clone();

        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            (*g).borrow_mut().was_clicked = true;
        });

        document
            .get_element_by_id(id)
            .unwrap()
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget();

        r
    }

    fn check(&mut self) -> bool {
        let result = self.was_clicked;
        self.was_clicked = false;
        result
    }
}
