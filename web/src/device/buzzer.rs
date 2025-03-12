use std::rc::Rc;

use wasm_bindgen::prelude::*;

impl shared::Buzzer for super::Device {
    fn mute(&mut self) {
        let binding = Rc::clone(&self.oscillator);
        let mut oscillator = binding.lock().unwrap();
        if let Some(o) = oscillator.as_mut() {
            match o.stop() {
                Ok(()) => {}
                Err(e) => {
                    let dom_exception: Option<&web_sys::DomException> = e.as_ref().dyn_ref();
                    if dom_exception.unwrap().code() != web_sys::DomException::INVALID_STATE_ERR {
                        // panic!("HEY");
                    }
                }
            }
        }
    }

    fn unmute(&mut self) {
        let binding = Rc::clone(&self.oscillator);
        let mut oscillator = binding.lock().unwrap();
        if let Some(o) = oscillator.as_mut() {
            match o.start() {
                Ok(()) => {}
                Err(e) => {
                    let dom_exception: Option<&web_sys::DomException> = e.as_ref().dyn_ref();
                    if dom_exception.unwrap().code() != web_sys::DomException::INVALID_STATE_ERR {
                        // panic!("HEY");
                    }
                }
            }
        }
    }

    fn set_frequency(&mut self, frequency: u16) {
        let binding = Rc::clone(&self.oscillator);
        let mut oscillator = binding.lock().unwrap();
        if let Some(o) = oscillator.as_mut() {
            o.frequency().set_value(frequency as f32);
        }
    }

    fn set_volume(&mut self, volume: u8) {
        let binding = Rc::clone(&self.gain);
        let mut gain = binding.lock().unwrap();
        if let Some(g) = gain.as_mut() {
            g.gain().set_value(volume as f32 / 100.0);
        }
    }
}
