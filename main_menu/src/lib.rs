#![no_std]

pub async fn main_menu(
    mut device: impl shared::Device,
    mut power: impl shared::PowerButton,
    mut system_response: impl shared::SystemResponse,
    mut system_request_handler: impl shared::SystemRequestHandler,
) {
    loop {
        if let Some(index) = shared::lock_screen::LockScreen::new(&[
            "Ringtones",
            "Clock",
            "Hardware Test",
            "Keyboard",
            "Snake",
            "Reboot to USB",
        ]).get_selection(&mut device).await {
            match index {
                0 => {
                    shared::run_app(
                        ringtones::Ringtones::new(&mut device, &mut [0; 1024]),
                        &mut device,
                        &mut power,
                        &mut system_response,
                        &mut system_request_handler,
                    )
                    .await
                }
                1 => {
                    shared::run_app(clock::Clock, &mut device, &mut power, &mut system_response, &mut system_request_handler)
                        .await
                }
                2 => {
                    shared::run_app(
                        hardware_test::HardwareTest::default(),
                        &mut device,
                        &mut power,
                        &mut system_response,
                        &mut system_request_handler,
                    )
                    .await
                }
                3 => {
                    shared::run_app(
                        keyboard::Keyboard::new(&mut device, &mut [0; 1024]),
                        &mut device,
                        &mut power,
                        &mut system_response,
                        &mut system_request_handler,
                    )
                    .await
                }
                4 => {
                    shared::run_app(snake::Snake::new(0), &mut device, &mut power, &mut system_response, &mut system_request_handler)
                        .await
                }
                _ => {
                    shared::run_app(reset_to_boot::ResetToBoot, &mut device, &mut power, &mut system_response, &mut system_request_handler)
                        .await
                }
            }
        }
    }
}
