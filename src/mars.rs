use std::collections::{HashMap, HashSet};

use crate::error::{MarsError, Result};
use crate::robot::{Robot, RobotId};
use crate::spatial::{Coordinate, Grid, Position};

// we could make this dynamic at run-time
#[derive(Eq, PartialEq, Debug)]
pub enum Instruction {
    Left,
    Right,
    Forward,
}

#[derive(Eq, PartialEq, Debug)]
pub struct FinalRobotPosition {
    robot: Robot,
    position: Position,
}
impl FinalRobotPosition {
    fn new(robot: Robot, position: Position) -> Self {
        Self { robot, position }
    }
}
impl std::fmt::Display for FinalRobotPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.position)?;
        if self.robot.is_lost() {
            write!(f, " LOST")?;
        }
        Ok(())
    }
}

pub struct Mars {
    grid: Grid,
    robots: HashMap<RobotId, Robot>,
    lost_scents: HashSet<Coordinate>,
    next_id: u128, // would use UUID in the future
}
impl Mars {
    pub fn new(max_col: usize, max_row: usize) -> Self {
        Self {
            grid: Grid::new(max_col, max_row),
            robots: HashMap::new(),
            lost_scents: HashSet::new(),
            next_id: 0,
        }
    }

    pub fn create_robot(&mut self, position: Position) -> Result<RobotId> {
        let id = self.next_robot_id();

        let robot = Robot::new(id);

        self.grid.insert(id, position)?;

        self.robots.insert(id, robot);

        Ok(id)
    }

    pub fn execute_instructions(
        &mut self,
        id: &RobotId,
        instructions: &[Instruction],
    ) -> Result<FinalRobotPosition> {
        let robot = self.robots.get_mut(id).ok_or(MarsError::RobotNotFound)?;

        for instruction in instructions {
            match instruction {
                Instruction::Forward => {
                    if let Err(error) = self.grid.move_forward(id) {
                        match error {
                            MarsError::CoordinateOutOfBounds => {
                                let coord = self.grid.get_position(id)?.coord();

                                if !self.lost_scents.contains(&coord) {
                                    robot.set_lost_status();

                                    self.lost_scents.insert(coord);
                                    let last_position = self.grid.remove(id)?;

                                    return Ok(FinalRobotPosition::new(
                                        robot.clone(),
                                        last_position,
                                    ));
                                }
                            }
                            _ => return Err(error), // should we treat RobotCollision differently and simply ignore the command
                        }
                    }
                }
                Instruction::Left => self.grid.pivot_left(id)?,
                Instruction::Right => self.grid.pivot_right(id)?,
            }
        }

        let last_position = self.grid.get_position(id)?;

        Ok(FinalRobotPosition::new(robot.clone(), last_position))
    }

    fn next_robot_id(&mut self) -> RobotId {
        let id = RobotId(self.next_id);
        self.next_id += 1;
        id
    }
}

#[cfg(test)]
mod test {
    /*
    Test:
    - Robot Collision

    */
    use super::*;
    use crate::spatial::Orientation;

    #[test]
    fn test_successful_instructions() {
        let mut mars = Mars::new(5, 3);

        // first robot
        let r1 = mars
            .create_robot(Position::new(Coordinate::new(1, 1), Orientation::East))
            .unwrap();
        let i1 = vec![
            Instruction::Right,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Forward,
        ];

        // second robot
        let r2 = mars
            .create_robot(Position::new(Coordinate::new(3, 2), Orientation::North))
            .unwrap();
        let i2 = vec![
            Instruction::Forward,
            Instruction::Right,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Left,
            Instruction::Left,
            Instruction::Forward,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Left,
            Instruction::Left,
        ];

        // third robot
        let r3 = mars
            .create_robot(Position::new(Coordinate::new(0, 3), Orientation::West))
            .unwrap();
        let i3 = vec![
            Instruction::Left,
            Instruction::Left,
            Instruction::Forward,
            Instruction::Forward,
            Instruction::Forward,
            Instruction::Left,
            Instruction::Forward,
            Instruction::Left,
            Instruction::Forward,
            Instruction::Left,
        ];

        // first test
        let actual1 = mars.execute_instructions(&r1, &i1).unwrap();
        let expected1 = FinalRobotPosition::new(
            Robot::new(RobotId(0)),
            Position::new(Coordinate::new(1, 1), Orientation::East),
        );
        assert_eq!(expected1, actual1);

        // second test
        let actual2 = mars.execute_instructions(&r2, &i2).unwrap();
        let expected2 = {
            let mut r = Robot::new(RobotId(1));
            r.set_lost_status();
            FinalRobotPosition::new(r, Position::new(Coordinate::new(3, 3), Orientation::North))
        };
        assert_eq!(expected2, actual2);

        // third test
        let actual3 = mars.execute_instructions(&r3, &i3).unwrap();
        let expected3 = FinalRobotPosition::new(
            Robot::new(RobotId(2)),
            Position::new(Coordinate::new(2, 3), Orientation::South),
        );
        assert_eq!(expected3, actual3);
    }
}
