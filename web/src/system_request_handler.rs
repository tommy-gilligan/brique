use web_sys::Element;

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
            shared::SystemRequest::UsbTx(shared::UsbTx::HidChar(c)) => match c.keycodes[0] {
                0 => {
                    self.hid_console
                        .append_with_str_1(&format!("Key Up\tModifier: {:?}\n", c.modifier))
                        .unwrap();
                }
                i => {
                    let report = usbd_hid::descriptor::KeyboardUsage::from(i);
                    self.hid_console
                        .append_with_str_1(&format!(
                            "Key Down: {:?}\tModifier: {:?}\n",
                            report, c.modifier
                        ))
                        .unwrap();
                }
            },
            shared::SystemRequest::UsbTx(shared::UsbTx::CdcBuffer(b)) => {
                let s = core::str::from_utf8(&b);
                self.cdc_console.append_with_str_1(s.unwrap()).unwrap();
            }
            shared::SystemRequest::ResetToBoot => todo!(),
        }
    }
}
