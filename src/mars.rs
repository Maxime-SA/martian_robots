use std::collections::{HashMap, HashSet};

use crate::error::{MarsError, Result};
use crate::robot::{Instruction, Robot, RobotId};
use crate::spatial::{Coordinate, Grid, Position};

pub struct FinalRobotPosition {
    robot: Robot,
    position: Position,
}
impl FinalRobotPosition {
    fn new(robot: Robot, position: Position) -> Self {
        Self { robot, position }
    }
}

pub struct Mars {
    grid: Grid,
    robots: HashMap<RobotId, Robot>,
    lost_scents: HashSet<Coordinate>,
    next_id: u128, // would use UUID in the future
}
impl Mars {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Self {
            grid: Grid::new(nrows, ncols),
            robots: HashMap::new(),
            lost_scents: HashSet::new(),
            next_id: 0,
        }
    }

    pub fn create_robot(&mut self, position: Position) -> Result<&RobotId> {
        let id = self.next_robot_id();
        let robot = Robot::new(id);

        self.grid.insert(id, position)?;

        self.robots.insert(id, robot);

        Ok(self.robots.get(&id).unwrap().id())
    }

    pub fn execute_instructions(
        &mut self,
        robot_id: &RobotId,
        instructions: &[Instruction],
    ) -> Result<FinalRobotPosition> {
        let robot = self
            .robots
            .get_mut(robot_id)
            .ok_or(MarsError::RobotNotFound)?;

        for instruction in instructions {
            match instruction {
                Instruction::Forward => {
                    if let Err(error) = self.grid.move_forward(robot_id) {
                        match error {
                            MarsError::CoordinateOutOfBounds => {
                                let coord = self.grid.get_coord(robot_id)?;

                                if !self.lost_scents.contains(coord) {
                                    robot.set_lost_status();

                                    self.lost_scents.insert(*coord);
                                    let last_position = self.grid.remove(robot_id)?;

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
                Instruction::Left => self.grid.pivot_left(robot_id)?,
                Instruction::Right => self.grid.pivot_right(robot_id)?,
            }
        }

        unimplemented!()
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
}
