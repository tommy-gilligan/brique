use embassy_rp::{
    Peripheral,
    gpio::{Input, Pin, Pull},
    peripherals::{
        PIN_4, PIN_5, PIN_6, PIN_7, PIN_8, PIN_9, PIN_10, PIN_11, PIN_12, PIN_13, PIN_14, PIN_16,
        PIN_17, PIN_18, PIN_19, PIN_20,
    },
};
use embassy_time::Timer;
use shared::Keypad;

struct Button<'a>(Input<'a>, bool);

enum ButtonEvent {
    Up,
    Down,
}

impl<'a> Button<'a> {
    fn new(pin: impl Peripheral<P = impl Pin> + 'a) -> Self {
        Self(Input::new(pin, Pull::Up), false)
    }

    async fn event(&mut self) -> ButtonEvent {
        if self.1 {
            self.0.wait_for_high().await;
            self.1 = false;
            ButtonEvent::Up
        } else {
            self.0.wait_for_low().await;
            self.1 = true;
            ButtonEvent::Down
        }
    }
}

pub struct ContactKeypad<'a> {
    cancel: Button<'a>,
    select: Button<'a>,
    up: Button<'a>,
    down: Button<'a>,
    one: Button<'a>,
    two: Button<'a>,
    three: Button<'a>,
    four: Button<'a>,
    five: Button<'a>,
    six: Button<'a>,
    seven: Button<'a>,
    eight: Button<'a>,
    nine: Button<'a>,
    asterisk: Button<'a>,
    zero: Button<'a>,
    hash: Button<'a>,
}

impl ContactKeypad<'_> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cancel: PIN_16,
        select: PIN_12,
        up: PIN_9,
        down: PIN_8,
        one: PIN_17,
        two: PIN_13,
        three: PIN_7,
        four: PIN_18,
        five: PIN_14,
        six: PIN_6,
        seven: PIN_19,
        eight: PIN_11,
        nine: PIN_5,
        asterisk: PIN_20,
        zero: PIN_10,
        hash: PIN_4,
    ) -> Self {
        Self {
            cancel: Button::new(cancel),
            select: Button::new(select),
            up: Button::new(up),
            down: Button::new(down),
            one: Button::new(one),
            two: Button::new(two),
            three: Button::new(three),
            four: Button::new(four),
            five: Button::new(five),
            six: Button::new(six),
            seven: Button::new(seven),
            eight: Button::new(eight),
            nine: Button::new(nine),
            asterisk: Button::new(asterisk),
            zero: Button::new(zero),
            hash: Button::new(hash),
        }
    }
}

unsafe impl Send for ContactKeypad<'_> {}

impl Keypad for ContactKeypad<'_> {
    async fn event(&mut self) -> shared::KeyEvent {
        Timer::after_millis(30).await;
        match embassy_futures::select::select_array([
            self.cancel.event(),
            self.select.event(),
            self.up.event(),
            self.down.event(),
            self.one.event(),
            self.two.event(),
            self.three.event(),
            self.four.event(),
            self.five.event(),
            self.six.event(),
            self.seven.event(),
            self.eight.event(),
            self.nine.event(),
            self.asterisk.event(),
            self.zero.event(),
            self.hash.event(),
        ])
        .await
        {
            (ButtonEvent::Up, 0) => shared::KeyEvent::Up(shared::Key::Cancel),
            (ButtonEvent::Up, 1) => shared::KeyEvent::Up(shared::Key::Select),
            (ButtonEvent::Up, 2) => shared::KeyEvent::Up(shared::Key::Up),
            (ButtonEvent::Up, 3) => shared::KeyEvent::Up(shared::Key::Down),
            (ButtonEvent::Up, 4) => shared::KeyEvent::Up(shared::Key::One),
            (ButtonEvent::Up, 5) => shared::KeyEvent::Up(shared::Key::Two),
            (ButtonEvent::Up, 6) => shared::KeyEvent::Up(shared::Key::Three),
            (ButtonEvent::Up, 7) => shared::KeyEvent::Up(shared::Key::Four),
            (ButtonEvent::Up, 8) => shared::KeyEvent::Up(shared::Key::Five),
            (ButtonEvent::Up, 9) => shared::KeyEvent::Up(shared::Key::Six),
            (ButtonEvent::Up, 10) => shared::KeyEvent::Up(shared::Key::Seven),
            (ButtonEvent::Up, 11) => shared::KeyEvent::Up(shared::Key::Eight),
            (ButtonEvent::Up, 12) => shared::KeyEvent::Up(shared::Key::Nine),
            (ButtonEvent::Up, 13) => shared::KeyEvent::Up(shared::Key::Asterisk),
            (ButtonEvent::Up, 14) => shared::KeyEvent::Up(shared::Key::Zero),
            (ButtonEvent::Up, 15) => shared::KeyEvent::Up(shared::Key::Hash),
            (ButtonEvent::Down, 0) => shared::KeyEvent::Down(shared::Key::Cancel),
            (ButtonEvent::Down, 1) => shared::KeyEvent::Down(shared::Key::Select),
            (ButtonEvent::Down, 2) => shared::KeyEvent::Down(shared::Key::Up),
            (ButtonEvent::Down, 3) => shared::KeyEvent::Down(shared::Key::Down),
            (ButtonEvent::Down, 4) => shared::KeyEvent::Down(shared::Key::One),
            (ButtonEvent::Down, 5) => shared::KeyEvent::Down(shared::Key::Two),
            (ButtonEvent::Down, 6) => shared::KeyEvent::Down(shared::Key::Three),
            (ButtonEvent::Down, 7) => shared::KeyEvent::Down(shared::Key::Four),
            (ButtonEvent::Down, 8) => shared::KeyEvent::Down(shared::Key::Five),
            (ButtonEvent::Down, 9) => shared::KeyEvent::Down(shared::Key::Six),
            (ButtonEvent::Down, 10) => shared::KeyEvent::Down(shared::Key::Seven),
            (ButtonEvent::Down, 11) => shared::KeyEvent::Down(shared::Key::Eight),
            (ButtonEvent::Down, 12) => shared::KeyEvent::Down(shared::Key::Nine),
            (ButtonEvent::Down, 13) => shared::KeyEvent::Down(shared::Key::Asterisk),
            (ButtonEvent::Down, 14) => shared::KeyEvent::Down(shared::Key::Zero),
            (ButtonEvent::Down, 15) => shared::KeyEvent::Down(shared::Key::Hash),
            _ => {
                unimplemented!()
            }
        }
    }
}
