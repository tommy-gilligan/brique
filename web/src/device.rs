use std::{cell::RefCell, rc::Rc, sync::Mutex};

use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, Element, GainNode, OscillatorNode, OscillatorType};

use crate::DomB;

mod backlight;
mod buzzer;
mod display;
mod keypad;
mod rtc;
mod vibration_motor;

pub struct Device {
    backlight_element: Element,
    buzzer_element: Element,
    display: WebSimulatorDisplay<embedded_graphics::pixelcolor::BinaryColor>,
    cancel: Rc<RefCell<DomB>>,
    select: Rc<RefCell<DomB>>,
    up: Rc<RefCell<DomB>>,
    down: Rc<RefCell<DomB>>,
    one: Rc<RefCell<DomB>>,
    two: Rc<RefCell<DomB>>,
    three: Rc<RefCell<DomB>>,
    four: Rc<RefCell<DomB>>,
    five: Rc<RefCell<DomB>>,
    six: Rc<RefCell<DomB>>,
    seven: Rc<RefCell<DomB>>,
    eight: Rc<RefCell<DomB>>,
    nine: Rc<RefCell<DomB>>,
    asterisk: Rc<RefCell<DomB>>,
    zero: Rc<RefCell<DomB>>,
    hash: Rc<RefCell<DomB>>,
    vibration_element: Element,
    oscillator: Rc<Mutex<Option<OscillatorNode>>>,
    gain: Rc<Mutex<Option<GainNode>>>,
    closure: RefCell<Option<Closure<dyn FnMut()>>>,
    offset: i64,
}

impl Device {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        backlight_element: Element,
        buzzer_element: Element,
        display_element: Element,
        cancel_id: &'static str,
        select_id: &'static str,
        up_id: &'static str,
        down_id: &'static str,
        one_id: &'static str,
        two_id: &'static str,
        three_id: &'static str,
        four_id: &'static str,
        five_id: &'static str,
        six_id: &'static str,
        seven_id: &'static str,
        eight_id: &'static str,
        nine_id: &'static str,
        asterisk_id: &'static str,
        zero_id: &'static str,
        hash_id: &'static str,
        vibration_element: Element,
    ) -> Self {
        let output_settings = OutputSettingsBuilder::new()
            .scale(1)
            .pixel_spacing(0)
            .alpha_color(embedded_graphics::pixelcolor::BinaryColor::On)
            .build();

        let display: WebSimulatorDisplay<embedded_graphics::pixelcolor::BinaryColor> =
            WebSimulatorDisplay::new((84, 48), &output_settings, Some(&display_element));

        let result = Self {
            display,
            backlight_element,
            buzzer_element,
            oscillator: Rc::new(Mutex::new(None)),
            gain: Rc::new(Mutex::new(None)),
            closure: RefCell::new(None),
            cancel: crate::DomB::new(cancel_id),
            select: crate::DomB::new(select_id),
            up: crate::DomB::new(up_id),
            down: crate::DomB::new(down_id),
            one: crate::DomB::new(one_id),
            two: crate::DomB::new(two_id),
            three: crate::DomB::new(three_id),
            four: crate::DomB::new(four_id),
            five: crate::DomB::new(five_id),
            six: crate::DomB::new(six_id),
            seven: crate::DomB::new(seven_id),
            eight: crate::DomB::new(eight_id),
            nine: crate::DomB::new(nine_id),
            asterisk: crate::DomB::new(asterisk_id),
            zero: crate::DomB::new(zero_id),
            hash: crate::DomB::new(hash_id),
            vibration_element,
            offset: 0,
        };

        let o = Rc::clone(&result.oscillator);
        let g = Rc::clone(&result.gain);

        result
            .closure
            .borrow_mut()
            .replace(Closure::<dyn FnMut()>::new(move || {
                let mut ox = o.lock().unwrap();
                if ox.is_none() {
                    let audio_context = AudioContext::new().unwrap();
                    let oscillator = audio_context.create_oscillator().unwrap();
                    let gain = audio_context.create_gain().unwrap();

                    oscillator.set_type(OscillatorType::Sine);
                    oscillator.connect_with_audio_node(&gain).unwrap();
                    gain.connect_with_audio_node(&audio_context.destination())
                        .unwrap();
                    ox.replace(oscillator);
                    g.lock().unwrap().replace(gain);
                }
            }));

        result
            .buzzer_element
            .add_event_listener_with_callback(
                "click",
                result
                    .closure
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap();

        result
    }
}
impl shared::Device for Device {}
