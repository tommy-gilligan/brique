use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::join::{join, join4};
use embassy_rp::usb::Driver;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_usb::{
    class::{
        cdc_acm, hid,
        hid::{HidReaderWriter, ReportId, RequestHandler},
    },
    control::OutResponse,
    driver::EndpointError,
};
use static_cell::StaticCell;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

use crate::{Irqs, Usbs};

pub static RX_CHANNEL: Channel<CriticalSectionRawMutex, shared::UsbRx, 10> = Channel::new();
pub static HID_TX_CHANNEL: Channel<CriticalSectionRawMutex, char, 10> = Channel::new();
pub static CDC_TX_CHANNEL: Channel<CriticalSectionRawMutex, [u8; 64], 10> = Channel::new();

#[embassy_executor::task]
pub async fn big_usb_task(_spawner: Spawner, usbs: Usbs) {
    let driver = Driver::new(usbs.usb, Irqs);
    let config = {
        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
        config.manufacturer = Some("Embassy");
        config.product = Some("USB-serial example");
        config.serial_number = Some("12345678");
        config.max_power = 100;
        config.max_packet_size_0 = 64;
        config
    };
    let mut builder = {
        static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

        embassy_usb::Builder::new(
            driver,
            config,
            CONFIG_DESCRIPTOR.init([0; 256]),
            BOS_DESCRIPTOR.init([0; 256]),
            &mut [], // no msos descriptors
            CONTROL_BUF.init([0; 64]),
        )
    };
    let mut class = {
        static STATE: StaticCell<cdc_acm::State> = StaticCell::new();
        let state = STATE.init(cdc_acm::State::new());
        cdc_acm::CdcAcmClass::new(&mut builder, state, 64)
    };

    let logger_class = {
        static LOGGER_STATE: StaticCell<cdc_acm::State> = StaticCell::new();
        let logger_state = LOGGER_STATE.init(cdc_acm::State::new());
        cdc_acm::CdcAcmClass::new(&mut builder, logger_state, 64)
    };

    let (reader, mut writer) = {
        static HID_STATE: StaticCell<hid::State> = StaticCell::new();
        let hid_state: &'static mut hid::State = HID_STATE.init(hid::State::new());
        let config = embassy_usb::class::hid::Config {
            report_descriptor: KeyboardReport::desc(),
            request_handler: None,
            poll_ms: 60,
            max_packet_size: 64,
        };
        let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, hid_state, config);
        hid.split()
    };

    let log_fut = embassy_usb_logger::with_class!(1024, log::LevelFilter::Info, logger_class);

    let mut usb = builder.build();
    let usb_fut = usb.run();

    let echo_fut = async {
        class.wait_connection().await;
        let mut buf = [0; 64];
        loop {
            let n = class.read_packet(&mut buf).await.unwrap();
            RX_CHANNEL.send(buf).await;
            class
                .write_packet(&CDC_TX_CHANNEL.receive().await[..n])
                .await
                .unwrap();
        }
    };

    let in_fut = async {
        loop {
            let _c = HID_TX_CHANNEL.receive().await;
            let report = KeyboardReport {
                keycodes: [4, 0, 0, 0, 0, 0],
                leds: 0,
                modifier: 0,
                reserved: 0,
            };
            match writer.write_serialize(&report).await {
                Ok(()) => {}
                Err(e) => log::warn!("Failed to send report: {:?}", e),
            };
            let report = KeyboardReport {
                keycodes: [0, 0, 0, 0, 0, 0],
                leds: 0,
                modifier: 0,
                reserved: 0,
            };
            match writer.write_serialize(&report).await {
                Ok(()) => {}
                Err(e) => log::warn!("Failed to send report: {:?}", e),
            };
        }
    };
    let mut request_handler = MyRequestHandler {};
    let out_fut = async {
        reader.run(false, &mut request_handler).await;
    };
    join4(usb_fut, echo_fut, log_fut, join(in_fut, out_fut)).await;
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => defmt::panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}
