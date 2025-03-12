#![no_std]
#![no_main]

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 6] = [
    embassy_rp::binary_info::rp_program_name!(c"rp235x-nokia-3310"),
    // in repo root: find pcb -type f \( -exec sha1sum "$PWD"/{} \; \) | awk '{print $1}' | sort | sha1sum | cut -b-10
    embassy_rp::binary_info::rp_pico_board!(c"rp235x-nokia-3310-5da11fc30e"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico on board LED, connected to gpio 25"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
    embassy_rp::binary_info::rp_program_url!(
        c"https://github.com/tommy-gilligan/rp235x-nokia-3310"
    ),
];

use core::cell::RefCell;

use assign_resources::assign_resources;
use defmt::unwrap;
use defmt_rtt as _;
use embassy_executor::{Executor, Spawner};
use embassy_rp::{
    bind_interrupts,
    block::ImageDef,
    multicore::{Stack, spawn_core1},
    peripherals,
    peripherals::USB,
    spi,
    spi::Spi,
    usb::InterruptHandler,
};
use embassy_sync::blocking_mutex::{Mutex, raw::NoopRawMutex};
use panic_probe as _;
use static_cell::StaticCell;

mod backlight;
mod button;
mod buzzer;
mod device;
mod display;
mod keypad;
mod rtc;
mod usb;
mod vibration_motor;

assign_resources! {
    usbs: Usbs{
        usb: USB,
    }
}

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let r = split_resources!(p);
    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| unwrap!(spawner.spawn(usb::big_usb_task(spawner, r.usbs))));
        },
    );

    let mut power = button::Button::new(p.PIN_28);

    let mut vibration_motor = vibration_motor::Motor::new(p.PIN_2);
    let mut buzzer = buzzer::Beeper::new(p.PWM_SLICE2, p.PIN_21);
    let mut clock = rtc::Clock::new(p.I2C1, p.PIN_46, p.PIN_47);

    let mut display_config = spi::Config::default();
    display_config.frequency = 4_000_000;
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(Spi::new_blocking(
        p.SPI0,
        p.PIN_38,
        p.PIN_39,
        p.PIN_32,
        display_config,
    )));
    let mut display = display::Display::new(&spi_bus, p.PIN_37, p.PIN_36, p.PIN_33);

    let mut keypad = keypad::ContactKeypad::new(
        p.PIN_16, p.PIN_12, p.PIN_9, p.PIN_8, p.PIN_17, p.PIN_13, p.PIN_7, p.PIN_18, p.PIN_14,
        p.PIN_6, p.PIN_19, p.PIN_11, p.PIN_5, p.PIN_20, p.PIN_10, p.PIN_4,
    );

    let mut backlight = backlight::Light::new(p.PIN_15);

    let items = ["Clock", "Hardware Test", "Keyboard", "Reboot to USB"];
    let mut menu = shared::menu::Menu::new(&items);

    loop {
        let result = match menu.process(&mut keypad, &mut display).await {
            0 => {
                let clock_app = clock::Clock;
                shared::run_app(
                    clock_app,
                    &mut vibration_motor,
                    &mut buzzer,
                    &mut display,
                    &mut keypad,
                    &mut clock,
                    &mut backlight,
                    &mut power,
                    usb::RX_CHANNEL.try_receive().ok(),
                )
                .await
            }
            1 => {
                let hardware_test = hardware_test::HardwareTest::default();
                shared::run_app(
                    hardware_test,
                    &mut vibration_motor,
                    &mut buzzer,
                    &mut display,
                    &mut keypad,
                    &mut clock,
                    &mut backlight,
                    &mut power,
                    usb::RX_CHANNEL.try_receive().ok(),
                )
                .await
            }
            2 => {
                let keyboard = keyboard::Keyboard;
                shared::run_app(
                    keyboard,
                    &mut vibration_motor,
                    &mut buzzer,
                    &mut display,
                    &mut keypad,
                    &mut clock,
                    &mut backlight,
                    &mut power,
                    usb::RX_CHANNEL.try_receive().ok(),
                )
                .await
            }
            _ => {
                let reset = reset_to_boot::ResetToBoot::default();
                shared::run_app(
                    reset,
                    &mut vibration_motor,
                    &mut buzzer,
                    &mut display,
                    &mut keypad,
                    &mut clock,
                    &mut backlight,
                    &mut power,
                    usb::RX_CHANNEL.try_receive().ok(),
                )
                .await
            }
        };

        match result {
            Some(shared::SystemRequest::UsbTx(shared::UsbTx::HidChar(c))) => {
                usb::HID_TX_CHANNEL.send(c).await
            }
            Some(shared::SystemRequest::UsbTx(shared::UsbTx::CdcBuffer(b))) => {
                usb::CDC_TX_CHANNEL.send(b).await
            }
            Some(shared::SystemRequest::ResetToBoot) => {
                embassy_rp::rom_data::reset_to_usb_boot(0, 0);
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
