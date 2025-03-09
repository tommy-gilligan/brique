use core::{ascii::Char, future::Future};
use crate::Key;
use defmt::Format;

use futures::{future, future::Either, pin_mut};

#[derive(Debug, PartialEq, Format, Copy, Clone)]
pub enum Event {
    Tentative(Char),
    Decided(Char),
    Case
}

pub struct MultiTap {
    last_press: Option<crate::Key>,
    last_emitted: Option<Event>,
    pending: Option<Event>,
    shift: bool
}

impl MultiTap {
    pub fn new() -> Self {
        Self {
            last_press: None,
            last_emitted: None,
            pending: None,
            shift: false
        }
    }

    pub async fn accept(&mut self) {
    }

    pub async fn event<T, KEYPAD>(
        &mut self,
        keypad: &mut KEYPAD,
        timeout_future: T,
        case: &crate::textbox::Case
    ) -> Event where T: Future<Output = ()>, KEYPAD: crate::Keypad {
        // if something has just been decided
        // still emit the next tentative
        // if let Some(pending) = self.pending {
        //     self.pending = None;
        //     self.last_emitted = Some(pending);
        //     return pending;
        // }

        let event_future = keypad.event();
        pin_mut!(event_future);
        pin_mut!(timeout_future);

        if self.last_press.is_some() {

            match future::select(timeout_future, event_future).await {
                Either::Left((..)) => {
                    if let Some(Event::Tentative(e)) = self.last_emitted {
                        self.last_emitted = None;
                        self.last_press = None;

                        return Event::Decided(e);
                    }
                }
                Either::Right((crate::KeyEvent::Down(crate::Key::Hash), _)) => {
                    return Event::Case;
                }
                Either::Right((crate::KeyEvent::Down(e), _)) => {
                    if let Some(p) = &self.last_press && *p != e {
                        if let Some(Event::Tentative(f)) = self.last_emitted {
                            self.last_press = Some(e.clone());
                            let e: Char = match case {
                                crate::textbox::Case::Upper => {
                                    e.clone().into()
                                },
                                _ => {
                                    let c: Char = e.clone().into();
                                    lowercase(c)
                                }
                            };
                            self.last_emitted = Some(Event::Tentative(e.clone().into()));
                            // TODO: panic if there is already a pending event
                            self.pending = Some(Event::Tentative(e.clone().into()));

                            return Event::Decided(f);
                        }
                    } else {
                        self.last_press = Some(e.clone());
                        self.last_emitted = match self.last_emitted {
                            Some(Event::Case) => Some(Event::Case),

                            Some(Event::Tentative(c)) => Some(Event::Tentative(next_char(c))),
                            Some(Event::Decided(_)) => None,
                            None => {
                                match case {
                                    crate::textbox::Case::Upper => {
                                        Some(Event::Tentative(e.clone().into()))
                                    },
                                    _ => {
                                        let c: Char = e.clone().into();
                                        Some(Event::Tentative(lowercase(c)))

                                    }
                                }
                            }
                        };

                        return self.last_emitted.unwrap();
                    }
                }
                Either::Right((crate::KeyEvent::Up(_), _)) => {}
            }

        } else if let crate::KeyEvent::Down(e) = event_future.await {
            match e {
                crate::Key::Hash => {
                    return Event::Case;
                },
                _ => {
                    if let Some(p) = &self.last_press && *p != e {
                        if let Some(Event::Tentative(f)) = self.last_emitted {
                            self.last_emitted = Some(Event::Tentative(e.clone().into()));
                            self.last_press = Some(e.clone());
                            // TODO: panic if there is already a pending event
                            self.pending = Some(Event::Tentative(e.clone().into()));

                            return Event::Decided(f);
                        }
                    } else {
                        self.last_press = Some(e.clone());
                        self.last_emitted = match self.last_emitted {
                            Some(Event::Case) => Some(Event::Case),

                            Some(Event::Tentative(c)) => Some(Event::Tentative(next_char(c))),
                            Some(Event::Decided(_)) => None,
                            None => {
                                match case {
                                    crate::textbox::Case::Upper => {
                                        Some(Event::Tentative(e.clone().into()))
                                    },
                                    _ => {
                                        let c: Char = e.clone().into();
                                        Some(Event::Tentative(lowercase(c)))

                                    }
                                }
                            },
                        };

                        return self.last_emitted.unwrap();
                    }
                }
            }

        }

        panic!()
    }
}

fn digit(k: Key) -> Char {
    match k {
        Key::One => core::ascii::Char::Digit1,
        Key::Two => core::ascii::Char::Digit2,
        Key::Three => core::ascii::Char::Digit3,
        Key::Four => core::ascii::Char::Digit4,
        Key::Five => core::ascii::Char::Digit5,
        Key::Six => core::ascii::Char::Digit6,
        Key::Seven => core::ascii::Char::Digit7,
        Key::Eight => core::ascii::Char::Digit8,
        Key::Nine => core::ascii::Char::Digit9,
        Key::Zero => core::ascii::Char::Digit0,
        _ => core::ascii::Char::Digit0
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
