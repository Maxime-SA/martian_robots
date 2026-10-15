#[derive(Debug)]
pub enum MarsError {
    RobotCollision,
    CoordinateOutOfBounds,
    RobotNotFound
}

pub type Result<T> = std::result::Result<T, MarsError>;
