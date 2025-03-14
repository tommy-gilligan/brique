use core::{ascii::Char, future::Future};

use defmt::Format;
use futures::{future, future::Either, pin_mut};
use core::pin::Pin;
use embassy_time::Timer;
use crate::{Key, KeyEvent};

#[derive(Debug, PartialEq, Format, Copy, Clone)]
pub enum Case {
    Upper,
    Lower,
    Number,
}

#[derive(Debug, PartialEq, Format, Copy, Clone)]
pub enum Event {
    Tentative(Char),
    Decided(Char),
    Case(Case),
}

pub struct MultiTap {
    case: crate::multitap::Case,
    down: Option<Key>,
    timer: Option<Timer>,
    prev_case: crate::multitap::Case,
    last_down: Option<Key>,
    last_tentative: Option<Char>,
    pending: Option<Event>,
    timer: Option<Timer>,
}

impl Default for MultiTap {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiTap {
    pub fn new() -> Self {
        Self {
            case: Case::Lower,
            down: None,
            timer: None,
            prev_case: Case::Lower,
            last_down: None,
            last_tentative: None,
            pending: None
        }
    }

    pub fn case(&self) -> Case {
        self.case
    }

    pub fn enable_numeric_case(&mut self) {
        self.prev_case = self.case;
        self.case = Case::Number;
    }

    pub fn cycle_case(&mut self) {
        self.case = match self.case {
            Case::Number => {
                match self.prev_case {
                    Case::Upper => Case::Lower,
                    Case::Lower => Case::Upper,
                    Case::Number => Case::Upper,
                }
            },
            Case::Upper => Case::Lower,
            Case::Lower => Case::Upper,
        }
    }

    fn set_key_down(&mut self, key: Key) {
        self.timer = Some(embassy_time::Timer::after_millis(1500));
        self.down = Some(key.clone());
        self.last_down = Some(key.clone());
    }

    fn clear_key_down(&mut self) {
        self.timer = None;
        self.down = None;
    }

    fn clear_pending(&mut self) {
        self.pending = None;
    }

    pub fn key_event(&mut self, e: KeyEvent) -> Option<Event> {
        match e {
            crate::KeyEvent::Up(_) => {
                self.clear_key_down();
                None
            },
            crate::KeyEvent::Down(Key::Hash) => {
                self.set_key_down(Key::Hash);
                self.cycle_case();
                Some(Event::Case(self.case))
            },
            crate::KeyEvent::Down(Key::Cancel) => {
                self.clear_key_down();
                Some(Event::Decided(core::ascii::Char::Backspace))
            }
            crate::KeyEvent::Down(d) => {
                if self.case == Case::Number {
                    self.set_key_down(d.clone());
                    Some(Event::Decided(digit(d)))
                } else {
                    match self.last_down.clone() {
                        Some(p) if p == d => {
                            self.set_key_down(d.clone());
                            self.last_tentative = Some(next_char(self.last_tentative.unwrap()));
                            Some(Event::Tentative(self.last_tentative.unwrap()))
                        }
                        Some(p) => {
                            self.set_key_down(d.clone());
                            let d: Char = match self.case {
                                Case::Upper => d.clone().into(),
                                _ => lowercase(d.clone().into())
                            };
                            let result = self.last_tentative.unwrap();
                            self.last_tentative = Some(d);
                            self.pending = Some(Event::Tentative(d));
                            Some(Event::Decided(result))
                        }
                        None => {
                            self.set_key_down(d.clone());
                            let d: Char = match self.case {
                                Case::Upper => d.clone().into(),
                                _ => lowercase(d.clone().into())
                            };
                            self.last_tentative = Some(d.clone().into());
                            Some(Event::Tentative(d.into()))
                        }
                    }
                }
            }
            _ => None
        }
    }

    pub fn timeout_event(&mut self) -> Option<Event> {
        match self.down.clone() {
            Some(Key::Hash) => {
                self.enable_numeric_case();
                self.clear_key_down();
                Some(Event::Case(self.case))
            }
            Some(d) => {
                self.clear_key_down();
                self.clear_pending();
                self.last_tentative = None;
                self.last_down = None;
                Some(Event::Decided(digit(d)))
            }
            None => {
                let result = self.last_tentative.unwrap();
                Some(Event::Decided(result))
            }
        }
    }

