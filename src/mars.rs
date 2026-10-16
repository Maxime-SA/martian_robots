use std::collections::HashSet;
use std::io::Write;

use crate::error::{MarsError, Result};
use crate::parser::*;
use crate::robot::Robot;
use crate::spatial::{Coordinate, Grid, Position};

#[derive(Eq, PartialEq, Debug)]
pub enum Instruction {
    Left,
    Right,
    Forward,
}

pub struct Mars {
    grid: Grid,
    lost_scents: HashSet<Coordinate>,
}
impl Mars {
    pub fn new(max_col: usize, max_row: usize) -> Self {
        Self {
            grid: Grid::new(max_col, max_row),
            lost_scents: HashSet::new(),
        }
    }

    pub fn parse_and_execute<W: Write>(input: &str, writer: &mut W) -> Result<()> {
        let mut iter = input.lines();

        let (max_col, max_row) = {
            let world_dimensions_line = iter.next().ok_or(MarsError::MissingWorldDimensions)?;

            parse_world_dimensions(world_dimensions_line)?
        };

        let mut mars = Self::new(max_col, max_row);

        while let Some(robot_position) = iter.next() {
            let raw_instructions = iter.next().ok_or(MarsError::MissingRobotInstructions)?;

            let init_position = {
                let pos = parse_init_position(robot_position)?;
                if !mars.grid.contains(pos) {
                    return Err(MarsError::RobotInitPositionOutOfBounds);
                }
                pos
            };

            let instructions = parse_instructions(raw_instructions)?;

            let robot = mars.execute(init_position, &instructions);

            writeln!(writer, "{robot}").map_err(|_| MarsError::IOError)?;
        }

        Ok(())
    }

