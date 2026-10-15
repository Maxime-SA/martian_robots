use crate::Coordinate;
use crate::error::{MarsError, Result};
use crate::mars::Instruction;
use crate::spatial::{Orientation, Position};

const MAX_COORDINATE: usize = 50;
const MAX_INSTRUCTION_LEN: usize = 100;

pub fn parse_world_dimensions(input: &str) -> Result<(usize, usize)> {
    if let Some((max_cols, max_rows)) = input.trim().split_once(' ')
        && let Ok(max_cols) = max_cols.trim().parse::<usize>()
        && let Ok(max_rows) = max_rows.trim().parse::<usize>()
    {
        if max_cols > MAX_COORDINATE || max_rows > MAX_COORDINATE {
            Err(MarsError::WorldDimensionTooLarge)
        } else {
            Ok((max_cols, max_rows))
        }
    } else {
        Err(MarsError::CouldNotParseWorldDimensions)
    }
}

pub fn parse_robot_position(input: &str) -> Result<Position> {
    let mut iter = input.trim().split_whitespace();

    let x = iter
        .next()
        .and_then(|x| x.parse::<usize>().ok())
        .ok_or(MarsError::CouldNotParseCoordinates)?;

    let y = iter
        .next()
        .and_then(|y| y.parse::<usize>().ok())
        .ok_or(MarsError::CouldNotParseCoordinates)?;

    let orientation = iter
        .next()
        .and_then(|orientation| orientation.parse::<Orientation>().ok())
        .ok_or(MarsError::CouldNotParseOrientation)?;

    Ok(Position::new(Coordinate::new(x, y), orientation))
}

pub fn parse_robot_instructions(input: &str) -> Result<Vec<Instruction>> {
    let mut result = vec![];
    for instruction in input.trim().as_bytes().iter().filter(|b| **b != b' ') {
        let next = match *instruction {
            b'F' => Ok(Instruction::Forward),
            b'L' => Ok(Instruction::Left),
            b'R' => Ok(Instruction::Right),
            _ => Err(MarsError::CouldNotParseInstructions),
        }?;
        result.push(next);
    }

    if result.len() > MAX_INSTRUCTION_LEN {
        Err(MarsError::TooManyInstructions)
    } else {
        Ok(result)
    }
}

impl std::str::FromStr for Orientation {
    type Err = MarsError;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "E" => Ok(Orientation::East),
            "N" => Ok(Orientation::North),
            "W" => Ok(Orientation::West),
            "S" => Ok(Orientation::South),
            _ => Err(MarsError::CouldNotParseOrientation),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // World Dimension Parsing
    #[test]
    fn test_successful_world_dimension() {
        let input = "10 10";
        let expected_result = Ok((10, 10));
        let actual_result = parse_world_dimensions(input);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_successful_world_dimension_with_whitespace() {
        let input = "  10   10 ";
        let expected_result = Ok((10, 10));
        let actual_result = parse_world_dimensions(input);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_invalid_world_dimension_missing_axis() {
        let input = "10";
        let expected_result = Err(MarsError::CouldNotParseWorldDimensions);
        let actual_result = parse_world_dimensions(input);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_invalid_world_dimension_too_large() {
        let input = "50 60";
        let expected_result = Err(MarsError::WorldDimensionTooLarge);
        let actual_result = parse_world_dimensions(input);
        assert_eq!(expected_result, actual_result);
    }

    // Robot Position Parsing
    #[test]
    fn test_successful_robot_position() {
        let input = "1 1 E";
        let expected_result = Ok(Position::new(Coordinate::new(1, 1), Orientation::East));
        let actual_result = parse_robot_position(input);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_successful_robot_position_with_whitespace() {
        let input = "10  5  N  ";
        let expected_result = Ok(Position::new(Coordinate::new(10, 5), Orientation::North));
        let actual_result = parse_robot_position(input);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_invalid_robot_position_missing_coordinate() {
        let input = "5 N";
        let expected_result = Err(MarsError::CouldNotParseCoordinates);
        let actual_result = parse_robot_position(input);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_invalid_robot_position_missing_orientation() {
        let input = "10 5";
        let expected_result = Err(MarsError::CouldNotParseOrientation);
        let actual_result = parse_robot_position(input);
        assert_eq!(expected_result, actual_result);
    }

    // Instruction Parsing
    #[test]
    fn test_successful_instruction() {
        let input = "RFRFRFRF";
        let expected_result = Ok(vec![
            Instruction::Right,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Forward,
        ]);
        let actual_result = parse_robot_instructions(input);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_successful_instruction_with_whitesapce() {
        let input = "RF  RFRF L F";
        let expected_result = Ok(vec![
            Instruction::Right,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Left,
            Instruction::Forward,
        ]);
        let actual_result = parse_robot_instructions(input);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_invalid_instruction() {
        let input = "W";
        let expected_result = Err(MarsError::CouldNotParseInstructions);
        let actual_result = parse_robot_instructions(input);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_invalid_instruction_too_many() {
        let input = "LLLLLLLLLLLLLRLRLRLRLRLFFLRLFLRFLRLFRLFLRFLRFLRLFLRFLRFLLRFLRLFRLFLRFLRLFRLFLRLFRLFRLFLRFLRFLRFLRLFRLLRFLRLF";
        let expected_result = Err(MarsError::TooManyInstructions);
        let actual_result = parse_robot_instructions(input);
        assert_eq!(expected_result, actual_result);
    }
}
