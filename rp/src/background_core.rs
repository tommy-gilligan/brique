use embassy_executor::Spawner;

use crate::Usbs;

mod usb;

#[embassy_executor::task]
pub async fn background(spawner: Spawner, usb: Usbs) {
    spawner.spawn(usb::usb_task(spawner, usb)).unwrap();
}
