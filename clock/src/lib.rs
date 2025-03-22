#![no_std]

use chrono::Timelike;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::PrimitiveStyle,
    text::{Alignment, Text},
};
use shared::Application;
use chrono::Datelike;
use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence, next_cycle};
use embedded_graphics::primitives::Rectangle;

#[derive(Sequence, Debug, PartialEq, Clone)]
enum Setting {
    Hour,
    Minute,
    Second,
    Day,
    Month,
    Year
}

pub struct Clock {
    setting: Option<Setting>,
    new_time: Option<i64>,
    clock_view: ClockView
}

impl Clock {
    pub fn new() -> Self {
        Self {
            setting: None,
            new_time: None,
            clock_view: ClockView::new()
        }
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            setting: None,
            new_time: None,
            clock_view: ClockView::new()
        }
    }
}

// TODO: use something better
fn to_char(digit: u32) -> char {
    match digit {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => '?',
    }
}

struct ClockView {
    hour: u32,
    minute: u32,
    second: u32,
    day: u32,
    month: u32,
    year: u32,
    selected: Option<Setting>
}

impl ClockView {
    pub fn new() -> Self {
        Self {
            hour: 12,
            minute: 34,
            second: 56,
            day: 1,
            month: 2,
            year: 2025,
            selected: None
        }
    }

    fn draw_separators(&mut self, device: &mut impl shared::Device) {
        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::Off);
        Text::with_alignment(
            ":",
            Point::new(28, 20),
            character_style,
            Alignment::Center,
        )
        .draw(device)
        .unwrap();
        Text::with_alignment(
            ":",
            Point::new(56, 20),
            character_style,
            Alignment::Center,
        )
        .draw(device)
        .unwrap();

