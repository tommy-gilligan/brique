use web_sys::Element;
pub struct Handler {
    hid_console: Element
}

impl Handler {
    pub fn new(hid_console: Element) -> Self {
        Self {
            hid_console
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
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
