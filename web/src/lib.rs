#![allow(unexpected_cfgs)]
mod device;

use embassy_executor::Spawner;
use embedded_graphics::pixelcolor::{BinaryColor, PixelColor};
use shared::Application;

#[derive(Clone, PartialEq, Copy)]
enum Inverted {
    On,
    Off,
}

impl PixelColor for Inverted {
    type Raw = <BinaryColor as PixelColor>::Raw;
}

impl From<Inverted> for BinaryColor {
    fn from(val: Inverted) -> Self {
        match val {
            Inverted::On => BinaryColor::On,
            Inverted::Off => BinaryColor::Off,
        }
    }
}

impl From<BinaryColor> for Inverted {
    fn from(v: BinaryColor) -> Self {
        match v {
            BinaryColor::On => Inverted::Off,
            BinaryColor::Off => Inverted::On,
        }
    }
}

// (draw_target.color_converted::<Inverted>()).color_converted::<BinaryColor>()

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    wasm_logger::init(wasm_logger::Config::default());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let mut device = device::Device::new(
        document.get_element_by_id("backlight").unwrap(),
        document.get_element_by_id("body").unwrap(),
        document.get_element_by_id("display").unwrap(),
        "cancel",
        "select",
        "up",
        "down",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "asterisk",
        "zero",
        "hash",
        document.get_element_by_id("svg1").unwrap(),
    );

    let mut ringtones = ringtones::Ringtones::new();
    let _ = ringtones.run(&mut device).await;
}

use core::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{JsCast, closure::Closure};

struct DomB {
    was_clicked: bool,
    was_unclicked: bool,
}

#[derive(PartialEq)]
pub enum Event {
    Up,
    Down,
}

impl DomB {
    #[allow(clippy::too_many_arguments)]
    fn new(id: &'static str) -> Rc<RefCell<Self>> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let s = Self {
            was_clicked: false,
            was_unclicked: false,
        };
        let r = Rc::new(RefCell::new(s));
        let g = r.clone();
        let h = r.clone();

        let mouse_down_closure =
            Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
                (*g).borrow_mut().was_clicked = true;
            });
        let mouse_up_closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            (*h).borrow_mut().was_unclicked = true;
        });

        document
            .get_element_by_id(id)
            .unwrap()
            .add_event_listener_with_callback(
                "mousedown",
                mouse_down_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        document
            .get_element_by_id(id)
            .unwrap()
            .add_event_listener_with_callback("mouseup", mouse_up_closure.as_ref().unchecked_ref())
            .unwrap();

        mouse_down_closure.forget();
        mouse_up_closure.forget();

        r
    }

    fn check(&mut self) -> Option<Event> {
        let clicked_result = self.was_clicked;
        let unclicked_result = self.was_unclicked;
        self.was_clicked = false;
        self.was_unclicked = false;
        if clicked_result {
            Some(Event::Down)
        } else if unclicked_result {
            Some(Event::Up)
        } else {
            None
        }
    }
}