        Text::with_alignment(
            "/",
            Point::new(28, 40),
            character_style,
            Alignment::Center,
        )
        .draw(device)
        .unwrap();
        Text::with_alignment(
            "/",
            Point::new(56, 40),
            character_style,
            Alignment::Center,
        )
        .draw(device)
        .unwrap();
    }

    fn increase_seconds(&mut self) {
        self.second = (self.second + 1) % 60;
    }

    fn decrease_seconds(&mut self) {
        if self.second == 0 {
            self.second = 59;
        } else {
            self.second -= 1;
        }
    }

    fn increase_minutes(&mut self) {
        self.minute = (self.minute + 1) % 60;
    }

    fn decrease_minutes(&mut self) {
        if self.minute == 0 {
            self.minute = 59;
        } else {
            self.minute -= 1;
        }
    }

    fn increase_hours(&mut self) {
        self.hour = (self.hour + 1) % 24;
    }

    fn decrease_hours(&mut self) {
        if self.hour == 0 {
            self.hour = 23;
        } else {
            self.hour -= 1;
        }
    }

    fn increase_days(&mut self) {
        self.day = (self.day + 1) % 32;
        if self.day == 0 {
            self.day += 1;
        }
    }

    fn decrease_days(&mut self) {
        if self.day == 1 {
            self.day = 31;
        } else {
            self.day -= 1;
        }
    }

    fn increase_months(&mut self) {
        self.month = (self.month + 1) % 13;
        if self.month == 0 {
            self.month += 1;
        }
    }

    fn decrease_months(&mut self) {
        if self.month == 1 {
            self.month = 12;
        } else {
            self.month -= 1;
        }
    }

    fn increase_years(&mut self) {
        self.year = (self.year + 1) % 2100;
        if self.year == 0 {
            self.year += 1;
        }
    }

    fn decrease_years(&mut self) {
        self.year -= 1;
        if self.year == 2000 {
            self.year = 2099
        }
    }

    fn draw_segment(&mut self, device: &mut impl shared::Device, segment: Setting) {
        let mut text: heapless::String<2> = heapless::String::new();
        let point = match segment {
            Setting::Hour => {
                text.push(to_char(self.hour / 10)).unwrap();
                text.push(to_char(self.hour % 10)).unwrap();
                Point::new(14, 20)
            }
            Setting::Minute => {
                text.push(to_char(self.minute / 10)).unwrap();
                text.push(to_char(self.minute % 10)).unwrap();
                Point::new(42, 20)
            }
            Setting::Second => {
                text.push(to_char(self.second / 10)).unwrap();
                text.push(to_char(self.second % 10)).unwrap();
                Point::new(70, 20)
            }
            Setting::Day => {
                text.push(to_char(self.day / 10)).unwrap();
                text.push(to_char(self.day % 10)).unwrap();
                Point::new(14, 40)
            }
            Setting::Month => {
                text.push(to_char(self.month / 10)).unwrap();
                text.push(to_char(self.month % 10)).unwrap();
                Point::new(42, 40)
            }
            Setting::Year => {
                text.push(to_char(((self.year) % 100) / 10)).unwrap();
                text.push(to_char(self.year % 10)).unwrap();
                Point::new(70, 40)
            }
        };

        let (foreground, background) = if self.selected == Some(segment) {
            (BinaryColor::On, BinaryColor::Off)
        } else {
            (BinaryColor::Off, BinaryColor::On)
        };

        let character_style = MonoTextStyle::new(&FONT_10X20, foreground);
        Rectangle::new(
            point - Point::new(9, 13),
            Size::new(20, 15)
        ).into_styled(PrimitiveStyle::with_fill(background)).draw(device).unwrap();

        Text::with_alignment(
            &text,
            point,
            character_style,
            Alignment::Center,
        )
        .draw(device)
        .unwrap();
    }

    fn draw(&mut self, device: &mut impl shared::Device) {
        device.clear(BinaryColor::On);
        self.draw_separators(device);
        self.draw_segment(device, Setting::Hour);
        self.draw_segment(device, Setting::Minute);
        self.draw_segment(device, Setting::Second);
        self.draw_segment(device, Setting::Day);
        self.draw_segment(device, Setting::Month);
        self.draw_segment(device, Setting::Year);
    }

    fn increment(&mut self) {
        match self.selected {
            Some(Setting::Hour) => {
                self.increase_hours();
            }
            Some(Setting::Minute) => {
                self.increase_minutes();
            }
            Some(Setting::Second) => {
                self.increase_seconds();
            }
            Some(Setting::Day) => {
                self.increase_days();
            }
            Some(Setting::Month) => {
                self.increase_months();
            }
            Some(Setting::Year) => {
                self.increase_years();
            }
            _ => {}
        }
    }

    fn decrement(&mut self) {
        match self.selected {
            Some(Setting::Hour) => {
                self.decrease_hours();
            }
            Some(Setting::Minute) => {
                self.decrease_minutes();
            }
            Some(Setting::Second) => {
                self.decrease_seconds();
            }
            Some(Setting::Day) => {
                self.decrease_days();
            }
            Some(Setting::Month) => {
                self.decrease_months();
            }
            Some(Setting::Year) => {
                self.decrease_years();
            }
            _ => {}
        }
    }

    async fn update(&mut self, device: &mut impl shared::Device) -> bool {
        match device.event().await {
            shared::KeyEvent::Down(shared::Key::Select) => {
                self.selected = match &self.selected {
                    Some(selected) => next::<Setting>(&self.selected.clone().unwrap()),
                    None => Some(first::<Setting>().unwrap()),
                };
                if self.selected.is_none() {
                    return true;
                }
            }
            shared::KeyEvent::Down(shared::Key::Up) => {
                self.increment();
            }
            shared::KeyEvent::Down(shared::Key::Down) => {
                self.decrement();
            }
            _ => {}
        }
        false
    }

    fn set_time(&mut self, time: chrono::DateTime::<chrono::Utc>) {
        self.hour = time.hour();
        self.minute = time.minute();
        self.second = time.second();
        self.day = time.day();
        self.month = time.month();
        self.year = time.year().try_into().unwrap();
    }
}

impl Application for Clock {
    async fn run(
        &mut self,
        device: &mut impl shared::Device,
        _system_response: Option<[u8; 64]>,
    ) -> Result<Option<shared::SystemRequest>, ()> {
        device.clear(BinaryColor::Off);

        let timestamp = device.timestamp().unwrap();
        if self.clock_view.selected.is_none() {
            self.clock_view.set_time(
                chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp, 0).unwrap()
            )
        };
        self.clock_view.draw(device);
        if self.clock_view.update(device).await {
            // chrono::DateTime::<chrono::Utc>::new(
            // )
        }

        embassy_time::Timer::after_millis(10).await;

        Ok(None)
    }
}
