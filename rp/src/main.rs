#![no_std]
#![no_main]

use core::cell::RefCell;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    block::{
        Link, Partition, PartitionFlag, PartitionTableBlock, Permission, UnpartitionedFlag,
        UnpartitionedSpace,
    },
    peripherals::USB,
    spi,
    spi::Spi,
    usb::InterruptHandler,
};
use embassy_sync::blocking_mutex::Mutex;
use panic_probe as _;

#[unsafe(link_section = ".start_block")]
#[used]
pub static PARTITION_TABLE: PartitionTableBlock = PartitionTableBlock::new()
    .add_partition_item(
        UnpartitionedSpace::new()
            .with_permission(Permission::SecureRead)
            .with_permission(Permission::SecureWrite)
            .with_permission(Permission::NonSecureRead)
            .with_permission(Permission::NonSecureWrite)
            .with_permission(Permission::BootRead)
            .with_permission(Permission::BootWrite)
            .with_flag(UnpartitionedFlag::AcceptsDefaultFamilyAbsolute),
        &[
            Partition::new(2, 512)
                .with_id(0)
                .with_flag(PartitionFlag::AcceptsDefaultFamilyRp2350ArmS)
                .with_flag(PartitionFlag::AcceptsDefaultFamilyRp2350Riscv)
                .with_permission(Permission::SecureRead)
                .with_permission(Permission::SecureWrite)
                .with_permission(Permission::NonSecureRead)
                .with_permission(Permission::NonSecureWrite)
                .with_permission(Permission::BootRead)
                .with_permission(Permission::BootWrite)
                .with_name("A"),
            Partition::new(513, 1023)
                .with_id(1)
                .with_flag(PartitionFlag::AcceptsDefaultFamilyRp2350ArmS)
                .with_flag(PartitionFlag::AcceptsDefaultFamilyRp2350Riscv)
                .with_link(Link::ToA { partition_idx: 0 })
                .with_permission(Permission::SecureRead)
                .with_permission(Permission::SecureWrite)
                .with_permission(Permission::NonSecureRead)
                .with_permission(Permission::NonSecureWrite)
                .with_permission(Permission::BootRead)
                .with_permission(Permission::BootWrite)
                .with_name("B"),
        ],
    )
    .with_version(1, 0)
    .with_sha256();

// pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

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

// mod background_core;
mod device;
// mod flash;
mod rtc;

// assign_resources! {
//     usbs: Usbs{
//         usb: USB,
//     },
//     // flashs: Flashs {
//     //     flash: FLASH,
//     //     dma_0: DMA_CH0
//     // }
// }

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

// static mut CORE1_STACK: Stack<4096> = Stack::new();
// static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // let r = split_resources!(p);
    let watchdog = embassy_rp::watchdog::Watchdog::new(p.WATCHDOG);
    // if watchdog.get_scratch(0) == WATCHDOG_MARKER {
    //     defmt::error!("Reset due to watchdog");
    // }
    // watchdog.set_scratch(0, WATCHDOG_MARKER);
    embassy_time::Timer::after_millis(10).await;

    // spawner.spawn(flash::flash_task(spawner, r.flashs)).unwrap();
    // spawn_core1(
    //     p.CORE1,
    //     unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
    //     move || {
    //         let executor1 = EXECUTOR1.init(Executor::new());
    //         executor1.run(|spawner| {
    //             unwrap!(spawner.spawn(background_core::background(spawner, r.usbs)))
    //         });
    //     },
    // );
    // FIX
    let _clock = rtc::Clock::new(p.I2C1, p.PIN_46, p.PIN_47);

    let mut display_config = spi::Config::default();
    display_config.frequency = 4_000_000;

    let display = Mutex::new(RefCell::new(Spi::new_blocking(
        p.SPI0,
        p.PIN_38,
        p.PIN_39,
        p.PIN_32,
        display_config,
    )));
    let _device = device::Device::new(
        watchdog,
        p.PIN_2,
        p.PIN_4,
        p.PIN_5,
        p.PIN_6,
        p.PIN_7,
        p.PIN_8,
        p.PIN_9,
        p.PIN_10,
        p.PIN_11,
        p.PIN_12,
        p.PIN_13,
        p.PIN_14,
        p.PIN_15,
        p.PIN_16,
        p.PIN_17,
        p.PIN_18,
        p.PIN_19,
        p.PIN_20,
        p.PIN_21,
        p.PIN_33,
        p.PIN_36,
        p.PIN_37,
        p.PWM_SLICE2,
        &display,
    )
    .unwrap();
    loop {
        log::info!("looping");
    }
}
