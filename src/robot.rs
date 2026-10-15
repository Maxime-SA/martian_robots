// we could make this dynamic at run-time
pub enum Instruction {
    Left,
    Right,
    Forward,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct RobotId(pub u128);

#[derive(Clone)]
enum RobotStatus {
    Alive,
    Lost,
}

#[derive(Clone)]
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

    pub fn id(&self) -> &RobotId {
        &self.id
    }

    pub fn status(&self) -> &RobotStatus {
        &self.status
    }

    pub fn set_lost_status(&mut self) {
        self.status = RobotStatus::Lost;
    }
}
