#![no_std]

use embassy_futures::select::Either;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    mono_font::{MonoTextStyle, ascii::FONT_4X6},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use numtoa::NumToA;
use shared::{
    Application, Key, KeyEvent,
    grid::{Direction, Grid},
};
mod cell;
use cell::Cell;

pub struct World<const ROWS: usize, const COLS: usize> {
    pub grid: Grid<Cell, ROWS, COLS>,
}

pub struct Snake {
    grid: Grid<Cell, 9, 20>,
    score: u16,
    first_draw: bool,
}

impl Snake {
    pub fn new(seed: u64) -> Self {
        let mut grid = Grid::new(seed);
        grid[(0, 0)] = Some(Cell::Food);
        grid[(1, 0)] = Some(Cell::Food);
        grid[(1, 1)] = Some(Cell::Food);

        Self {
            grid,
            score: 0,
            first_draw: true,
        }
    }

    fn draw<Display>(&mut self, draw_target: &mut Display)
    where
        Display: DrawTarget<Color = BinaryColor>,
    {
        if self.first_draw {
            self.first_draw = false;
            self.draw_border(draw_target);
            self.draw_score(draw_target);
        }
        let _ = self.grid.translate(Point::new(2, 10)).draw(draw_target);
    }

    fn draw_score<Display>(&mut self, draw_target: &mut Display)
    where
        Display: DrawTarget<Color = BinaryColor>,
    {
        let mut buffer = [0u8; 5];

        let style = MonoTextStyle::new(&FONT_4X6, BinaryColor::Off);
        let s: &str = core::str::from_utf8(self.score.numtoa(10, &mut buffer)).unwrap();

        let t = Text::new(s, Point::new(1, 4), style);
        let fill = PrimitiveStyle::with_fill(BinaryColor::On);
        let _ = t.bounding_box().into_styled(fill).draw(draw_target);
        let _ = t.draw(draw_target);
    }

    fn draw_border<Display>(&mut self, draw_target: &mut Display)
    where
        Display: DrawTarget<Color = BinaryColor>,
    {
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::Off, 1);

        let _ = Rectangle::new(Point::new(0, 8), Size::new(84, 40))
            .into_styled(thin_stroke)
            .draw(draw_target);

        let _ = Line::new(Point::new(0, 6), Point::new(83, 6))
            .into_styled(thin_stroke)
            .draw(draw_target);
    }
}

impl Application for Snake {
    async fn run(
        &mut self,
        device: &mut impl shared::Device,
        _system_response: Option<[u8; 64]>,
    ) -> Result<Option<shared::SystemRequest>, ()> {
        self.draw(device);
        let event_future = device.event();
        let timeout_future = embassy_time::Timer::after_millis(100);

        let _direction = match embassy_futures::select::select(event_future, timeout_future).await {
            Either::First(KeyEvent::Down(Key::Two)) => Direction::Up,
            Either::First(KeyEvent::Down(Key::Four)) => Direction::Left,
            Either::First(KeyEvent::Down(Key::Six)) => Direction::Right,
            Either::First(KeyEvent::Down(Key::Eight)) => Direction::Down,
            _ => {
                // if let Some(Cell::Critter(head_direction)) = self.grid[self.head_index]
                // {
                //     head_direction
                // } else {
                //     Direction::Down
                // }
                Direction::Down
            }
        };

        // updating the model should give a list of cells to redraw
        // let (cell_to_clear, body_now, collision) = self.update(direction);
        Ok(None)
    }
}

// #[cfg(test)]
// mod test {
//     use embedded_graphics::mock_display::MockDisplay;
//
//     use super::*;
//     struct TestKeypad;
//
//     impl Keypad for TestKeypad {
//         async fn event(&mut self) -> Event<Button> {
//             embassy_time::Timer::after_millis(100).await;
//             Event::Down(Button::Down)
//         }
//     }
//
//     #[test]
//     fn test_draw() {
//         let mut display = MockDisplay::new();
//         display.set_allow_out_of_bounds_drawing(true);
//         // TODO: set false to find overdraws
//         display.set_allow_overdraw(true);
//         let mut snake = Snake::new(TestKeypad, display, 0);
//         snake.grid.place_randomly(Cell::Food);
//         snake.draw();
//
//         snake.release().assert_pattern(&[
//             " #.##                                                           ",
//             " .#.#                                                           ",
//             " ...#                                                           ",
//             " .#.#                                                           ",
//             " #.##                                                           ",
//             " ####                                                           ",
//             "................................................................",
//             "                                                                ",
//             "................................................................",
//             ".                                                               ",
//             ". #.############################################################",
//             ". .#.###########################################################",
//             ". #.############################################################",
//             ". ##############################################################",
//             ". #.###.########################################################",
//             ". .#.#.#.#######################################################",
//             ". #.###.########################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". #####################################.########################",
//             ". ####################################.#.#######################",
//             ". #####################################.########################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ".                                                               ",
//             "................................................................",
//         ]);
//     }
// }