    pub fn execute(&mut self, init_position: Position, instructions: &[Instruction]) -> Robot {
        let mut robot = Robot::new(init_position);

        for instruction in instructions {
            let next_position = match instruction {
                Instruction::Forward => match self.grid.move_forward(robot.position()) {
                    Some(next) => next,
                    None => {
                        let position = robot.position();
                        let coord = position.coord();

                        if !self.lost_scents.contains(&coord) {
                            robot.set_lost_status();

                            self.lost_scents.insert(coord);

                            break;
                        }

                        position
                    }
                },
                Instruction::Left => self.grid.pivot_left(robot.position()),
                Instruction::Right => self.grid.pivot_right(robot.position()),
            };

            robot.set_position(next_position);
        }

        robot
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::spatial::Orientation;

    fn alive(x: usize, y: usize, orientation: Orientation) -> Robot {
        Robot::new(Position::new(x, y, orientation))
    }

    fn lost(x: usize, y: usize, orientation: Orientation) -> Robot {
        let mut robot = alive(x, y, orientation);
        robot.set_lost_status();
        robot
    }

    fn instr(instructions: &str) -> Vec<Instruction> {
        parse_instructions(instructions).unwrap()
    }

    fn run(input: &str) -> (String, Result<()>) {
        let mut output = Vec::new();
        let result = Mars::parse_and_execute(input, &mut output);
        (String::from_utf8(output).unwrap(), result)
    }

    // parse_and_execute
    #[test]
    fn test_parse_and_execute_sample() {
        let input = "5 3\n1 1 E\nRFRFRFRF\n3 2 N\nFRRFLLFFRRFLL\n0 3 W\nLLFFFLFLFL\n";

        let (actual_output, actual_result) = run(input);

        assert_eq!("1 1 E\n3 3 N LOST\n2 3 S\n", actual_output);
        assert_eq!(Ok(()), actual_result);
    }

    #[test]
    fn test_parse_and_execute_errors() {
        let cases = [
            ("", MarsError::MissingWorldDimensions),
            ("5", MarsError::CouldNotParseWorldDimensions),
            ("a b", MarsError::CouldNotParseWorldDimensions),
            ("5 3 7", MarsError::CouldNotParseWorldDimensions),
            ("51 3", MarsError::WorldDimensionsTooLarge),
            ("3 51", MarsError::WorldDimensionsTooLarge),
            ("5 3\n1 1 E\n", MarsError::MissingRobotInstructions),
            ("5 3\n-1 1 E\nF\n", MarsError::CouldNotParseCoordinates),
            ("5 3\na 1 E\nF\n", MarsError::CouldNotParseCoordinates),
            ("5 3\n1 E\nF\n", MarsError::CouldNotParseCoordinates),
            ("5 3\n1 1 X\nF\n", MarsError::CouldNotParseOrientation),
            ("5 3\n1 1 e\nF\n", MarsError::CouldNotParseOrientation),
            ("5 3\n1 1\nF\n", MarsError::CouldNotParseOrientation),
            ("5 3\n6 0 N\nF\n", MarsError::RobotInitPositionOutOfBounds),
            ("5 3\n0 4 N\nF\n", MarsError::RobotInitPositionOutOfBounds),
            ("5 3\n1 1 E\nFX\n", MarsError::CouldNotParseInstructions),
            ("5 3\n1 1 E\nf\n", MarsError::CouldNotParseInstructions),
            ("5 3\n1 1 E\nF1\n", MarsError::CouldNotParseInstructions),
        ];

        for (input, expected_error) in cases {
            let (_, actual_result) = run(input);
            assert_eq!(Err(expected_error), actual_result);
        }
    }

    #[test]
    fn test_parse_and_execute_instruction_length_boundary() {
        let cases = [
            (99, "0 0 E\n", Ok(())),
            (100, "", Err(MarsError::TooManyInstructions)),
        ];

        for (count, expected_output, expected_result) in cases {
            let input = format!("5 3\n0 0 N\n{}\n", "L".repeat(count));

            let (actual_output, actual_result) = run(&input);

            assert_eq!(expected_output, actual_output);
            assert_eq!(expected_result, actual_result);
        }
    }

    #[test]
    fn test_parse_and_execute_world_size_boundary() {
        let cases = [
            ("50 50\n50 50 N\nF\n", "50 50 N LOST\n", Ok(())),
            ("51 50\n", "", Err(MarsError::WorldDimensionsTooLarge)),
            ("50 51\n", "", Err(MarsError::WorldDimensionsTooLarge)),
        ];

        for (input, expected_output, expected_result) in cases {
            let (actual_output, actual_result) = run(input);

            assert_eq!(expected_output, actual_output);
            assert_eq!(expected_result, actual_result);
        }
    }

    #[test]
    fn test_parse_and_execute_keeps_output_before_error() {
        let input = "5 3\n1 1 E\nF\n6 0 N\nF\n";

        let (actual_output, actual_result) = run(input);

        assert_eq!("2 1 E\n", actual_output);
        assert_eq!(Err(MarsError::RobotInitPositionOutOfBounds), actual_result);
    }

    // execute
    #[test]
    fn test_execute_ignores_move_off_world_at_scent() {
        let mut mars = Mars::new(5, 3);

        let actual_result = mars.execute(Position::new(5, 1, Orientation::East), &instr("F"));
        assert_eq!(lost(5, 1, Orientation::East), actual_result);

        let actual_result = mars.execute(Position::new(5, 1, Orientation::East), &instr("FL"));
        assert_eq!(alive(5, 1, Orientation::North), actual_result);
    }

    #[test]
    fn test_execute_scent_is_coordinate_only() {
        let mut mars = Mars::new(5, 3);

        let actual_result = mars.execute(Position::new(5, 3, Orientation::North), &instr("F"));
        assert_eq!(lost(5, 3, Orientation::North), actual_result);

        let actual_result = mars.execute(Position::new(5, 3, Orientation::East), &instr("F"));
        assert_eq!(alive(5, 3, Orientation::East), actual_result);
    }

    #[test]
    fn test_execute_scent_does_not_protect_neighbours() {
        let mut mars = Mars::new(5, 3);

        let actual_result = mars.execute(Position::new(5, 1, Orientation::East), &instr("F"));
        assert_eq!(lost(5, 1, Orientation::East), actual_result);

        let actual_result = mars.execute(Position::new(5, 2, Orientation::East), &instr("F"));
        assert_eq!(lost(5, 2, Orientation::East), actual_result);
    }

    #[test]
    fn test_execute_ignored_move_then_loss_elsewhere() {
        let mut mars = Mars::new(5, 3);

        let actual_result = mars.execute(Position::new(5, 1, Orientation::East), &instr("F"));
        assert_eq!(lost(5, 1, Orientation::East), actual_result);

        // ignores the move at the first scent, then falls off the north edge
        let actual_result = mars.execute(Position::new(5, 1, Orientation::East), &instr("FLFFFF"));
        assert_eq!(lost(5, 3, Orientation::North), actual_result);

        // protected by both scents
        let actual_result = mars.execute(Position::new(5, 1, Orientation::East), &instr("FLFFFF"));
        assert_eq!(alive(5, 3, Orientation::North), actual_result);
    }

    #[test]
    fn test_execute_loss_at_each_edge() {
        let cases = [
            ((2, 3), Orientation::North),
            ((2, 0), Orientation::South),
            ((5, 1), Orientation::East),
            ((0, 1), Orientation::West),
        ];

        for ((x, y), orientation) in cases {
            let mut mars = Mars::new(5, 3);

            let actual_result = mars.execute(Position::new(x, y, orientation), &instr("F"));

            assert_eq!(lost(x, y, orientation), actual_result);
        }
    }

    #[test]
    fn test_execute_lost_robot_ignores_remaining_instructions() {
        let mut mars = Mars::new(5, 3);

        let actual_result = mars.execute(Position::new(5, 3, Orientation::North), &instr("FRFRFR"));

        assert_eq!(lost(5, 3, Orientation::North), actual_result);
    }

    #[test]
    fn test_execute_zero_size_world() {
        let mut mars = Mars::new(0, 0);

        let actual_result = mars.execute(Position::new(0, 0, Orientation::North), &instr("F"));
        assert_eq!(lost(0, 0, Orientation::North), actual_result);

        // every move is ignored because of the scent, only the turn applies
        let actual_result = mars.execute(Position::new(0, 0, Orientation::North), &instr("FRF"));
        assert_eq!(alive(0, 0, Orientation::East), actual_result);
    }

    #[test]
    fn test_execute_non_square_worlds() {
        use Orientation::{East, North};

        // every robot starts at (0, 0)
        let cases = [
            ((5, 3), North, "FFF", alive(0, 3, North)),
            ((5, 3), North, "FFFF", lost(0, 3, North)),
            ((5, 3), East, "FFFFF", alive(5, 0, East)),
            ((5, 3), East, "FFFFFF", lost(5, 0, East)),
            ((3, 5), North, "FFFFF", alive(0, 5, North)),
            ((3, 5), North, "FFFFFF", lost(0, 5, North)),
            ((3, 5), East, "FFF", alive(3, 0, East)),
            ((3, 5), East, "FFFF", lost(3, 0, East)),
        ];

        for ((max_col, max_row), orientation, instructions, expected_result) in cases {
            let mut mars = Mars::new(max_col, max_row);

            let actual_result =
                mars.execute(Position::new(0, 0, orientation), &instr(instructions));

            assert_eq!(expected_result, actual_result);
        }
    }
}
