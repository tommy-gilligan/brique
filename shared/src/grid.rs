use core::ops::{Index, IndexMut};

use embedded_graphics::{
    Drawable,
    image::{Image, ImageDrawable},
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Primitive, Size},
    primitives::{PrimitiveStyle, Rectangle},
    transform::Transform,
};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Row<T, const COLS: usize>(pub [Option<T>; COLS]);

impl<T, const COLS: usize> Default for Row<T, COLS>
where
    T: Copy + ImageDrawable<Color = BinaryColor>,
{
    fn default() -> Self {
        Self([const { None }; COLS])
    }
}

pub struct Grid<T, const ROWS: usize, const COLS: usize> {
    rows: [Row<T, COLS>; ROWS],
    rng: fastrand::Rng,
    translation: Point,
}

impl<T, const ROWS: usize, const COLS: usize> Default for Grid<T, ROWS, COLS>
where
    T: Copy + ImageDrawable<Color = BinaryColor>,
{
    fn default() -> Self {
        Self::new(0)
    }
}

impl<T, const ROWS: usize, const COLS: usize> Grid<T, ROWS, COLS>
where
    T: Copy + ImageDrawable<Color = BinaryColor>,
{
    pub fn new(seed: u64) -> Self {
        Grid {
            rows: [Row::default(); ROWS],
            rng: fastrand::Rng::with_seed(seed),
            translation: Point::default(),
        }
    }

    pub fn neighbour_index(
        &self,
        location: (usize, usize),
        direction: Direction,
    ) -> (usize, usize) {
        match direction {
            Direction::Up => {
                if location.0 == 0 {
                    (self.rows.len() - 1, location.1)
                } else {
                    (location.0 - 1, location.1)
                }
            }
            Direction::Down => {
                if location.0 == (self.rows.len() - 1) {
                    (0, location.1)
                } else {
                    (location.0 + 1, location.1)
                }
            }
            Direction::Right => {
                if location.1 == (self.rows[0].0.len() - 1) {
                    (location.0, 0)
                } else {
                    (location.0, location.1 + 1)
                }
            }
            Direction::Left => {
                if location.1 == 0 {
                    (location.0, self.rows[0].0.len() - 1)
                } else {
                    (location.0, location.1 - 1)
                }
            }
        }
    }

    fn random(&mut self) -> Option<(usize, usize)> {
        let ys = 0..ROWS;
        let xs = 0..COLS;
        let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));
        let empty_cells = cross.filter(|(x, y)| self.rows[*y].0[*x].is_none());

        let count = 0..(empty_cells.clone().count());

        let i = self.rng.usize(count);
        empty_cells
            .enumerate()
            .find(|(x, _)| *x == i)
            .map(|(_, x)| x)
    }

    pub fn place_randomly(&mut self, cell: T) {
        if let Some((column_index, row_index)) = self.random() {
            self.rows[row_index].0[column_index] = Some(cell);
        }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Index<(usize, usize)> for Grid<T, ROWS, COLS>
where
    T: Copy + ImageDrawable<Color = BinaryColor>,
{
    type Output = Option<T>;

    fn index(&self, (row_index, column_index): (usize, usize)) -> &Self::Output {
        &self.rows[row_index].0[column_index]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<(usize, usize)> for Grid<T, ROWS, COLS>
where
    T: Copy + ImageDrawable<Color = BinaryColor>,
{
    fn index_mut(&mut self, (row_index, column_index): (usize, usize)) -> &mut Self::Output {
        &mut self.rows[row_index].0[column_index]
    }
}

impl<T, const ROWS: usize, const COLS: usize> Drawable for Grid<T, ROWS, COLS>
where
    T: Copy + ImageDrawable<Color = BinaryColor>,
{
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        for (row_index, row) in self.rows.iter().enumerate() {
            for (column_index, cell) in row.0.iter().enumerate() {
                match cell {
                    Some(drawable) => {
                        let _ = Image::new(
                            drawable,
                            Point::new(
                                <usize as TryInto<i32>>::try_into(column_index).unwrap() * 4
                                    + self.translation.x,
                                <usize as TryInto<i32>>::try_into(row_index).unwrap() * 4
                                    + self.translation.y,
                            ),
                        )
                        .draw(target);
                    }
                    None => {
                        let _ = Rectangle::new(
                            Point::new(
                                <usize as TryInto<i32>>::try_into(column_index).unwrap() * 4
                                    + self.translation.x,
                                <usize as TryInto<i32>>::try_into(row_index).unwrap() * 4
                                    + self.translation.y,
                            ),
                            Size::new(4, 4),
                        )
                        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                        .draw(target);
                    }
                }
            }
        }

        Ok(())
    }
}

impl<T, const ROWS: usize, const COLS: usize> Transform for Grid<T, ROWS, COLS>
where
    T: Copy + ImageDrawable<Color = BinaryColor>,
{
    fn translate(&self, by: Point) -> Self {
        Grid {
            // TODO: rows should not be copy
            rows: self.rows,
            rng: self.rng.clone(),
            translation: by,
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.translation = by;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum Cell {
        Nought,
        Cross,
    }

    use embedded_graphics::{
        geometry::{OriginDimensions, Size},
        image::ImageRaw,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        prelude::*,
    };

    #[rustfmt::skip]
    const DATA: &[u8] = &[
        0b1001_0110,
        0b0110_1001,
        0b0110_1001,
        0b1001_0110,
    ];

    impl ImageDrawable for Cell {
        type Color = BinaryColor;

        fn draw<
            D: embedded_graphics::draw_target::DrawTarget<
                    Color = <Self as embedded_graphics::image::ImageDrawable>::Color,
                >,
        >(
            &self,
            display: &mut D,
        ) -> Result<(), <D as embedded_graphics::draw_target::DrawTarget>::Error> {
            let raw: ImageRaw<BinaryColor> = ImageRaw::new(DATA, 8);
            match self {
                Cell::Nought => {
                    raw.sub_image(&Rectangle::new(Point::new(0, 0), Size::new(4, 4)))
                        .draw(display);
                }
                Cell::Cross => {
                    raw.sub_image(&Rectangle::new(Point::new(4, 0), Size::new(4, 4)))
                        .draw(display);
                }
            }
            Ok(())
        }

        fn draw_sub_image<
            D: embedded_graphics::draw_target::DrawTarget<
                    Color = <Self as embedded_graphics::image::ImageDrawable>::Color,
                >,
        >(
            &self,
            _: &mut D,
            _: &Rectangle,
        ) -> Result<(), <D as embedded_graphics::draw_target::DrawTarget>::Error> {
            Ok(())
        }
    }

    impl OriginDimensions for Cell {
        fn size(&self) -> Size {
            Size::new(4, 4)
        }
    }

    #[test]
    fn test_draw() {
        let mut grid: crate::grid::Grid<Cell, 3, 3> = Grid::new(0);
        grid[(0, 0)] = Some(Cell::Cross);
        grid[(0, 1)] = Some(Cell::Nought);

        let mut display = MockDisplay::new();
        grid.draw(&mut display);

        display.assert_pattern(&[
            ".##.#..#####",
            "#..#.##.####",
            "#..#.##.####",
            ".##.#..#####",
            "############",
            "############",
            "############",
            "############",
            "############",
            "############",
            "############",
            "############",
        ]);
    }

    #[test]
    fn test_neighbour_index() {
        let grid: crate::grid::Grid<Cell, 9, 20> = Grid::new(0);

        // indices are row, column
        // middle-ish
        assert_eq!(grid.neighbour_index((4, 10), Direction::Up), (3, 10),);
        assert_eq!(grid.neighbour_index((4, 10), Direction::Left), (4, 9),);
        assert_eq!(grid.neighbour_index((4, 10), Direction::Down), (5, 10),);
        assert_eq!(grid.neighbour_index((4, 10), Direction::Right), (4, 11),);

        // left edge
        assert_eq!(grid.neighbour_index((4, 0), Direction::Up), (3, 0),);
        assert_eq!(grid.neighbour_index((4, 0), Direction::Left), (4, 19),);
        assert_eq!(grid.neighbour_index((4, 0), Direction::Down), (5, 0),);
        assert_eq!(grid.neighbour_index((4, 0), Direction::Right), (4, 1),);

        // right edge
        assert_eq!(grid.neighbour_index((4, 19), Direction::Up), (3, 19),);
        assert_eq!(grid.neighbour_index((4, 19), Direction::Left), (4, 18),);
        assert_eq!(grid.neighbour_index((4, 19), Direction::Down), (5, 19),);
        assert_eq!(grid.neighbour_index((4, 19), Direction::Right), (4, 0),);

        // top edge
        assert_eq!(grid.neighbour_index((0, 10), Direction::Up), (8, 10),);
        assert_eq!(grid.neighbour_index((0, 10), Direction::Left), (0, 9),);
        assert_eq!(grid.neighbour_index((0, 10), Direction::Down), (1, 10),);
        assert_eq!(grid.neighbour_index((0, 10), Direction::Right), (0, 11),);

        // bottom edge
        assert_eq!(grid.neighbour_index((8, 10), Direction::Up), (7, 10),);
        assert_eq!(grid.neighbour_index((8, 10), Direction::Left), (8, 9),);
        assert_eq!(grid.neighbour_index((8, 10), Direction::Down), (0, 10),);
        assert_eq!(grid.neighbour_index((8, 10), Direction::Right), (8, 11),);
    }

    #[test]
    fn test_small_neighbour_index() {
        let grid: crate::grid::Grid<Cell, 3, 3> = Grid::new(0);

        // indices are row, column
        // middle-ish
        assert_eq!(grid.neighbour_index((1, 1), Direction::Up), (0, 1),);
        assert_eq!(grid.neighbour_index((1, 1), Direction::Left), (1, 0),);
        assert_eq!(grid.neighbour_index((1, 1), Direction::Down), (2, 1),);
        assert_eq!(grid.neighbour_index((1, 1), Direction::Right), (1, 2),);

        // left edge
        assert_eq!(grid.neighbour_index((1, 0), Direction::Up), (0, 0),);
        assert_eq!(grid.neighbour_index((1, 0), Direction::Left), (1, 2),);
        assert_eq!(grid.neighbour_index((1, 0), Direction::Down), (2, 0),);
        assert_eq!(grid.neighbour_index((1, 0), Direction::Right), (1, 1),);

        // right edge
        assert_eq!(grid.neighbour_index((1, 2), Direction::Up), (0, 2),);
        assert_eq!(grid.neighbour_index((1, 2), Direction::Left), (1, 1),);
        assert_eq!(grid.neighbour_index((1, 2), Direction::Down), (2, 2),);
        assert_eq!(grid.neighbour_index((1, 2), Direction::Right), (1, 0),);

        // top edge
        assert_eq!(grid.neighbour_index((0, 1), Direction::Up), (2, 1),);
        assert_eq!(grid.neighbour_index((0, 1), Direction::Left), (0, 0),);
        assert_eq!(grid.neighbour_index((0, 1), Direction::Down), (1, 1),);
        assert_eq!(grid.neighbour_index((0, 1), Direction::Right), (0, 2),);

        // bottom edge
        assert_eq!(grid.neighbour_index((2, 1), Direction::Up), (1, 1),);
        assert_eq!(grid.neighbour_index((2, 1), Direction::Left), (2, 0),);
        assert_eq!(grid.neighbour_index((2, 1), Direction::Down), (0, 1),);
        assert_eq!(grid.neighbour_index((2, 1), Direction::Right), (2, 2),);
    }

    #[test]
    fn test_place_randomly() {
        let mut grid: crate::grid::Grid<Cell, 3, 3> = Grid::new(0);
        grid.place_randomly(Cell::Cross);
        assert_eq!(grid[(0, 0)], None);
        assert_eq!(grid[(0, 1)], None);
        assert_eq!(grid[(0, 2)], None);
        assert_eq!(grid[(1, 0)], None);
        assert_eq!(grid[(1, 1)], None);
        assert_eq!(grid[(2, 2)], None);
        assert_eq!(grid[(2, 0)], None);
        assert_eq!(grid[(2, 1)], None);
        assert_eq!(grid[(1, 2)], Some(Cell::Cross));
    }

    #[test]
    fn test_place_randomly_other_seed() {
        let mut grid: crate::grid::Grid<Cell, 3, 3> = Grid::new(2);
        grid.place_randomly(Cell::Cross);
        assert_eq!(grid[(0, 0)], None);
        assert_eq!(grid[(0, 1)], Some(Cell::Cross));
        assert_eq!(grid[(0, 2)], None);
        assert_eq!(grid[(1, 0)], None);
        assert_eq!(grid[(1, 1)], None);
        assert_eq!(grid[(2, 2)], None);
        assert_eq!(grid[(2, 0)], None);
        assert_eq!(grid[(2, 1)], None);
        assert_eq!(grid[(1, 2)], None);
    }
}
