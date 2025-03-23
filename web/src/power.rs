pub struct DomPower;

impl DomPower {
    pub fn new() -> Self {
        Self
    }
}

unsafe impl Send for DomPower {}

impl shared::PowerButton for DomPower {
    fn clear(&mut self) {
        let window = web_sys::window().expect("no global `window` exists");
        let location = window.location();
        location.reload().unwrap();
    }
}
