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

    let s = heapless::String::<240>::new();
    let mut keyboard = keyboard::Keyboard::new(s);
    loop {
        let _ = keyboard.run(&mut device).await;
    }

    // let mut ringtones = ringtones::Ringtones::new();
    // let _ = ringtones.run(&mut device).await;
}

use core::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{JsCast, closure::Closure};

#[derive(PartialEq)]
pub enum Event {
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
pub enum KeyEvent {
    Up(char),
    Down(char),
}

struct DomB {
    was_clicked: bool,
    was_unclicked: bool,
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

struct DomK {
    was_clicked: Option<char>,
    was_unclicked: Option<char>,
}

impl DomK {
    #[allow(clippy::too_many_arguments)]
    fn new() -> Rc<RefCell<Self>> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body: web_sys::Element = document.body().unwrap().into();

        let s = Self {
            was_clicked: None,
            was_unclicked: None,
        };
        let r = Rc::new(RefCell::new(s));
        let g = r.clone();
        let h = r.clone();

        let mouse_down_closure =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
                let c = match event.key_code() {
                    48 => '0',
                    49 => '1',
                    50 => '2',
                    51 => '3',
                    52 => '4',
                    53 => '5',
                    54 => '6',
                    55 => '7',
                    56 => '8',
                    57 => '9',
                    40 => 'd',
                    38 => 'u',
                    13 => 'E',
                    27 => 'e',
                    _ => '0',
                };
                (*g).borrow_mut().was_clicked = Some(c);
            });
        let mouse_up_closure =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
                let c = match event.key_code() {
                    48 => '0',
                    49 => '1',
                    50 => '2',
                    51 => '3',
                    52 => '4',
                    53 => '5',
                    54 => '6',
                    55 => '7',
                    56 => '8',
                    57 => '9',
                    40 => 'd',
                    38 => 'u',
                    13 => 'E',
                    27 => 'e',
                    _ => '0',
                };
                (*h).borrow_mut().was_unclicked = Some(c);
            });

        body.add_event_listener_with_callback(
            "keydown",
            mouse_down_closure.as_ref().unchecked_ref(),
        )
        .unwrap();

        body.add_event_listener_with_callback("keyup", mouse_up_closure.as_ref().unchecked_ref())
            .unwrap();

        mouse_down_closure.forget();
        mouse_up_closure.forget();

        r
    }

    fn check(&mut self) -> Option<KeyEvent> {
        let clicked_result = self.was_clicked;
        let unclicked_result = self.was_unclicked;
        self.was_clicked = None;
        self.was_unclicked = None;

        if clicked_result.is_some() {
            clicked_result.map(KeyEvent::Down)
        } else if unclicked_result.is_some() {
            unclicked_result.map(KeyEvent::Up)
        } else {
            None
        }
    }
}
