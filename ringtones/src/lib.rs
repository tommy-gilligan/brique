#![no_std]
#![feature(ascii_char_variants)]
#![feature(ascii_char)]

use shared::Application;

pub struct Ringtones<'a> {
    menu: shared::menu::Menu<'a>,
    song: Option<rtttl::Song<'a>>
}

use core::fmt::Debug;

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};

const HAUNTED_HOUSE: &str = "HauntHouse: d=4,o=5,b=108: 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4, 1p, 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4";
const COUNTDOWN: &str = "countdown:d=4, o=5, b=125:p, 8p, 16b, 16a, b, e, p, 8p, 16c6, 16b, 8c6, 8b, a, p, 8p, 16c6, 16b, c6, e, p, 8p, 16a, 16g, 8a, 8g, 8f#, 8a, g., 16f#, 16g, a., 16g, 16a, 8b, 8a, 8g, 8f#, e, c6, 2b., 16b, 16c6, 16b, 16a, 1b";
const MISSION: &str = "Mission:d=4, o=6, b=100:32d, 32d#, 32d, 32d#, 32d, 32d#, 32d, 32d#, 32d, 32d, 32d#, 32e, 32f, 32f#, 32g, 16g, 8p, 16g, 8p, 16a#, 16p, 16c, 16p, 16g, 8p, 16g, 8p, 16f, 16p, 16f#, 16p, 16g, 8p, 16g, 8p, 16a#, 16p, 16c, 16p, 16g, 8p, 16g, 8p, 16f, 16p, 16f#, 16p, 16a#, 16g, 2d, 32p, 16a#, 16g, 2c#, 32p, 16a#, 16g, 2c, 16p, 16a#5, 16c";

const SONGS: [&str; 3] = [
    HAUNTED_HOUSE,
    COUNTDOWN,
    MISSION
];

impl <'a>Ringtones<'a> {
    pub fn new() -> Self {
        Self {
            menu: shared::menu::Menu::new(
                &[
                    HAUNTED_HOUSE,
                    COUNTDOWN,
                    MISSION
                ]
            ),
            song: None
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
                match self.menu.process(device).await {
                    i => {
                        self.song = Some(rtttl::Song::new(SONGS[i]));
                    }
                }
            },
            Some(song) => {
                if let Some(note) = song.next() {
                    if let Some(frequency) = note.frequency() {
                        device.unmute_buzzer();
                        device.set_frequency(frequency.unwrap() as u16);
                    } else {
                        device.mute_buzzer();
                    }

                    embassy_time::Timer::after_millis(note.duration(song.beats_per_minute).into()).await
                } else {
                    device.mute_buzzer();
                    self.song = None;
                }
            }
        }

        Ok(None)
    }
}
