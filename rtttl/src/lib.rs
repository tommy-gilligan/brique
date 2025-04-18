#![no_std]

pub mod note;

#[derive(Clone, Debug)]
pub struct Song<'a> {
    pub title: &'a str,
    duration: u32,
    octave: u32,
    pub beats_per_minute: u32,
    note_source: &'a str,
    notes: Option<core::iter::Peekable<core::str::Split<'a, &'a str>>>,
    _time: u32,
}

impl<'a> Song<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut split = text.splitn(3, ':');
        let title = split.next().unwrap().trim();
        let settings = split.next().unwrap().trim().split(',').map(|setting| {
            let s = setting.split_once("=").unwrap();
            Some((s.0.trim(), s.1.trim()))
        });

        let mut duration = 4;
        let mut octave = 5;
        let mut beats_per_minute = 108;

        for setting in settings {
            match setting {
                Some(("o", o)) | Some(("O", o)) => octave = o.parse().unwrap(),
                Some(("d", d)) | Some(("D", d)) => duration = d.parse().unwrap(),
                Some(("b", b)) | Some(("B", b)) => beats_per_minute = b.parse().unwrap(),
                Some((k, v)) => panic!("panic {} {}", k, v),
                None => panic!("panic"),
            }
        }

        Self {
            title,
            duration,
            octave,
            beats_per_minute,
            note_source: split.next().unwrap().trim(),
            notes: None,
            _time: 0,
        }
    }

    pub fn reset(&mut self) {
        self.notes = Some(self.note_source.split(",").peekable());
    }

    pub fn next(&mut self) -> Option<note::Note> {
        if self.notes.is_none() {
            self.reset();
        }
        self.notes
            .as_mut()
            .unwrap()
            .next()
            .map(|n| crate::note::Note::new(n, self.octave, self.duration, self.beats_per_minute))
    }

    // pub fn note_at(&mut self, time_ms: u32) -> Option<note::Note> {
    //     let note = note::Note::new(self.notes.peek().unwrap(), self.octave, self.duration);
    //     self.time += note.duration(self.beats_per_minute);
    //     if self.time > time_ms {
    //         return Some(note);
    //     }
    //     None
    // }
}

impl<'a> core::convert::AsRef<str> for Song<'a> {
    fn as_ref(&self) -> &str {
        self.title
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const HAUNTED_HOUSE: &str = "HauntHouse: d=4,o=5,b=108: 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4, 1p, 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4";
    const COUNTDOWN: &str = "countdown:d=4, o=5, b=125:p, 8p, 16b, 16a, b, e, p, 8p, 16c6, 16b, 8c6, 8b, a, p, 8p, 16c6, 16b, c6, e, p, 8p, 16a, 16g, 8a, 8g, 8f#, 8a, g., 16f#, 16g, a., 16g, 16a, 8b, 8a, 8g, 8f#, e, c6, 2b., 16b, 16c6, 16b, 16a, 1b";
    const MISSION: &str = "Mission:d=4, o=6, b=100:32d, 32d#, 32d, 32d#, 32d, 32d#, 32d, 32d#, 32d, 32d, 32d#, 32e, 32f, 32f#, 32g, 16g, 8p, 16g, 8p, 16a#, 16p, 16c, 16p, 16g, 8p, 16g, 8p, 16f, 16p, 16f#, 16p, 16g, 8p, 16g, 8p, 16a#, 16p, 16c, 16p, 16g, 8p, 16g, 8p, 16f, 16p, 16f#, 16p, 16a#, 16g, 2d, 32p, 16a#, 16g, 2c#, 32p, 16a#, 16g, 2c, 16p, 16a#5, 16c";
    const WANNABE: &str = "Wannabe:o=5,d=8,b=125,b=125:16g,16g,16g,16g,g,a,g,e,p,16c,16d,16c,d,d,c,4e,4p,g,g,g,a,g,e,p,4c6,c6,b,g,a,16b,16a,4g";

    #[test]
    fn test_countdown() {
        let mut song = Song::new(COUNTDOWN);

        assert_eq!(song.title, "countdown");
        assert_eq!(song.duration, 4);
        assert_eq!(song.octave, 5);
        assert_eq!(song.beats_per_minute, 125);

        let note = song.next().unwrap();
        assert_eq!(note.duration(), 480);
        assert_eq!(note.frequency(), None);

        let note = song.next().unwrap();
        assert_eq!(note.frequency(), None);
        assert_eq!(note.duration(), 240);

        let note = song.next().unwrap();
        assert_eq!(note.frequency(), Some(Ok(987)));
        assert_eq!(note.duration(), 120);

        let note = song.next().unwrap();
        assert_eq!(note.frequency(), Some(Ok(880)));
        assert_eq!(note.duration(), 120);

        let note = song.next().unwrap();
        assert_eq!(note.frequency(), Some(Ok(987)));
        assert_eq!(note.duration(), 480);

        let note = song.next().unwrap();
        assert_eq!(note.frequency(), Some(Ok(659)));
        assert_eq!(note.duration(), 480);

        let note = song.next().unwrap();
        assert_eq!(note.duration(), 480);
        assert_eq!(note.frequency(), None);

        let note = song.next().unwrap();
        assert_eq!(note.frequency(), None);
        assert_eq!(note.duration(), 240);

        let note = song.next().unwrap();
        assert_eq!(note.frequency(), Some(Ok(1046)));
        assert_eq!(note.duration(), 120);
    }

    #[test]
    fn test_song() {
        let mut song = Song::new(HAUNTED_HOUSE);

        assert_eq!(song.title, "HauntHouse");
        assert_eq!(song.duration, 4);
        assert_eq!(song.octave, 5);
        assert_eq!(song.beats_per_minute, 108);

        let first_note = song.next().unwrap();
        assert_eq!(first_note.duration(), 1111);
        assert_eq!(first_note.frequency().unwrap().unwrap(), 440);

        let second_note = song.next().unwrap();
        assert_eq!(second_note.duration(), 1111);
        assert_eq!(second_note.frequency().unwrap().unwrap(), 659);
    }

    #[test]
    fn test_mission() {
        let mut song = Song::new(MISSION);

        assert_eq!(song.title, "Mission");
        assert_eq!(song.duration, 4);
        assert_eq!(song.octave, 6);
        assert_eq!(song.beats_per_minute, 100);

        let first_note = song.next().unwrap();
        assert_eq!(first_note.duration(), 75);
        assert_eq!(first_note.frequency().unwrap().unwrap(), 1174);

        let second_note = song.next().unwrap();
        assert_eq!(second_note.duration(), 75);
        assert_eq!(second_note.frequency().unwrap().unwrap(), 1244);
    }

    #[test]
    fn test_wannabe() {
        let mut song = Song::new(WANNABE);

        assert_eq!(song.title, "Wannabe");
        assert_eq!(song.duration, 8);
        assert_eq!(song.octave, 5);
        assert_eq!(song.beats_per_minute, 125);

        let first_note = song.next().unwrap();
        assert_eq!(first_note.duration(), 120);
        assert_eq!(first_note.frequency().unwrap().unwrap(), 783);

        let second_note = song.next().unwrap();
        assert_eq!(second_note.duration(), 120);
        assert_eq!(second_note.frequency().unwrap().unwrap(), 783);
    }
}
