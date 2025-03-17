use embassy_futures::select::Either;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    image::ImageDrawable,
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use numtoa::NumToA;

use crate::{
    grid::{Direction, Grid},
    keypad::{Button, Event, Keypad},
};
mod cell;
use cell::Cell;

pub struct World<const ROWS: usize, const COLS: usize> {
    pub grid: Grid<Cell, ROWS, COLS>,
}

pub struct Snake<KEYPAD, DRAW_TARGET>
where
    KEYPAD: Keypad,
    DRAW_TARGET: DrawTarget<Color = BinaryColor>,
{
    keypad: KEYPAD,
    draw_target: DRAW_TARGET,
    grid: Grid<Cell, 9, 20>,
    score: u16,
    first_draw: bool,
}

impl<KEYPAD, DRAW_TARGET> Snake<KEYPAD, DRAW_TARGET>
where
    KEYPAD: Keypad,
    DRAW_TARGET: DrawTarget<Color = BinaryColor>,
{
    pub fn new(keypad: KEYPAD, draw_target: DRAW_TARGET, seed: u64) -> Self {
        let mut grid = Grid::new(seed);
        grid[(0, 0)] = Some(Cell::Food);
        grid[(1, 0)] = Some(Cell::Food);
        grid[(1, 1)] = Some(Cell::Food);

        Self {
            keypad,
            draw_target,
            grid,
            score: 0,
            first_draw: true,
        }
    }

    fn draw(&mut self) {
        if self.first_draw {
            self.first_draw = false;
            self.draw_border();
            self.draw_score();
        }
        let _ = self
            .grid
            .translate(Point::new(2, 10))
            .draw(&mut self.draw_target);
    }

    fn draw_score(&mut self) {
        let mut buffer = [0u8; 5];

        let style = MonoTextStyle::new(&FONT_4X6, BinaryColor::Off);
        let s: &str = core::str::from_utf8(self.score.numtoa(10, &mut buffer)).unwrap();

        let t = Text::new(s, Point::new(1, 4), style);
        let fill = PrimitiveStyle::with_fill(BinaryColor::On);
        let _ = t
            .bounding_box()
            .into_styled(fill)
            .draw(&mut self.draw_target);
        let _ = t.draw(&mut self.draw_target);
    }

    fn draw_border(&mut self) {
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::Off, 1);

        let _ = Rectangle::new(Point::new(0, 8), Size::new(84, 40))
            .into_styled(thin_stroke)
            .draw(&mut self.draw_target);

        let _ = Line::new(Point::new(0, 6), Point::new(83, 6))
            .into_styled(thin_stroke)
            .draw(&mut self.draw_target);
    }

    pub async fn process(&mut self) {
        self.draw();
        let event_future = self.keypad.event();
        let timeout_future = embassy_time::Timer::after_millis(100);

        let direction = match embassy_futures::select::select(event_future, timeout_future).await {
            Either::First(Event::Down(Button::Two)) => Direction::Up,
            Either::First(Event::Down(Button::Four)) => Direction::Left,
            Either::First(Event::Down(Button::Six)) => Direction::Right,
            Either::First(Event::Down(Button::Eight)) => Direction::Down,
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
    }

    fn release(self) -> DRAW_TARGET {
        self.draw_target
    }
}

#[cfg(test)]
mod test {
    use embedded_graphics::mock_display::MockDisplay;

    use super::*;
    struct TestKeypad;

    impl Keypad for TestKeypad {
        async fn event(&mut self) -> Event<Button> {
            embassy_time::Timer::after_millis(100).await;
            Event::Down(Button::Down)
        }
    }

    #[test]
    fn test_draw() {
        let mut display = MockDisplay::new();
        display.set_allow_out_of_bounds_drawing(true);
        // TODO: set false to find overdraws
        display.set_allow_overdraw(true);
        let mut snake = Snake::new(TestKeypad, display, 0);
        snake.grid.place_randomly(Cell::Food);
        snake.draw();

        snake.release().assert_pattern(&[
            " #.##                                                           ",
            " .#.#                                                           ",
            " ...#                                                           ",
            " .#.#                                                           ",
            " #.##                                                           ",
            " ####                                                           ",
            "................................................................",
            "                                                                ",
            "................................................................",
            ".                                                               ",
            ". #.############################################################",
            ". .#.###########################################################",
            ". #.############################################################",
            ". ##############################################################",
            ". #.###.########################################################",
            ". .#.#.#.#######################################################",
            ". #.###.########################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". #####################################.########################",
            ". ####################################.#.#######################",
            ". #####################################.########################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ". ##############################################################",
            ".                                                               ",
            "................................................................",
        ]);
    }
}
