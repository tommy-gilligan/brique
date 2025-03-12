use web_sys::Element;
pub struct Handler(Element);

impl Handler {
    pub fn new(element: Element) -> Self {
        Self(element)
    }
}

impl shared::SystemRequestHandler for Handler {
    async fn handle_request(&mut self, request: shared::SystemRequest) {
        match request {
            shared::SystemRequest::UsbTx(shared::UsbTx::HidChar(c)) => {
                let mut buf: [u8; 10] = [0; 10];
                let _ = ssmarshal::serialize(&mut buf, &c);
                self.0.append_with_str_1(&format!("{:?}\n", &buf)).unwrap();
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
