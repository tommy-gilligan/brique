#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NoteName {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    Pause,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseNoteNameError;

use core::str::FromStr;

impl FromStr for NoteName {
    type Err = ParseNoteNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "A" | "a" => Ok(NoteName::A),
            "A#" | "a#" => Ok(NoteName::ASharp),
            "B" | "b" => Ok(NoteName::B),
            "C" | "c" => Ok(NoteName::C),
            "C#" | "c#" => Ok(NoteName::CSharp),
            "D" | "d" => Ok(NoteName::D),
            "D#" | "d#" => Ok(NoteName::DSharp),
            "E" | "e" => Ok(NoteName::E),
            "F" | "f" => Ok(NoteName::F),
            "F#" | "f#" => Ok(NoteName::FSharp),
            "G" | "g" => Ok(NoteName::G),
            "G#" | "g#" => Ok(NoteName::GSharp),
            "P" | "p" => Ok(NoteName::Pause),
            _ => Err(ParseNoteNameError),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Note {
    name: NoteName,
    duration: u32,
    octave: u32,
    tripled: bool,
    beats_per_minute: u32,
}

impl Note {
    pub fn new(
        text: &str,
        default_octave: u32,
        default_duration: u32,
        beats_per_minute: u32,
    ) -> Self {
        let n = text.trim();
        let mut not_digit = n.match_indices(|c: char| c.is_ascii_alphabetic());
        let (name_start_index, _) = not_digit.next().unwrap();
        let (mut name_end_index, _) = not_digit.next().unwrap_or((name_start_index + 1, ""));
        if n.contains('#') {
            name_end_index += 1;
        }
        let mut end = n.len();
        if n.ends_with(".") {
            end -= 1;
        }
        let octave = n[name_end_index..end.max(name_end_index)]
            .parse()
            .unwrap_or(default_octave);

        Self {
            octave,
            name: n[name_start_index..name_end_index].parse().unwrap(),
            duration: n[..name_start_index].parse().unwrap_or(default_duration),
            tripled: n.ends_with("."),
            beats_per_minute,
        }
    }

    pub fn duration(&self) -> u32 {
        if self.tripled {
            (2 * 240 * 1000) / (3 * self.beats_per_minute * self.duration)
        } else {
            (240 * 1000) / (self.beats_per_minute * self.duration)
        }
    }

