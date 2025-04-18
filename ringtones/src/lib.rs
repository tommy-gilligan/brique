#![no_std]
#![feature(ascii_char_variants)]
#![feature(ascii_char)]
#![feature(iter_advance_by)]

use embassy_futures::select::Either;
use embedded_graphics::{
    Drawable,
    mono_font::ascii::{FONT_6X9, FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::Point,
    primitives::{Primitive, PrimitiveStyle},
    text::Text,
};
use shared::Application;

pub struct Ringtones<'a> {
    songs: [rtttl::Song<'a>; 6],
}

const HAUNTED_HOUSE: &str = "HauntHouse: d=4,o=5,b=108: 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4, 1p, 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4";
const COUNTDOWN: &str = "countdown:d=4, o=5, b=125:p, 8p, 16b, 16a, b, e, p, 8p, 16c6, 16b, 8c6, 8b, a, p, 8p, 16c6, 16b, c6, e, p, 8p, 16a, 16g, 8a, 8g, 8f#, 8a, g., 16f#, 16g, a., 16g, 16a, 8b, 8a, 8g, 8f#, e, c6, 2b., 16b, 16c6, 16b, 16a, 1b";
const MISSION: &str = "Mission:d=4, o=6, b=100:32d, 32d#, 32d, 32d#, 32d, 32d#, 32d, 32d#, 32d, 32d, 32d#, 32e, 32f, 32f#, 32g, 16g, 8p, 16g, 8p, 16a#, 16p, 16c, 16p, 16g, 8p, 16g, 8p, 16f, 16p, 16f#, 16p, 16g, 8p, 16g, 8p, 16a#, 16p, 16c, 16p, 16g, 8p, 16g, 8p, 16f, 16p, 16f#, 16p, 16a#, 16g, 2d, 32p, 16a#, 16g, 2c#, 32p, 16a#, 16g, 2c, 16p, 16a#5, 16c";
const BARBIE_GIRL: &str = "Barbie Girl:o=5,d=8,b=125,b=125:g#,e,g#,c#6,4a,4p,f#,d#,f#,b,4g#,f#,e,4p,e,c#,4f#,4c#,4p,f#,e,4g#,4f#";
const RICH_MAN: &str = "Rich Man's World:o=6,d=8,b=112,b=112:e,e,e,e,e,e,16e5,16a5,16c,16e,d#,d#,d#,d#,d#,d#,16f5,16a5,16c,16d#,4d,c,a5,c,4c,2a5,32a5,32c,32e,a6";
const WANNABE: &str = "Wannabe:o=5,d=8,b=125,b=125:16g,16g,16g,16g,g,a,g,e,p,16c,16d,16c,d,d,c,4e,4p,g,g,g,a,g,e,p,4c6,c6,b,g,a,16b,16a,4g";

impl Default for Ringtones<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Ringtones<'_> {
    pub fn new() -> Self {
        Self {
            songs: [
                rtttl::Song::new(WANNABE),
                rtttl::Song::new(RICH_MAN),
                rtttl::Song::new(BARBIE_GIRL),
                rtttl::Song::new(HAUNTED_HOUSE),
                rtttl::Song::new(COUNTDOWN),
                rtttl::Song::new(MISSION),
            ],
        }
    }
}

impl Application for Ringtones<'_> {
    async fn run(&mut self, device: &mut impl shared::Device) -> Result<(), ()> {
        let mut menu = shared::menu::Menu::new(&mut self.songs, Some("PLAY"));
        loop {
            if let Some(song) = menu.process(device).await {
                let _ = device
                    .bounding_box()
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(device);

                let text_style = embedded_graphics::text::TextStyleBuilder::new()
                    .alignment(embedded_graphics::text::Alignment::Center)
                    .baseline(embedded_graphics::text::Baseline::Top)
                    .build();

                let _ = Text::with_text_style(
                    "Playing",
                    Point::new(42, 0),
                    embedded_graphics::mono_font::MonoTextStyleBuilder::new()
                        .text_color(BinaryColor::Off)
                        .font(&FONT_6X10)
                        .build(),
                    text_style,
                )
                .draw(device);

                let _ = Text::with_text_style(
                    song.title,
                    Point::new(42, 10),
                    embedded_graphics::mono_font::MonoTextStyleBuilder::new()
                        .text_color(BinaryColor::Off)
                        .font(&FONT_6X10)
                        .build(),
                    text_style,
                )
                .draw(device);

                let text_style = embedded_graphics::text::TextStyleBuilder::new()
                    .alignment(embedded_graphics::text::Alignment::Center)
                    .baseline(embedded_graphics::text::Baseline::Bottom)
                    .build();
                let _ = Text::with_text_style(
                    "STOP",
                    Point::new(42, 47),
                    embedded_graphics::mono_font::MonoTextStyleBuilder::new()
                        .text_color(BinaryColor::Off)
                        .font(&FONT_6X9)
                        .build(),
                    text_style,
                )
                .draw(device);

                loop {
                    if let Some(note) = song.next() {
                        if let Some(frequency) = note.frequency() {
                            match frequency {
                                Ok(f) => {
                                    let _ = device.unmute_buzzer();
                                    let _ = device.set_frequency(f as u16);
                                }
                                _ => {}
                            }
                        } else {
                            let _ = device.mute_buzzer();
                        }

                        match embassy_futures::select::select(
                            device.event(),
                            embassy_time::Timer::after_millis(note.duration().into()),
                        )
                        .await
                        {
                            Either::First(shared::KeyEvent::Down(_)) => {
                                let _ = device.mute_buzzer();
                                break;
                            }
                            _ => {}
                        }
                    } else {
                        let _ = device.mute_buzzer();
                        break;
                    }
                }
            }
        }
    }
}
