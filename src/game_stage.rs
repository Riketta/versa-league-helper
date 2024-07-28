use crate::game_stage_interval::GameStageInterval;

#[derive(Clone, Copy, Debug)]
pub enum GameStage {
    Start,
    Early,
    MidAndLate,
}

impl GameStage {
    pub fn from(seconds: u64) -> Self {
        match seconds {
            0..=90 => Self::Start,
            91..840 => Self::Early,
            _ => Self::MidAndLate,
        }
    }

    pub fn to_interval(&self) -> GameStageInterval {
        match self {
            GameStage::Start => GameStageInterval {
                start_timestamp: 0,
                end_timestamp: 90,
            },
            GameStage::Early => GameStageInterval {
                start_timestamp: 91,
                end_timestamp: 840,
            },
            GameStage::MidAndLate => GameStageInterval {
                start_timestamp: 841,
                end_timestamp: 3600,
            },
        }
    }
}
