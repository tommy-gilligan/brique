use shared::VibrationMotor;

impl VibrationMotor for super::Device {
    fn start(&mut self) {
        self.vibration_element
            .class_list()
            .add_1("vibrating")
            .unwrap();
    }

    fn stop(&mut self) {
        self.vibration_element
            .class_list()
            .remove_1("vibrating")
            .unwrap();
    }
}
