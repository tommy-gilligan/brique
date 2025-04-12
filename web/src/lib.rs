#![allow(unexpected_cfgs)]
mod device;
mod power;
mod system_request_handler;

use embassy_executor::Spawner;
use web_sys::Element;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_10X20};
use embedded_graphics::prelude::Point;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::text::Text;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTargetExt;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::prelude::Size;
use embedded_graphics::text::renderer::TextRenderer;
use shared::Buzzer;

pub struct CdcSend {
    rx: Rc<RefCell<Option<[u8; 64]>>>,
}

impl CdcSend {
    fn new(element: Element) -> Self {
        let rx = Rc::new(RefCell::new(None));
        let l = rx.clone();
        let change_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
            let input = event
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
            let mut buffer: [u8; 64] = [0; 64];
            for (a, b) in buffer.iter_mut().zip(input.value().as_bytes()) {
                *a = *b;
            }
            input.set_value("");

            *(*l).borrow_mut() = Some(buffer);
        });

        element
            .add_event_listener_with_callback("change", change_closure.as_ref().unchecked_ref())
            .unwrap();

        change_closure.forget();

        Self { rx }
    }
}

impl shared::SystemResponse for CdcSend {
    fn take(&mut self) -> Option<[u8; 64]> {
        let l = self.rx.clone();
        (*l).borrow_mut().take()
    }
}

#[derive(Clone, PartialEq, Copy)]
enum Inverted {
    On,
    Off
}

impl PixelColor for Inverted {
    type Raw = <BinaryColor as PixelColor>::Raw;
}

impl Into<BinaryColor> for Inverted {
    fn into(self) -> BinaryColor {
        match self {
            Inverted::On => BinaryColor::On,
            Inverted::Off => BinaryColor::Off,
        }
    }
}

impl From<BinaryColor> for Inverted {
    fn from(v: BinaryColor) -> Self {
        match v {
            BinaryColor::On => Inverted::Off,
            BinaryColor::Off => Inverted::On,
        }
    }
}

// (draw_target.color_converted::<Inverted>()).color_converted::<BinaryColor>()

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    wasm_logger::init(wasm_logger::Config::default());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let mut device = device::Device::new(
        document.get_element_by_id("backlight").unwrap(),
        document.get_element_by_id("body").unwrap(),
        document.get_element_by_id("display").unwrap(),
        "cancel",
        "select",
        "up",
        "down",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "asterisk",
        "zero",
        "hash",
        document.get_element_by_id("svg1").unwrap(),
    );

    let mut power = power::DomPower::new();
    let hid_console = document.get_element_by_id("hid-console").unwrap();
    let cdc_console = document.get_element_by_id("cdc-console").unwrap();
    let mut system_request_handler = system_request_handler::Handler::new(hid_console, cdc_console);

    let mut system_response = CdcSend::new(document.get_element_by_id("cdc-console-rx").unwrap());

    let display_area = device.bounding_box();
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::Off);
    let text = Text::new("Select", Point::zero() + Point::new(0, 10), text_style);

    let rect = display_area.into_styled(PrimitiveStyle::with_fill(
        BinaryColor::On 
    ));

    let HAUNTED_HOUSE: &str = "HauntHouse: d=4,o=5,b=108: 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4, 1p, 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4";
    let RICH_MAN: &str = "Rich Man's World:o=6,d=8,b=112,b=112:e,e,e,e,e,e,16e5,16a5,16c,16e,d#,d#,d#,d#,d#,d#,16f5,16a5,16c,16d#,4d,c,a5,c,4c,2a5,32a5,32c,32e,a6";
    let WANNABE: &str = "Wannabe:o=5,d=8,b=125,b=125:16g,16g,16g,16g,g,a,g,e,p,16c,16d,16c,d,d,c,4e,4p,g,g,g,a,g,e,p,4c6,c6,b,g,a,16b,16a,4g";

    let mut items = [
        "A", "B", "C", "D", "E", "F", "G",
        "H", "I", "J", "K", "L", "M", "N",
        "O", "P", "Q", "R", "S", "T", "U",
    ];
    let mut menu = Menu::new(&mut items);
    loop {
        menu.process(&mut device).await;

      // let mut song = &mut items[0];// items[new_menu.run(&mut device, "Play").await].clone();

      // loop {
      //     if let Some(note) = song.item.next() {
      //         if let Some(frequency) = note.frequency() {
      //             device.unmute_buzzer().unwrap();
      //             log::debug!("Playing {}Hz", frequency.unwrap());
      //             device.set_frequency(frequency.unwrap() as u16).unwrap();
      //         } else {
      //             device.mute_buzzer().unwrap();
      //         }
      //         embassy_time::Timer::after_millis(note.duration().into()).await
      //     } else {
      //         device.mute_buzzer().unwrap();
      //         break;
      //     }
      // }
    }
}

use core::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{JsCast, closure::Closure};

struct DomB {
    was_clicked: bool,
    was_unclicked: bool,
}

#[derive(PartialEq)]
pub enum Event {
    Up,
    Down,
}

