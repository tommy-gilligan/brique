#![expect(dead_code)]
use core::ascii::Char;

use defmt::Format;

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

impl Event {
    fn decide(self) -> Option<Self> {
        match self {
            Self::Tentative(c) => Some(Self::Decided(c)),
            _ => None,
        }
    }

    fn next_char(self) -> Option<Self> {
        match self {
            Self::Tentative(c) => {
                let result = match c {
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
                };
                Some(Self::Tentative(result))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Last {
    held_key_event: Option<crate::held_key::Event>,
    event: Option<Event>,
}

impl Last {
    fn new() -> Self {
        Self {
            held_key_event: None,
            event: None,
        }
    }

    fn clear(&mut self) {
        self.held_key_event = None;
        self.event = None;
    }

    fn set_held_key_event(
        &mut self,
        held_key_event: Option<crate::held_key::Event>,
    ) -> Option<crate::held_key::Event> {
        let result = self.held_key_event.clone();
        self.held_key_event = held_key_event;
        result
    }

    fn set_event(&mut self, event: Option<Event>) -> Option<Event> {
        let result = self.event;
        self.event = event;
        result
    }
}

#[derive(Debug)]
pub struct MultiTap {
    case_state: CaseState,
    last: Last,
    pending: Pending<Event>,
    held_key: HeldKey,
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
            held_key: HeldKey::new(15000, 5000),
            duration,
        }
    }

    fn case(&self) -> Case {
        self.case_state.case()
    }

    pub async fn event(&mut self, keypad: &mut impl crate::Keypad) -> Option<Event> {
        if let Some(pending) = self.pending.dequeue() {
            self.last.set_event(Some(pending));
            return Some(pending);
        }

        let key = self.held_key.event(keypad).await;
        let last_key = self.last.set_held_key_event(key.clone());

        let result = match key {
            Some(crate::held_key::Event::Down(Key::Asterisk)) => Some(Event::ShowSpecialCharacters),
            Some(crate::held_key::Event::Down(Key::Hash)) => {
                self.case_state.cycle_case();
                Some(Event::Case(self.case()))
            }
            Some(crate::held_key::Event::Down(Key::Cancel)) => {
                Some(Event::Decided(core::ascii::Char::Backspace))
            }
            Some(crate::held_key::Event::Delay(Key::Hash)) => {
                self.case_state.enable_numeric_case();
                Some(Event::Case(self.case()))
            }
            Some(crate::held_key::Event::Delay(Key::Cancel)) => None,
            Some(crate::held_key::Event::Delay(d)) => {
                self.last.clear();
                Some(Event::Decided(digit(d)))
            }
            Some(crate::held_key::Event::Down(ref now)) => {
                if key == last_key {
                    self.last.event.unwrap().next_char()
                } else if last_key.is_some() {
                    let result = self.last.event.unwrap().decide();
                    self.pending
                        .enqueue(Event::Tentative(lowercase(now.clone().into())));
                    result
                } else {
                    Some(Event::Tentative(lowercase(now.clone().into())))
                }
            }
            None | Some(crate::held_key::Event::Repeat(_)) => None,
        };

        self.last.set_event(result);
        result
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

#[cfg(test)]
mod test {
    use futures_executor::block_on;

    #[test]
    fn test_tentative() {
        block_on(async {
            let mut keypad = crate::test::Keypad::new(&[crate::KeyEvent::Down(crate::Key::Two)]);
            let mut multitap = super::MultiTap::new(1000);
            assert_eq!(
                multitap.event(&mut keypad).await,
                Some(super::Event::Case(super::Case::Lower))
            );
            assert_eq!(
                multitap.event(&mut keypad).await,
                Some(super::Event::Tentative(core::ascii::Char::SmallA))
            );
        });
    }

    #[test]
    fn test_tentative_next() {
        block_on(async {
            let mut keypad = crate::test::Keypad::new(&[
                crate::KeyEvent::Down(crate::Key::Two),
                crate::KeyEvent::Down(crate::Key::Two),
            ]);
            let mut multitap = super::MultiTap::new(1000);
            assert_eq!(
                multitap.event(&mut keypad).await,
                Some(super::Event::Case(super::Case::Lower))
            );
            assert_eq!(
                multitap.event(&mut keypad).await,
                Some(super::Event::Tentative(core::ascii::Char::SmallA))
            );
            assert_eq!(
                multitap.event(&mut keypad).await,
                Some(super::Event::Tentative(core::ascii::Char::SmallB))
            );
        });
    }

    #[test]
    fn test_decided_by_timeout() {}

    #[test]
    fn test_hold_for_number() {}

    #[test]
    fn test_decided_by_other() {
        block_on(async {
            let mut keypad = crate::test::Keypad::new(&[
                crate::KeyEvent::Down(crate::Key::Two),
                crate::KeyEvent::Down(crate::Key::Three),
            ]);
            let mut multitap = super::MultiTap::new(1000);
            assert_eq!(
                multitap.event(&mut keypad).await,
                Some(super::Event::Case(super::Case::Lower))
            );
            assert_eq!(
                multitap.event(&mut keypad).await,
                Some(super::Event::Tentative(core::ascii::Char::SmallA))
            );
            assert_eq!(
                multitap.event(&mut keypad).await,
                Some(super::Event::Decided(core::ascii::Char::SmallA))
            );
            assert_eq!(
                multitap.event(&mut keypad).await,
                Some(super::Event::Tentative(core::ascii::Char::SmallD))
            );
        });
    }
}
