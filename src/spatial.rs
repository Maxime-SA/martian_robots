#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}
impl Orientation {
    fn forward_delta(&self) -> (isize, isize) {
        match self {
            Orientation::East => (1, 0),
            Orientation::West => (-1, 0),
            Orientation::North => (0, 1),
            Orientation::South => (0, -1),
        }
    }

    fn left(&self) -> Self {
        match self {
            Orientation::East => Orientation::North,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
            Orientation::North => Orientation::West,
        }
    }

    fn right(&self) -> Self {
        match self {
            Orientation::East => Orientation::South,
            Orientation::West => Orientation::North,
            Orientation::South => Orientation::West,
            Orientation::North => Orientation::East,
        }
    }
}
impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Orientation::East => "E",
                Orientation::North => "N",
                Orientation::South => "S",
                Orientation::West => "W",
            }
        )
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
}
impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn add(&self, dx: isize, dy: isize) -> Option<Self> {
        self.x
            .checked_add_signed(dx)
            .and_then(|x| self.y.checked_add_signed(dy).map(|y| Self { x, y }))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Position {
    coordinate: Coordinate,
    orientation: Orientation,
}
impl Position {
    pub fn new(x: usize, y: usize, orientation: Orientation) -> Self {
        Self {
            coordinate: Coordinate::new(x, y),
            orientation,
        }
    }

    pub fn coord(&self) -> Coordinate {
        self.coordinate
    }

    pub fn with_orientation(&self, orientation: Orientation) -> Self {
        Self {
            coordinate: self.coordinate,
            orientation,
        }
    }

    pub fn with_coordinate(&self, coordinate: Coordinate) -> Self {
        Self {
            coordinate,
            orientation: self.orientation,
        }
    }
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.coordinate.x, self.coordinate.y, self.orientation
        )
    }
}

pub struct Grid {
    ncols: usize,
    nrows: usize,
}
impl Grid {
    pub fn new(max_col: usize, max_row: usize) -> Self {
        let ncols = max_col + 1;
        let nrows = max_row + 1;

        Self { ncols, nrows }
    }

    pub fn pivot_left(&self, position: Position) -> Position {
        let new_orientation = position.orientation.left();
        position.with_orientation(new_orientation)
    }

    pub fn pivot_right(&self, position: Position) -> Position {
        let new_orientation = position.orientation.right();
        position.with_orientation(new_orientation)
    }

    pub fn contains(&self, position: Position) -> bool {
        self.map_coord(&position.coord()).is_some()
    }

    pub fn move_forward(&self, position: Position) -> Option<Position> {
        let (dx, dy) = position.orientation.forward_delta();

        let new_coord = position.coordinate.add(dx, dy)?;

        self.map_coord(&new_coord)
            .map(|_| position.with_coordinate(new_coord))
    }

    fn map_coord(&self, coord: &Coordinate) -> Option<usize> {
        /*
           Primarily used as a Coordinate validation function which maps 2D to 1D indices

           Coordinate assumes:
           - bottom-left is (0, 0)
           - top-right is (ncols - 1, nrows - 1)

           Gird assumes:
           - top-left is (0, 0)
           - bottom-right is (ncols - 1, nrows - 1)

           Mapping fn:
           - coord.x -> coord.x (identity function)
           - coord.y -> (nrows - coord.y - 1)
        */
        let y = self.nrows.checked_sub_signed(coord.y as isize + 1)?;

        if coord.x < self.ncols && y < self.nrows {
            Some(self.ncols * y + coord.x)
        } else {
            None
        }
    }

