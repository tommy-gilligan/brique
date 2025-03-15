use web_sys::Element;
use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, closure::Closure};

pub struct Handler {
    hid_console: Element,
    cdc_console: Element,
}

impl Handler {
    pub fn new(hid_console: Element, cdc_console: Element) -> Self {
        Self {
            hid_console,
            cdc_console,

        }
    }
}

impl shared::SystemRequestHandler for Handler {
    async fn handle_request(&mut self, request: shared::SystemRequest) {
        match request {
            shared::SystemRequest::UsbTx(shared::UsbTx::HidChar(c)) => {
                let mut buf: [u8; 10] = [0; 10];
                let _ = ssmarshal::serialize(&mut buf, &c);
                self.hid_console.append_with_str_1(&format!("{:?}\n", &buf)).unwrap();
            },
            shared::SystemRequest::UsbTx(shared::UsbTx::CdcBuffer(b)) => {
                let s = core::str::from_utf8(&b);

                self.cdc_console.append_with_str_1(&s.unwrap()).unwrap();
            }
            shared::SystemRequest::ResetToBoot => todo!()
        }
    }
}
