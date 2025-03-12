use shared::Backlight;

impl Backlight for super::Device {
    fn on(&mut self) {
        self.backlight_element
            .set_attribute("style", "color: lime")
            .unwrap();
    }

    fn off(&mut self) {
        self.backlight_element
            .set_attribute("style", "color: black")
            .unwrap();
    }
}
