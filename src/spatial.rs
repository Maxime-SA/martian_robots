use std::collections::HashMap;

use crate::error::{MarsError, Result};
use crate::robot::RobotId;

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
    x: usize, // could be u8 since the max value is 50 but better DevEx to use usize
    y: usize,
}
impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn add(&self, dx: isize, dy: isize) -> Result<Self> {
        let x = self
            .x
            .checked_add_signed(dx)
            .ok_or(MarsError::CoordinateOutOfBounds)?;

        let y = self
            .y
            .checked_add_signed(dy)
            .ok_or(MarsError::CoordinateOutOfBounds)?;

        Ok(Self { x, y })
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Position {
    coordinate: Coordinate,
    orientation: Orientation,
}
impl Position {
    pub fn new(coordinate: Coordinate, orientation: Orientation) -> Self {
        Self {
            coordinate,
            orientation,
        }
    }

    pub fn coord(&self) -> Coordinate {
        self.coordinate
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

// talk about how we could make this generic over RobotId
pub struct Grid {
    ncols: usize,
    nrows: usize,
    grid: Vec<Option<RobotId>>, // chose 1D for better cache efficiency - this could also just be a hashset
    positions: HashMap<RobotId, Position>,
}
impl Grid {
    pub fn new(max_col: usize, max_row: usize) -> Self {
        let ncols = max_col + 1;
        let nrows = max_row + 1;

        Self {
            ncols,
            nrows,
            grid: vec![None; nrows * ncols],
            positions: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: RobotId, position: Position) -> Result<()> {
        let i = self.map_coord(&position.coordinate)?;

        if self.grid[i].is_some() {
            Err(MarsError::RobotCollision)
        } else {
            self.positions.insert(id, position);
            self.grid[i] = Some(id);
            Ok(())
        }
    }

    pub fn remove(&mut self, id: &RobotId) -> Result<Position> {
        let position = self.positions.remove(id).ok_or(MarsError::RobotNotFound)?;
        let i = self.map_coord(&position.coordinate)?;
        self.grid[i] = None;
        Ok(position)
    }

    pub fn pivot_left(&mut self, id: &RobotId) -> Result<()> {
        let position = self.positions.get_mut(id).ok_or(MarsError::RobotNotFound)?;
        position.orientation = position.orientation.left();
        Ok(())
    }

    pub fn pivot_right(&mut self, id: &RobotId) -> Result<()> {
        let position = self.positions.get_mut(id).ok_or(MarsError::RobotNotFound)?;
        position.orientation = position.orientation.right();
        Ok(())
    }

    pub fn move_forward(&mut self, id: &RobotId) -> Result<()> {
        let position = self.positions.get(id).ok_or(MarsError::RobotNotFound)?;

        let (dx, dy) = position.orientation.forward_delta();
        let new_coord = position.coordinate.add(dx, dy)?;
        let new_i = self.map_coord(&new_coord)?;

        if self.grid[new_i].is_some() {
            Err(MarsError::RobotCollision)
        } else {
            let old_i = self.map_coord(&position.coordinate)?;
            self.grid[old_i] = None;

            *self.positions.get_mut(id).unwrap() = Position::new(new_coord, position.orientation);
            self.grid[new_i] = Some(id.clone());

            Ok(())
        }
    }

    pub fn get_position(&self, id: &RobotId) -> Result<Position> {
        self.positions
            .get(id)
            .cloned()
            .ok_or(MarsError::RobotNotFound)
    }

    fn map_coord(&self, coord: &Coordinate) -> Result<usize> {
        /*
           Grid representation:
           - bottom-left is (0, 0)
           - top-right is (ncols - 1, nrows - 1)

           coord.x maps to coord.x (identity function)
           coord.y maps to (nrows - coord.y - 1)
        */
        let y = self
            .nrows
            .checked_sub_signed(coord.y as isize + 1)
            .ok_or(MarsError::CoordinateOutOfBounds)?;

        if coord.x < self.ncols && y < self.nrows {
            Ok(self.ncols * y + coord.x)
        } else {
            Err(MarsError::CoordinateOutOfBounds)
        }
    }
}

// practical for debugging purposes
impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..self.nrows).rev() {
            for col in 0..self.ncols {
                let i = self.map_coord(&Coordinate::new(col, row)).unwrap();
                if let Some(id) = self.grid[i].as_ref() {
                    let token = match self
                        .get_position(id)
                        .map(|position| position.orientation)
                        .unwrap()
                    {
                        Orientation::East => "\u{2192}",
                        Orientation::North => "\u{2191}",
                        Orientation::South => "\u{2193}",
                        Orientation::West => "\u{2190}",
                    };
                    write!(f, "\x1b[32m{token}\x1b[0m")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}
