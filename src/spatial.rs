use std::collections::HashMap;
use std::fmt::write;

use crate::error::{MarsError, Result};
use crate::robot::RobotId;

#[derive(Clone, Copy)]
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
            Orientation::South => (0, 1),
            Orientation::North => (0, -1),
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

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Coordinate {
    x: usize, // could be u8 since the max value is 50 but better DevEx to use usize
    y: usize,
}
impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
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

#[derive(Clone, Copy)]
pub struct Position {
    coordinate: Coordinate,
    orientation: Orientation,
}
impl Position {
    fn new(coordinate: Coordinate, orientation: Orientation) -> Self {
        Self {
            coordinate,
            orientation,
        }
    }
}

// talk about how we could make this generic over RobotId
pub struct Grid {
    nrows: usize,
    ncols: usize,
    grid: Vec<Option<()>>, // chose 1D for better cache efficiency - this could also just be a hashset
    positions: HashMap<RobotId, Position>,
}
impl Grid {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Self {
            nrows,
            ncols,
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
            self.grid[i] = Some(());
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
            self.grid[new_i] = Some(());

            Ok(())
        }
    }

    pub fn get_coord(&self, id: &RobotId) -> Result<&Coordinate> {
        self.positions
            .get(id)
            .ok_or(MarsError::RobotNotFound)
            .map(|position| &position.coordinate)
    }

    fn map_coord(&self, coord: &Coordinate) -> Result<usize> {
        if coord.x < self.ncols && coord.y < self.nrows {
            Ok(self.ncols * coord.y + coord.x)
        } else {
            Err(MarsError::CoordinateOutOfBounds)
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                let i = self.map_coord(&Coordinate::new(row, col)).unwrap();
                if self.grid[i].is_some() {
                    write!(f, "\x1b[32mR\x1b[0m")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}