    // practical for debugging
    #[allow(dead_code)]
    fn print(&mut self, position: Position) {
        let target_i = self
            .map_coord(&position.coordinate)
            .expect("position needs to be within Grid dimensions for printing");

        for row in (0..self.nrows).rev() {
            for col in 0..self.ncols {
                let i = self.map_coord(&Coordinate::new(col, row)).unwrap();

                if i == target_i {
                    let token = match position.orientation {
                        Orientation::East => "\u{2192}",
                        Orientation::North => "\u{2191}",
                        Orientation::South => "\u{2193}",
                        Orientation::West => "\u{2190}",
                    };
                    print!("\x1b[32m{token}\x1b[0m");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Orientation Tests
    #[test]
    fn test_orientation_forward_delta() {
        let cases = [
            (Orientation::North, (0, 1)),
            (Orientation::South, (0, -1)),
            (Orientation::East, (1, 0)),
            (Orientation::West, (-1, 0)),
        ];

        for (orientation, expected_result) in cases {
            assert_eq!(expected_result, orientation.forward_delta());
        }
    }

    #[test]
    fn test_orientation_left() {
        let cases = [
            (Orientation::North, Orientation::West),
            (Orientation::West, Orientation::South),
            (Orientation::South, Orientation::East),
            (Orientation::East, Orientation::North),
        ];

        for (actual_result, expected_result) in cases {
            assert_eq!(expected_result, actual_result.left());
        }
    }

    #[test]
    fn test_orientation_right() {
        let cases = [
            (Orientation::North, Orientation::East),
            (Orientation::East, Orientation::South),
            (Orientation::South, Orientation::West),
            (Orientation::West, Orientation::North),
        ];

        for (actual_result, expected_result) in cases {
            assert_eq!(expected_result, actual_result.right());
        }
    }

    // Coordinate Tests
    #[test]
    fn test_coordinate_add_valid() {
        let cases = [
            ((2, 3), (1, 0), (3, 3)),
            ((2, 3), (0, 1), (2, 4)),
            ((2, 3), (-1, 0), (1, 3)),
            ((2, 3), (0, -1), (2, 2)),
            ((2, 3), (0, 0), (2, 3)),
            ((1, 1), (-1, -1), (0, 0)),
        ];

        for ((x, y), (dx, dy), (expected_x, expected_y)) in cases {
            let expected_result = Some(Coordinate::new(expected_x, expected_y));
            let actual_result = Coordinate::new(x, y).add(dx, dy);

            assert_eq!(expected_result, actual_result);
        }
    }

    #[test]
    fn test_coordinate_add_negative_result() {
        let cases = [
            ((0, 0), (-1, 0)),
            ((0, 0), (0, -1)),
            ((0, 0), (-1, -1)),
            ((1, 1), (-2, 0)),
            ((1, 1), (0, -2)),
            ((3, 0), (1, -1)),
        ];

        for ((x, y), (dx, dy)) in cases {
            let actual_result = Coordinate::new(x, y).add(dx, dy);

            assert_eq!(None, actual_result);
        }
    }

    // Grid Tests
    #[test]
    fn test_grid_pivot_left() {
        let grid = Grid::new(5, 3);

        let cases = [
            (Orientation::North, Orientation::West),
            (Orientation::West, Orientation::South),
            (Orientation::South, Orientation::East),
            (Orientation::East, Orientation::North),
        ];

        for (orientation, expected_orientation) in cases {
            let expected_result = Position::new(2, 1, expected_orientation);
            let actual_result = grid.pivot_left(Position::new(2, 1, orientation));

            assert_eq!(expected_result, actual_result);
        }
    }

    #[test]
    fn test_grid_pivot_right() {
        let grid = Grid::new(5, 3);

        let cases = [
            (Orientation::North, Orientation::East),
            (Orientation::East, Orientation::South),
            (Orientation::South, Orientation::West),
            (Orientation::West, Orientation::North),
        ];

        for (orientation, expected_orientation) in cases {
            let expected_result = Position::new(2, 1, expected_orientation);
            let actual_result = grid.pivot_right(Position::new(2, 1, orientation));

            assert_eq!(expected_result, actual_result);
        }
    }

    #[test]
    fn test_grid_contains() {
        let grid = Grid::new(5, 3);

        let cases = [
            ((0, 0), true),
            ((5, 3), true),
            ((5, 0), true),
            ((0, 3), true),
            ((2, 1), true),
            ((6, 0), false),
            ((0, 4), false),
            ((6, 4), false),
            ((100, 100), false),
        ];

        for ((x, y), expected_result) in cases {
            let actual_result = grid.contains(Position::new(x, y, Orientation::North));

            assert_eq!(expected_result, actual_result);
        }
    }

    #[test]
    fn test_grid_move_forward() {
        let grid = Grid::new(5, 3);

        let cases = [
            // valid
            ((2, 1), Orientation::North, Some((2, 2))),
            ((2, 1), Orientation::South, Some((2, 0))),
            ((2, 1), Orientation::East, Some((3, 1))),
            ((2, 1), Orientation::West, Some((1, 1))),
            ((1, 2), Orientation::North, Some((1, 3))),
            ((1, 1), Orientation::South, Some((1, 0))),
            ((4, 1), Orientation::East, Some((5, 1))),
            ((1, 1), Orientation::West, Some((0, 1))),
            ((5, 2), Orientation::North, Some((5, 3))),
            // invalid
            ((1, 3), Orientation::North, None),
            ((1, 0), Orientation::South, None),
            ((5, 1), Orientation::East, None),
            ((0, 1), Orientation::West, None),
        ];

        for ((x, y), orientation, expected_coord) in cases {
            let expected_result =
                expected_coord.map(|(new_x, new_y)| Position::new(new_x, new_y, orientation));

            let actual_result = grid.move_forward(Position::new(x, y, orientation));

            assert_eq!(expected_result, actual_result);
        }
    }
}
