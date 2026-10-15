// talk about how I would make these more descriptive
#[derive(Debug, Eq, PartialEq)]
pub enum MarsError {
    RobotCollision,
    CoordinateOutOfBounds,
    RobotNotFound,
    CouldNotParseWorldDimensions,
    CouldNotParseOrientation,
    CouldNotParseCoordinates,
    CouldNotParseInstructions,
    TooManyInstructions,
    WorldDimensionTooLarge,
}

pub type Result<T> = std::result::Result<T, MarsError>;
