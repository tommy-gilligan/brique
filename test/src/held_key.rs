#[cfg(test)]
mod test {
    use futures_executor::block_on;

    #[test]
    fn test_held_key() {
        let f = async {
            let driver = embassy_time::MockDriver::get();
            driver.reset();
            let mut keypad =
                crate::keypad::Keypad::new(&[shared::KeyEvent::Down(shared::Key::Two)]);
            let mut held_key = shared::held_key::HeldKey::new(500, 100);

            assert_eq!(
                held_key.event(&mut keypad).await,
                Some(shared::held_key::Event::Down(shared::Key::Two))
            );
            driver.advance(embassy_time::Duration::from_millis(100));
            keypad.pending();
            // held_key.event(&mut keypad).await;

            // driver.advance(embassy_time::Duration::from_millis(401));
            // assert_eq!(
            //     held_key.event(&mut keypad).await,
            //     None
            // );
            // assert_eq!(
            //     held_key.event(&mut keypad).await,
            //     Some(shared::held_key::Event::Delay(shared::Key::Two))
            // );
        };
        block_on(f);
    }
}
