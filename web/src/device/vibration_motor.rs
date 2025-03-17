use shared::VibrationMotor;

impl VibrationMotor for super::Device {
    fn start_vibrating(&mut self) {
        self.vibration_element
            .class_list()
            .add_1("vibrating")
            .unwrap();
    }

    fn stop_vibrating(&mut self) {
        self.vibration_element
            .class_list()
            .remove_1("vibrating")
            .unwrap();
    }
}
