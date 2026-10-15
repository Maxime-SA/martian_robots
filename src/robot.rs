#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct RobotId(pub u128);

#[derive(Clone, Eq, PartialEq, Debug)]
enum RobotStatus {
    Alive,
    Lost,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Robot {
    id: RobotId,
    status: RobotStatus,
}
impl Robot {
    pub fn new(id: RobotId) -> Self {
        Self {
            id,
            status: RobotStatus::Alive,
        }
    }

    pub fn set_lost_status(&mut self) {
        self.status = RobotStatus::Lost;
    }

    pub fn is_lost(&self) -> bool {
        self.status == RobotStatus::Lost
    }
}
