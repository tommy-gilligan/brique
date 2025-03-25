#![expect(dead_code)]
use core::ascii::Char;

use defmt::Format;
use embassy_time::Timer;

use crate::{Key, held_key::HeldKey};
mod case;
pub use case::*;
mod pending;
pub use pending::*;

#[derive(Debug, PartialEq, Format, Copy, Clone)]
pub enum Event {
    Tentative(Char),
    Decided(Char),
    Case(Case),
    ShowSpecialCharacters,
}

pub struct Last {
    last_down: Option<Key>,
    last_tentative: Option<Char>,
}

impl Last {
    fn new() -> Self {
        Self {
            last_down: None,
            last_tentative: None,
        }
    }

    fn clear(&mut self) {
        self.last_down = None;
        self.last_tentative = None;
    }
}

pub struct MultiTap {
    case_state: CaseState,
    last: Last,
    pending: Pending<Event>,
    held_key: HeldKey,
    timer: Option<Timer>,
    duration: u64,
}

impl MultiTap {
    pub fn new(duration: u64) -> Self {
        let case_state = CaseState::new(Case::Lower);
        let mut pending = Pending::new();
        pending.enqueue(Event::Case(case_state.case()));

        Self {
            case_state,
            last: Last::new(),
            pending,
            held_key: HeldKey::new(1500, 500),
            duration,
            timer: None,
        }
    }

    fn case(&self) -> Case {
        self.case_state.case()
    }

    fn key_to_char(&mut self, d: Key) -> Char {
        match self.case() {
            Case::Upper => d.clone().into(),
            _ => lowercase(d.clone().into()),
        }
    }

    pub async fn event(&mut self, keypad: &mut impl crate::Keypad) -> Option<Event> {
        if let Some(pending) = self.pending.dequeue() {
            return Some(pending);
        }

        match self.held_key.event(keypad).await {
            Some(crate::held_key::Event::Down(Key::Asterisk)) => Some(Event::ShowSpecialCharacters),
            Some(crate::held_key::Event::Down(Key::Hash)) => {
                self.case_state.cycle_case();
                Some(Event::Case(self.case()))
            }
            Some(crate::held_key::Event::Delay(Key::Hash)) => {
                self.case_state.enable_numeric_case();
                Some(Event::Case(self.case()))
            }
            Some(crate::held_key::Event::Down(Key::Cancel)) => {
                Some(Event::Decided(core::ascii::Char::Backspace))
            }
            Some(crate::held_key::Event::Delay(Key::Cancel)) => None,

            Some(crate::held_key::Event::Delay(d)) => {
                self.last.clear();
                Some(Event::Decided(digit(d)))
            }
            Some(crate::held_key::Event::Down(d)) => {
                if self.case() == Case::Number {
                    Some(Event::Decided(digit(d)))
                } else {
                    None
                }
            }
            None | Some(crate::held_key::Event::Repeat(_)) => None,
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