    pub fn frequency(&self) -> Option<Result<u32, ()>> {
        match (self.octave, self.name) {
            (_, NoteName::Pause) => None,
            (1, NoteName::C) => Some(Ok(33)),
            (1, NoteName::CSharp) => Some(Ok(35)),
            (1, NoteName::D) => Some(Ok(37)),
            (1, NoteName::DSharp) => Some(Ok(39)),
            (1, NoteName::E) => Some(Ok(42)),
            (1, NoteName::F) => Some(Ok(44)),
            (1, NoteName::FSharp) => Some(Ok(46)),
            (1, NoteName::G) => Some(Ok(49)),
            (1, NoteName::GSharp) => Some(Ok(52)),
            (1, NoteName::A) => Some(Ok(55)),
            (1, NoteName::ASharp) => Some(Ok(58)),
            (1, NoteName::B) => Some(Ok(61)),
            (2, NoteName::C) => Some(Ok(65)),
            (2, NoteName::CSharp) => Some(Ok(69)),
            (2, NoteName::D) => Some(Ok(73)),
            (2, NoteName::DSharp) => Some(Ok(78)),
            (2, NoteName::E) => Some(Ok(82)),
            (2, NoteName::F) => Some(Ok(87)),
            (2, NoteName::FSharp) => Some(Ok(92)),
            (2, NoteName::G) => Some(Ok(98)),
            (2, NoteName::GSharp) => Some(Ok(104)),
            (2, NoteName::A) => Some(Ok(110)),
            (2, NoteName::ASharp) => Some(Ok(117)),
            (2, NoteName::B) => Some(Ok(123)),
            (3, NoteName::C) => Some(Ok(130)),
            (3, NoteName::CSharp) => Some(Ok(138)),
            (3, NoteName::D) => Some(Ok(146)),
            (3, NoteName::DSharp) => Some(Ok(155)),
            (3, NoteName::E) => Some(Ok(164)),
            (3, NoteName::F) => Some(Ok(174)),
            (3, NoteName::FSharp) => Some(Ok(184)),
            (3, NoteName::G) => Some(Ok(195)),
            (3, NoteName::GSharp) => Some(Ok(207)),
            (3, NoteName::A) => Some(Ok(220)),
            (3, NoteName::ASharp) => Some(Ok(233)),
            (3, NoteName::B) => Some(Ok(246)),
            (4, NoteName::C) => Some(Ok(261)),
            (4, NoteName::CSharp) => Some(Ok(277)),
            (4, NoteName::D) => Some(Ok(293)),
            (4, NoteName::DSharp) => Some(Ok(311)),
            (4, NoteName::E) => Some(Ok(329)),
            (4, NoteName::F) => Some(Ok(349)),
            (4, NoteName::FSharp) => Some(Ok(369)),
            (4, NoteName::G) => Some(Ok(391)),
            (4, NoteName::GSharp) => Some(Ok(415)),
            (4, NoteName::A) => Some(Ok(440)),
            (4, NoteName::ASharp) => Some(Ok(466)),
            (4, NoteName::B) => Some(Ok(493)),
            (5, NoteName::C) => Some(Ok(523)),
            (5, NoteName::CSharp) => Some(Ok(554)),
            (5, NoteName::D) => Some(Ok(587)),
            (5, NoteName::DSharp) => Some(Ok(622)),
            (5, NoteName::E) => Some(Ok(659)),
            (5, NoteName::F) => Some(Ok(698)),
            (5, NoteName::FSharp) => Some(Ok(739)),
            (5, NoteName::G) => Some(Ok(783)),
            (5, NoteName::GSharp) => Some(Ok(830)),
            (5, NoteName::A) => Some(Ok(880)),
            (5, NoteName::ASharp) => Some(Ok(932)),
            (5, NoteName::B) => Some(Ok(987)),
            (6, NoteName::C) => Some(Ok(1046)),
            (6, NoteName::CSharp) => Some(Ok(1108)),
            (6, NoteName::D) => Some(Ok(1174)),
            (6, NoteName::DSharp) => Some(Ok(1244)),
            (6, NoteName::E) => Some(Ok(1318)),
            (6, NoteName::F) => Some(Ok(1396)),
            (6, NoteName::FSharp) => Some(Ok(1479)),
            (6, NoteName::G) => Some(Ok(1567)),
            (6, NoteName::GSharp) => Some(Ok(1661)),
            (6, NoteName::A) => Some(Ok(1760)),
            (6, NoteName::ASharp) => Some(Ok(1864)),
            (6, NoteName::B) => Some(Ok(1975)),
            (7, NoteName::C) => Some(Ok(2093)),
            (7, NoteName::CSharp) => Some(Ok(2217)),
            (7, NoteName::D) => Some(Ok(2349)),
            (7, NoteName::DSharp) => Some(Ok(2489)),
            (7, NoteName::E) => Some(Ok(2637)),
            (7, NoteName::F) => Some(Ok(2794)),
            (7, NoteName::FSharp) => Some(Ok(2960)),
            (7, NoteName::G) => Some(Ok(3136)),
            (7, NoteName::GSharp) => Some(Ok(3322)),
            (7, NoteName::A) => Some(Ok(3520)),
            (7, NoteName::ASharp) => Some(Ok(3729)),
            (7, NoteName::B) => Some(Ok(3951)),
            (_o, _n) => Some(Err(())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_note() {
        assert_eq!(
            Note::new("2a3", 5, 4, 108),
            Note {
                name: NoteName::A,
                octave: 3,
                duration: 2,
                tripled: false,
                beats_per_minute: 108
            }
        );
    }

    #[test]
    fn test_note_tripled() {
        assert_eq!(
            Note::new("2a3.", 5, 4, 100),
            Note {
                name: NoteName::A,
                octave: 3,
                duration: 2,
                tripled: true,
                beats_per_minute: 100
            }
        );
    }

    #[test]
    fn test_note_sharp() {
        assert_eq!(
            Note::new("32d#", 5, 4, 104),
            Note {
                name: NoteName::DSharp,
                octave: 5,
                duration: 32,
                tripled: false,
                beats_per_minute: 104
            }
        );
    }

    #[test]
    fn test_note_sharp_tripled() {
        assert_eq!(
            Note::new("32d#.", 5, 4, 102),
            Note {
                name: NoteName::DSharp,
                octave: 5,
                duration: 32,
                tripled: true,
                beats_per_minute: 102
            }
        );
    }
}