    pub async fn event<KEYPAD>(
        &mut self,
        keypad: &mut KEYPAD,
    ) -> Option<Event> where KEYPAD: crate::Keypad, {
        // if something has just been decided
        // still emit the next tentative
        if let Some(pending) = self.pending {
            self.pending = None;
            // self.last_emitted = Some(pending);
            return Some(pending);
        }

        let event_future = keypad.event();
        pin_mut!(event_future);

        if let Some(timer) = &mut self.timer {
            match future::select(timer, event_future).await {
                Either::Left((..)) => self.timeout_event(),
                Either::Right((e, _)) => self.key_event(e)
                
            }
        } else {
            self.key_event(event_future.await)
        }
    }
}

fn digit(k: crate::Key) -> Char {
    match k {
        crate::Key::One => core::ascii::Char::Digit1,
        crate::Key::Two => core::ascii::Char::Digit2,
        crate::Key::Three => core::ascii::Char::Digit3,
        crate::Key::Four => core::ascii::Char::Digit4,
        crate::Key::Five => core::ascii::Char::Digit5,
        crate::Key::Six => core::ascii::Char::Digit6,
        crate::Key::Seven => core::ascii::Char::Digit7,
        crate::Key::Eight => core::ascii::Char::Digit8,
        crate::Key::Nine => core::ascii::Char::Digit9,
        crate::Key::Zero => core::ascii::Char::Digit0,
        _ => core::ascii::Char::Digit0,
    }
}

fn lowercase(c: Char) -> Char {
    match c {
        core::ascii::Char::CapitalA => core::ascii::Char::SmallA,
        core::ascii::Char::CapitalB => core::ascii::Char::SmallB,
        core::ascii::Char::CapitalC => core::ascii::Char::SmallC,
        core::ascii::Char::CapitalD => core::ascii::Char::SmallD,
        core::ascii::Char::CapitalE => core::ascii::Char::SmallE,
        core::ascii::Char::CapitalF => core::ascii::Char::SmallF,
        core::ascii::Char::CapitalG => core::ascii::Char::SmallG,
        core::ascii::Char::CapitalH => core::ascii::Char::SmallH,
        core::ascii::Char::CapitalI => core::ascii::Char::SmallI,
        core::ascii::Char::CapitalJ => core::ascii::Char::SmallJ,
        core::ascii::Char::CapitalK => core::ascii::Char::SmallK,
        core::ascii::Char::CapitalL => core::ascii::Char::SmallL,
        core::ascii::Char::CapitalM => core::ascii::Char::SmallM,
        core::ascii::Char::CapitalN => core::ascii::Char::SmallN,
        core::ascii::Char::CapitalO => core::ascii::Char::SmallO,
        core::ascii::Char::CapitalP => core::ascii::Char::SmallP,
        core::ascii::Char::CapitalQ => core::ascii::Char::SmallQ,
        core::ascii::Char::CapitalR => core::ascii::Char::SmallR,
        core::ascii::Char::CapitalS => core::ascii::Char::SmallS,
        core::ascii::Char::CapitalT => core::ascii::Char::SmallT,
        core::ascii::Char::CapitalU => core::ascii::Char::SmallU,
        core::ascii::Char::CapitalV => core::ascii::Char::SmallV,
        core::ascii::Char::CapitalW => core::ascii::Char::SmallW,
        core::ascii::Char::CapitalX => core::ascii::Char::SmallX,
        core::ascii::Char::CapitalY => core::ascii::Char::SmallY,
        core::ascii::Char::CapitalZ => core::ascii::Char::SmallZ,
        t => t,
    }
}

// #[cfg(test)]
// mod test {
//     use core::time::Duration;
//     use tokio::time::sleep;
//     use super::*;
//
//     #[derive(Debug, PartialEq, Format, Copy, Clone)]
//     pub enum Key {
//         One,
//         Two,
//     }
//
//     impl From<Key> for Char {
//         fn from(key: Key) -> Char {
//             match key {
//                 Key::One => Char::Digit1,
//                 Key::Two => Char::CapitalA,
//             }
//         }
//     }
//
//     struct TwoKeys<'a>(&'a [Key], usize);
//
//     impl<'a> TwoKeys<'a> {
//         fn new(presses: &'a [Key]) -> Self {
//             TwoKeys(presses, 0)
//         }
//     }
//
//     impl Keypad for TwoKeys<'_> {
//         type Button = Key;
//
//         async fn event(&mut self) -> crate::keypad::Event<Self::Button> {
//             let result = self.0[self.1];
//             self.1 += 1;
//             crate::keypad::Event::Down(result)
//         }
//     }
//
//     #[tokio::test]
//     #[should_panic]
//     async fn test_timeout() {
//         let presses = [];
//         let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));
//         multi_tap.event(async {}).await;
//     }
//
//     #[tokio::test]
//     async fn test_one() {
//         let presses = [Key::One];
//         let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Tentative(Char::Digit1)
//         )
//     }
//
//     #[tokio::test]
//     async fn test_one_two() {
//         let presses = [Key::One, Key::Two];
//         let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));
//
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Tentative(Char::Digit1)
//         );
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Decided(Char::Digit1)
//         );
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Tentative(Char::CapitalA)
//         );
//     }
//
//     #[tokio::test]
//     async fn test_one_one() {
//         let presses = [Key::One, Key::One];
//         let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));
//
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Tentative(Char::Digit1)
//         );
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Tentative(Char::Digit2)
//         );
//     }
//
//     #[tokio::test]
//     async fn test_one_timeout() {
//         let presses = [Key::One];
//         let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Tentative(Char::Digit1)
//         );
//         assert_eq!(
//             multi_tap.event(async {}).await,
//             Event::Decided(Char::Digit1)
//         );
//     }
//
//     #[tokio::test]
//     async fn test_one_two_timeout() {
//         let presses = [Key::One, Key::Two];
//         let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));
//
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Tentative(Char::Digit1)
//         );
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Decided(Char::Digit1)
//         );
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Tentative(Char::CapitalA)
//         );
//         assert_eq!(
//             multi_tap.event(async {}).await,
//             Event::Decided(Char::CapitalA)
//         );
//     }
//
//     #[tokio::test]
//     async fn test_one_one_timeout() {
//         let presses = [Key::One, Key::One];
//         let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));
//
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Tentative(Char::Digit1)
//         );
//         assert_eq!(
//             multi_tap.event(sleep(Duration::from_secs(100))).await,
//             Event::Tentative(Char::Digit2)
//         );
//         assert_eq!(
//             multi_tap.event(async {}).await,
//             Event::Decided(Char::Digit2)
//         );
//     }
// }

pub fn next_char(c: Char) -> Char {
    match c {
        Char::CapitalA => Char::CapitalB,
        Char::CapitalB => Char::CapitalC,
        Char::CapitalC => Char::CapitalA,
        Char::CapitalD => Char::CapitalE,
        Char::CapitalE => Char::CapitalF,
        Char::CapitalF => Char::CapitalD,
        Char::CapitalG => Char::CapitalH,
        Char::CapitalH => Char::CapitalI,
        Char::CapitalI => Char::CapitalG,
        Char::CapitalJ => Char::CapitalK,
        Char::CapitalK => Char::CapitalL,
        Char::CapitalL => Char::CapitalJ,
        Char::CapitalM => Char::CapitalN,
        Char::CapitalN => Char::CapitalO,
        Char::CapitalO => Char::CapitalM,
        Char::CapitalP => Char::CapitalQ,
        Char::CapitalQ => Char::CapitalR,
        Char::CapitalR => Char::CapitalS,
        Char::CapitalS => Char::CapitalP,
        Char::CapitalT => Char::CapitalU,
        Char::CapitalU => Char::CapitalV,
        Char::CapitalV => Char::CapitalT,
        Char::CapitalW => Char::CapitalX,
        Char::CapitalX => Char::CapitalY,
        Char::CapitalY => Char::CapitalZ,
        Char::CapitalZ => Char::CapitalW,
        Char::SmallA => Char::SmallB,
        Char::SmallB => Char::SmallC,
        Char::SmallC => Char::SmallA,
        Char::SmallD => Char::SmallE,
        Char::SmallE => Char::SmallF,
        Char::SmallF => Char::SmallD,
        Char::SmallG => Char::SmallH,
        Char::SmallH => Char::SmallI,
        Char::SmallI => Char::SmallG,
        Char::SmallJ => Char::SmallK,
        Char::SmallK => Char::SmallL,
        Char::SmallL => Char::SmallJ,
        Char::SmallM => Char::SmallN,
        Char::SmallN => Char::SmallO,
        Char::SmallO => Char::SmallM,
        Char::SmallP => Char::SmallQ,
        Char::SmallQ => Char::SmallR,
        Char::SmallR => Char::SmallS,
        Char::SmallS => Char::SmallP,
        Char::SmallT => Char::SmallU,
        Char::SmallU => Char::SmallV,
        Char::SmallV => Char::SmallT,
        Char::SmallW => Char::SmallX,
        Char::SmallX => Char::SmallY,
        Char::SmallY => Char::SmallZ,
        Char::SmallZ => Char::SmallW,
        Char::Digit1 => Char::Digit2,
        Char::Digit2 => Char::Digit3,
        Char::Digit3 => Char::Digit4,
        Char::Digit4 => Char::Digit5,
        Char::Digit5 => Char::Digit6,
        Char::Digit6 => Char::Digit7,
        Char::Digit7 => Char::Digit8,
        Char::Digit8 => Char::Digit9,
        Char::Digit9 => Char::Digit0,
        Char::Digit0 => Char::Digit1,
        e => e,
    }
}
