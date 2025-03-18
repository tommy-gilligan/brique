use embassy_executor::Spawner;

use crate::{Irqs, Usbs, Flashs};

mod usb;
mod flash;

#[embassy_executor::task]
pub async fn background(spawner: Spawner, usb: Usbs, flash: Flashs) {
    spawner.spawn(usb::usb_task(spawner, usb)).unwrap();
    spawner.spawn(flash::flash_task(spawner, flash)).unwrap();
}
