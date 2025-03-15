use core::cell::RefCell;

use display_interface_spi::SPIInterface;
use embassy_rp::{
    gpio::{Level, Output},
    peripherals::{PIN_33, PIN_36, PIN_37, SPI0},
    spi,
};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Delay;
use embedded_graphics_core::{
    Pixel,
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget},
    primitives::Rectangle,
};

type SpiDeviceWithConfig<'a> = embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<
    'a,
    NoopRawMutex,
    embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>,
    Output<'a>,
>;

pub struct Display<'a>(
    pcd8544::Driver<
        SPIInterface<SpiDeviceWithConfig<'a>, Output<'a>>,
        Output<'a>,
        core::convert::Infallible,
    >,
);

impl<'a> Display<'a> {
    pub fn new(
        spi_bus: &'a embassy_sync::blocking_mutex::Mutex<
            NoopRawMutex,
            RefCell<embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>>,
        >,
        thirty_seven: PIN_37,
        thirty_six: PIN_36,
        thirty_three: PIN_33,
    ) -> Self {
        let mut display_config = spi::Config::default();
        display_config.frequency = 4_000_000;

        let display_spi = SpiDeviceWithConfig::new(
            spi_bus,
            Output::new(thirty_seven, Level::High),
            display_config,
        );

        let mut pcd8544: pcd8544::Driver<
            SPIInterface<SpiDeviceWithConfig<'a>, Output<'a>>,
            Output<'a>,
            core::convert::Infallible,
        > = pcd8544::Driver::new(
            SPIInterface::new(display_spi, Output::new(thirty_six, Level::High)),
            Output::new(thirty_three, Level::High),
        );

        pcd8544.init(&mut Delay).unwrap();
        pcd8544.set_contrast(64).unwrap();
        pcd8544.invert_display(true).unwrap();
        pcd8544.clear(BinaryColor::Off).unwrap();

        Self(pcd8544)
    }
}

impl<'a> DrawTarget for Display<'a> {
    type Color = BinaryColor;

    type Error = <pcd8544::Driver<
        SPIInterface<SpiDeviceWithConfig<'a>, Output<'a>>,
        Output<'a>,
        core::convert::Infallible,
    > as DrawTarget>::Error;

    fn draw_iter<I: IntoIterator<Item = Pixel<<Self as DrawTarget>::Color>>>(
        &mut self,
        i: I,
    ) -> Result<(), <Self as DrawTarget>::Error> {
        let a = self.0.draw_iter(i);
        let _ = self.0.flush();
        a
    }
}

impl Dimensions for Display<'_> {
    fn bounding_box(&self) -> Rectangle {
        self.0.bounding_box()
    }
}
