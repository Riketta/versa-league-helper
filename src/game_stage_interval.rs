#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameStageInterval {
    pub start_timestamp: i64,
    pub end_timestamp: i64,
}

impl GameStageInterval {
    pub fn is_duration_fits(&self, seconds: i64) -> bool {
        if seconds >= self.start_timestamp && seconds <= self.end_timestamp {
            true
        } else {
            false
        }
    }
}
