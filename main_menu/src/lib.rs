#![no_std]

pub async fn main_menu(
    mut device: impl shared::Device,
    mut power: impl shared::PowerButton,
    mut cdc_send: impl shared::SystemResponse,
    mut handler: impl shared::SystemRequestHandler,
) {
    let items = [
        "Ringtones",
        "Clock",
        "Hardware Test",
        "Keyboard",
        "Snake",
        "Reboot to USB",
    ];
    let mut lock_screen = shared::lock_screen::LockScreen::new(&items);
    loop {
        if let Some(index) = lock_screen.process(&mut device).await {
            match index {
                0 => {
                    let mut buffer: [u8; 1024] = [0; 1024];
                    let ringtones = ringtones::Ringtones::new(&mut device, &mut buffer);

                    shared::run_app(
                        ringtones,
                        &mut device,
                        &mut power,
                        &mut cdc_send,
                        &mut handler,
                    )
                    .await
                }
                1 => {
                    let clock = clock::Clock;

                    shared::run_app(clock, &mut device, &mut power, &mut cdc_send, &mut handler)
                        .await
                }
                2 => {
                    let hardware_test = hardware_test::HardwareTest::default();

                    shared::run_app(
                        hardware_test,
                        &mut device,
                        &mut power,
                        &mut cdc_send,
                        &mut handler,
                    )
                    .await
                }
                3 => {
                    let mut buffer: [u8; 1024] = [0; 1024];
                    let keyboard = keyboard::Keyboard::new(&mut device, &mut buffer);

                    shared::run_app(
                        keyboard,
                        &mut device,
                        &mut power,
                        &mut cdc_send,
                        &mut handler,
                    )
                    .await
                }
                4 => {
                    let snake = snake::Snake::new(0);

                    shared::run_app(snake, &mut device, &mut power, &mut cdc_send, &mut handler)
                        .await
                }
                _ => {
                    let reset = reset_to_boot::ResetToBoot;

                    shared::run_app(reset, &mut device, &mut power, &mut cdc_send, &mut handler)
                        .await
                }
            }
        }
    }
}