impl DomB {
    #[allow(clippy::too_many_arguments)]
    fn new(id: &'static str) -> Rc<RefCell<Self>> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let s = Self {
            was_clicked: false,
            was_unclicked: false,
        };
        let r = Rc::new(RefCell::new(s));
        let g = r.clone();
        let h = r.clone();

        let mouse_down_closure =
            Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
                (*g).borrow_mut().was_clicked = true;
            });
        let mouse_up_closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            (*h).borrow_mut().was_unclicked = true;
        });

        document
            .get_element_by_id(id)
            .unwrap()
            .add_event_listener_with_callback(
                "mousedown",
                mouse_down_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        document
            .get_element_by_id(id)
            .unwrap()
            .add_event_listener_with_callback("mouseup", mouse_up_closure.as_ref().unchecked_ref())
            .unwrap();

        mouse_down_closure.forget();
        mouse_up_closure.forget();

        r
    }

    fn check(&mut self) -> Option<Event> {
        let clicked_result = self.was_clicked;
        let unclicked_result = self.was_unclicked;
        self.was_clicked = false;
        self.was_unclicked = false;
        if clicked_result {
            Some(Event::Down)
        } else if unclicked_result {
            Some(Event::Up)
        } else {
            None
        }
    }
}

struct Menu<'a, T> where T: AsRef<str> {
    items: &'a mut [T],
    index: usize,
    start_of_page_index: usize,
    bottom_visible_index: usize,
    page_size: usize
}

impl <'a, T>Menu<'a, T> where T: AsRef<str> {
    pub fn new(items: &'a mut [T]) -> Self {
        assert!(items.len() > 0);
        Self {
            items,
            index: 0,
            start_of_page_index: 0,
            bottom_visible_index: 0,
            page_size: 0
        }
    }

    fn down(&mut self) {
        if self.index < self.items.len() - 1 {
            self.index += 1;
        }
        if self.index > self.bottom_visible_index {
            self.page_size = self.index - self.start_of_page_index;
            self.start_of_page_index = self.index;
        }
    }

    fn up(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
        if self.index < self.start_of_page_index {
            self.start_of_page_index = self.index.saturating_sub(self.page_size - 1);
        }
    }

    fn text_style(&self, selected: bool) -> MonoTextStyle<'_, BinaryColor> {
        embedded_graphics::mono_font::MonoTextStyleBuilder::new()
            .text_color(if selected { BinaryColor::On } else { BinaryColor::Off }).font(&FONT_6X10).build()
    }
}

fn row_render(draw_target: &mut impl DrawTarget<Color = BinaryColor>, selected: bool, thing: &str, point: Point, text_style: MonoTextStyle<'_, BinaryColor>) -> Option<Point> {
    let text_bounding_box = text_style.measure_string(
        thing,
        point,
        embedded_graphics::text::Baseline::Top
    ).bounding_box;

    if draw_target.bounding_box().contains(
        Point::new(
            0,
            text_bounding_box.bottom_right().unwrap().y
        )
    ) {
        Rectangle::new(
            point,
            Size::new(
                draw_target.bounding_box().size.width,
                text_bounding_box.size.height
            )
        ).into_styled(PrimitiveStyle::with_fill(
            if selected {
                BinaryColor::Off
            } else {
                BinaryColor::On
            }
        )).draw(draw_target);

        Text::with_baseline(thing, point, text_style, embedded_graphics::text::Baseline::Top).draw(draw_target);

        Some(Point::new(point.x, point.y + text_bounding_box.size.height as i32))
    } else {
        None
    }
}

impl <'a, T>embedded_graphics::Drawable for Menu<'a, T> where T: AsRef<str> {
    type Color = BinaryColor;
    type Output = usize;

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, <D as DrawTarget>::Error> where D: DrawTarget<Color = Self::Color> {
        target.bounding_box().into_styled(PrimitiveStyle::with_fill(BinaryColor::On)).draw(target);

        let mut last_drawn_index = 0;
        let mut point = Point::zero();
        for (index, a) in self.items[self.start_of_page_index..].iter().enumerate() {
            if let Some(p) = row_render(&mut target.clipped(&target.bounding_box()), (self.index - self.start_of_page_index) == index, a.as_ref(), point, self.text_style((self.index - self.start_of_page_index) == index)) {
                point = p;
                last_drawn_index = index;
            }
        }

        Ok(last_drawn_index + self.start_of_page_index)
    }
}


impl <'a, T>Menu<'a, T> where T: AsRef<str>, Menu<'a, T>: embedded_graphics::Drawable<Color = BinaryColor, Output = usize> {
    pub async fn process(&mut self, device: &mut impl shared::Device) -> &mut T {
        loop {
            match self.draw(device) {
                Ok(point) => {
                    self.bottom_visible_index = point;
                    match device.event().await {
                        shared::KeyEvent::Down(shared::Key::Down) => {
                            self.down()
                        },
                        shared::KeyEvent::Down(shared::Key::Up) => {
                            self.up()
                        },
                        shared::KeyEvent::Down(shared::Key::Select) => {
                            return &mut self.items[self.index];
                        },
                        _ => {
                        },
                    }
                },
                _ => {}
            }
        }
    }
}
