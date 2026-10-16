use crate::spatial::Position;

#[derive(Clone, Eq, PartialEq, Debug)]
enum RobotStatus {
    Alive,
    Lost,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Robot {
    status: RobotStatus,
    position: Position,
}
impl Robot {
    pub fn new(position: Position) -> Self {
        Self {
            status: RobotStatus::Alive,
            position,
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn set_lost_status(&mut self) {
        self.status = RobotStatus::Lost;
    }

    pub fn is_lost(&self) -> bool {
        self.status == RobotStatus::Lost
    }
}

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.position)?;
        if self.is_lost() {
            write!(f, " LOST")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::spatial::Orientation;

    #[test]
    fn test_new_robot_is_alive_at_initial_position() {
        let init_position = Position::new(1, 2, Orientation::North);
        let robot = Robot::new(init_position);

        assert_eq!(robot.position(), init_position);
        assert!(!robot.is_lost());
    }

    #[test]
    fn test_set_position_updates_position_only() {
        let mut robot = Robot::new(Position::new(1, 2, Orientation::North));
        let next = Position::new(3, 4, Orientation::West);

        robot.set_position(next);

        assert_eq!(robot.position(), next);
        assert!(!robot.is_lost());
    }

    #[test]
    fn test_set_lost_status_marks_robot_lost_and_keeps_position() {
        let init_position = Position::new(1, 2, Orientation::North);
        let mut robot = Robot::new(init_position);

        robot.set_lost_status();

        assert!(robot.is_lost());
        assert_eq!(robot.position(), init_position);
    }

    #[test]
    fn test_display_alive_robot() {
        let robot = Robot::new(Position::new(1, 1, Orientation::East));

        assert_eq!(robot.to_string(), "1 1 E");
    }

    #[test]
    fn test_display_lost_robot() {
        let mut robot = Robot::new(Position::new(3, 3, Orientation::North));
        robot.set_lost_status();

        assert_eq!(robot.to_string(), "3 3 N LOST");
    }
}
