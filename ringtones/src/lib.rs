#![no_std]
#![feature(ascii_char_variants)]
#![feature(ascii_char)]
#![feature(iter_advance_by)]

use shared::Application;

pub struct Ringtones<'a> {
    menu: shared::menu::Menu<'a>,
    song: Option<rtttl::Song<'a>>,
    song_index: Option<usize>
}

use core::fmt::Debug;

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};

const HAUNTED_HOUSE: &str = "HauntHouse: d=4,o=5,b=108: 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4, 1p, 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4";
const COUNTDOWN: &str = "countdown:d=4, o=5, b=125:p, 8p, 16b, 16a, b, e, p, 8p, 16c6, 16b, 8c6, 8b, a, p, 8p, 16c6, 16b, c6, e, p, 8p, 16a, 16g, 8a, 8g, 8f#, 8a, g., 16f#, 16g, a., 16g, 16a, 8b, 8a, 8g, 8f#, e, c6, 2b., 16b, 16c6, 16b, 16a, 1b";
const MISSION: &str = "Mission:d=4, o=6, b=100:32d, 32d#, 32d, 32d#, 32d, 32d#, 32d, 32d#, 32d, 32d, 32d#, 32e, 32f, 32f#, 32g, 16g, 8p, 16g, 8p, 16a#, 16p, 16c, 16p, 16g, 8p, 16g, 8p, 16f, 16p, 16f#, 16p, 16g, 8p, 16g, 8p, 16a#, 16p, 16c, 16p, 16g, 8p, 16g, 8p, 16f, 16p, 16f#, 16p, 16a#, 16g, 2d, 32p, 16a#, 16g, 2c#, 32p, 16a#, 16g, 2c, 16p, 16a#5, 16c";
const BARBIE_GIRL: &str = "Barbie Girl:o=5,d=8,b=125,b=125:g#,e,g#,c#6,4a,4p,f#,d#,f#,b,4g#,f#,e,4p,e,c#,4f#,4c#,4p,f#,e,4g#,4f#";
const RICH_MAN: &str = "Rich Man's World:o=6,d=8,b=112,b=112:e,e,e,e,e,e,16e5,16a5,16c,16e,d#,d#,d#,d#,d#,d#,16f5,16a5,16c,16d#,4d,c,a5,c,4c,2a5,32a5,32c,32e,a6";
const WANNABE: &str = "Wannabe:o=5,d=8,b=125,b=125:16g,16g,16g,16g,g,a,g,e,p,16c,16d,16c,d,d,c,4e,4p,g,g,g,a,g,e,p,4c6,c6,b,g,a,16b,16a,4g";

const SONGS: [&str; 6] = [
    WANNABE,
    RICH_MAN,
    BARBIE_GIRL,
    HAUNTED_HOUSE,
    COUNTDOWN,
    MISSION
];

impl <'a>Ringtones<'a> {
    pub fn new() -> Self {
        Self {
            menu: shared::menu::Menu::new(
                &[
                    WANNABE,
                    RICH_MAN,
                    BARBIE_GIRL,
                    HAUNTED_HOUSE,
                    COUNTDOWN,
                    MISSION
                ]
            ),
            song: None,
            song_index: None
        }
    }
}

impl Application for Ringtones<'_> {
    async fn run(
        &mut self,
        device: &mut impl shared::Device,
        _system_response: Option<[u8; 64]>,
    ) -> Result<Option<shared::SystemRequest>, ()> {

        match &mut self.song {
            None => {
                match embassy_time::with_timeout(
                    embassy_time::Duration::from_millis(1000),
                    self.menu.process(device, "Pause"),
                ).await {
                    Ok(Some(i)) => {
                        log::debug!("Selected ringtone {}", i);
                        self.song = Some(rtttl::Song::new(SONGS[i]));
                    },
                    _ => {}
                }
            },
            Some(song) => {
                if let Some(note) = song.next() {
                    if let Some(frequency) = note.frequency() {
                        device.unmute_buzzer();
                        log::debug!("Playing {}Hz", frequency.unwrap());
                        device.set_frequency(frequency.unwrap() as u16);
                    } else {
                        device.mute_buzzer();
                    }
                    embassy_time::Timer::after_millis(note.duration().into()).await
                } else {
                    device.mute_buzzer();
                    self.song = None;
                    self.song_index = None;
                }
            }
        }

        Ok(None)
    }
}
