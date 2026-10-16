#[derive(Debug, Eq, PartialEq)]
pub enum MarsError {
    CouldNotParseWorldDimensions,
    CouldNotParseOrientation,
    CouldNotParseCoordinates,
    CouldNotParseInstructions,
    TooManyInstructions,
    WorldDimensionsTooLarge,
    MissingWorldDimensions,
    MissingRobotInstructions,
    RobotInitPositionOutOfBounds,
    IOError,
}

pub type Result<T> = std::result::Result<T, MarsError>;
