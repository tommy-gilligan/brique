#![no_std]

use shared::Application;

pub struct ResetToBoot;

impl ResetToBoot {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ResetToBoot {
    fn default() -> Self {
        Self
    }
}

impl Application for ResetToBoot {
    async fn run(
        &mut self,
        _device: &mut impl shared::Device,
        _system_response: Option<[u8; 64]>,
    ) -> Option<shared::SystemRequest> {
        None
    }
}
