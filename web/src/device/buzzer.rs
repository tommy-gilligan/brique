use std::rc::Rc;

impl shared::Buzzer for super::Device {
    type Error = ();

    fn mute_buzzer(&mut self) -> Result<(), ()> {
        if !self.mute {
            self.mute = true;
            self.set_volume(0);
        }
        Ok(())
    }

    fn unmute_buzzer(&mut self) -> Result<(), ()> {
        if self.mute {
            self.set_volume(100);
            self.mute = false;
            let binding = Rc::clone(&self.oscillator);
            if let Ok(mut oscillator) = binding.lock() {
                if let Some(o) = oscillator.as_mut() {
                    let _ = o.start();
                }
            }
        }
        Ok(())
    }

    fn set_frequency(&mut self, frequency: u16) -> Result<(), ()> {
        if !self.mute {
            let binding = Rc::clone(&self.oscillator);

            if let Ok(mut oscillator) = binding.lock() {
                if let Some(o) = oscillator.as_mut() {
                    o.frequency().set_value(frequency as f32);
                }
            }
        }
        Ok(())
    }

    fn set_volume(&mut self, volume: u8) {
        let binding = Rc::clone(&self.gain);

        if let Ok(mut gain) = binding.lock() {
            if let Some(g) = gain.as_mut() {
                g.gain().set_value(volume as f32 / 100.0);
            }
        }
    }
}
