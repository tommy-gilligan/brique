use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    image::{ImageDrawable, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle,
};
use shared::grid::Direction;
#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Critter(Direction),
    Food,
}

impl OriginDimensions for Cell {
    fn size(&self) -> Size {
        Size::new(4, 4)
    }
}

impl ImageDrawable for Cell {
    type Color = BinaryColor;

    fn draw<D: DrawTarget<Color = Self::Color>>(&self, display: &mut D) -> Result<(), D::Error> {
        let top_left = match self {
            Cell::Critter(Direction::Up) => Point::new(4, 0),
            Cell::Critter(Direction::Down) => Point::new(4, 0),
            Cell::Critter(Direction::Left) => Point::new(0, 0),
            Cell::Critter(Direction::Right) => Point::new(0, 0),
            Cell::Food => Point::new(0, 20),
        };
        let raw: ImageRaw<BinaryColor> = ImageRaw::new(DATA, 8);
        let _ = raw
            .sub_image(&Rectangle::new(top_left, self.size()))
            .draw(display);
        Ok(())
    }

    fn draw_sub_image<D: DrawTarget<Color = Self::Color>>(
        &self,
        _display: &mut D,
        _: &Rectangle,
    ) -> Result<(), D::Error> {
        todo!()
    }
}

#[rustfmt::skip]
const DATA: &[u8] = &[
    // body
    0b1111_1001,
    0b0000_1001,
    0b0000_1001,
    0b1111_1001,

    0b1111_1111,
    0b1000_0001,
    0b1000_0001,
    0b1001_1001,

    0b1001_1001,
    0b1000_1001,
    0b1000_0001,
    0b1111_1111,

    // head
    0b1001_1011,
    0b1001_0100,
    0b0101_0000,
    0b1001_1111,

    0b1101_1001,
    0b0010_0101,
    0b0000_1001,
    0b1111_1001,

    // food
    0b1011_1111,
    0b0101_1111,
    0b1011_1111,
    0b1111_1111,
];

#[cfg(test)]
mod test {
    use embedded_graphics::mock_display::MockDisplay;
    use shared::grid::Grid;

    use super::*;

    #[test]
    fn test_draw() {
        let mut grid: Grid<Cell, 3, 3> = Grid::new(0);
        grid[(1, 0)] = Some(Cell::Critter(Direction::Right));
        grid[(1, 1)] = Some(Cell::Critter(Direction::Right));
        grid.place_randomly(Cell::Food);

        let mut display = MockDisplay::new();
        let _ = grid.draw(&mut display);

        display.assert_pattern(&[
            "############",
            "############",
            "############",
            "############",
            "############",
            "........####",
            "........####",
            "############",
            "#.##########",
            ".#.#########",
            "#.##########",
            "############",
        ]);
    }
}
