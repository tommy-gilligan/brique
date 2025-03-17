#![no_std]
#![feature(ascii_char_variants)]
#![feature(ascii_char)]

use shared::Application;

pub struct Keyboard<'a>(shared::textbox::Textbox<'a>);

use core::fmt::Debug;

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};

impl<'a> Keyboard<'a> {
    pub fn new<D: DrawTarget<Color = BinaryColor>>(
        draw_target: &mut D,
        buffer: &'a mut [u8],
    ) -> Self
    where
        <D as DrawTarget>::Error: Debug,
    {
        Self(shared::textbox::Textbox::new(draw_target, buffer))
    }
}

impl Application for Keyboard<'_> {
    async fn run(
        &mut self,
        device: &mut impl shared::Device,
        _system_response: Option<[u8; 64]>,
    ) -> Result<Option<shared::SystemRequest>, ()> {
        Ok(
            self.0
                .process(device)
                .await
                .map(|c| shared::SystemRequest::UsbTx(shared::UsbTx::HidChar(shared::build_report(c))))
        )
    }
}
