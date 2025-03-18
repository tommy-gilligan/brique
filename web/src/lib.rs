#![allow(unexpected_cfgs)]
mod device;
mod power;
mod system_request_handler;

use embassy_executor::Spawner;
use web_sys::Element;

pub struct CdcSend {
    rx: Rc<RefCell<Option<[u8; 64]>>>,
}

impl CdcSend {
    fn new(element: Element) -> Self {
        let rx = Rc::new(RefCell::new(None));
        let l = rx.clone();
        let change_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
            let input = event
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
            let mut buffer: [u8; 64] = [0; 64];
            for (a, b) in buffer.iter_mut().zip(input.value().as_bytes()) {
                *a = *b;
            }
            input.set_value("");

            *(*l).borrow_mut() = Some(buffer);
        });

        element
            .add_event_listener_with_callback("change", change_closure.as_ref().unchecked_ref())
            .unwrap();

        change_closure.forget();

        Self { rx }
    }
}

impl shared::SystemResponse for CdcSend {
    fn take(&mut self) -> Option<[u8; 64]> {
        let l = self.rx.clone();
        (*l).borrow_mut().take()
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    wasm_logger::init(wasm_logger::Config::default());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let device = device::Device::new(
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

    let power_button = power::DomPower::new("power");
    let hid_console = document.get_element_by_id("hid-console").unwrap();
    let cdc_console = document.get_element_by_id("cdc-console").unwrap();
    let system_request_handler = system_request_handler::Handler::new(hid_console, cdc_console);

    let system_response = CdcSend::new(document.get_element_by_id("cdc-console-rx").unwrap());

    main_menu::main_menu(device, power_button, system_response, system_request_handler).await
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
