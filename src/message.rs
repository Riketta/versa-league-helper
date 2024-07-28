use crate::{game_stage::GameStage, game_stage_interval::GameStageInterval};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MessageInvocationType {
    Always,
    Interval(GameStageInterval),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Role {
    Top,
    Jungle,
    Mid,
    ADC,
    Support,
}

impl Role {
    pub fn all() -> Vec<Role> {
        vec![Role::Top, Role::Jungle, Role::Mid, Role::ADC, Role::Support]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    pub text: String,
    pub roles: Vec<Role>,
    pub invocation_type: MessageInvocationType,
}

impl Message {
    pub fn new(roles: Vec<Role>, text: String, invocation_type: MessageInvocationType) -> Self {
        Self {
            text,
            roles,
            invocation_type,
        }
    }

    pub fn generate_default_messages() -> Vec<Message> {
        let messages = vec![
            // On Start.
            Message::new(
                Role::all(),
                "Determine the starting side of the enemy jungler.".to_string(),
                MessageInvocationType::Interval(GameStage::Start.to_interval()),
            ),
            Message::new(
                vec![Role::Jungle],
                "Kite camps.".to_string(),
                MessageInvocationType::Interval(GameStage::Start.to_interval()),
            ),
            // Always.
            Message::new(
                Role::all(),
                "Stop autopiloting. Focus!".to_string(),
                MessageInvocationType::Always,
            ),
            Message::new(
                vec![Role::Jungle],
                "Damage control - match enemy pressure, trade something in crossmap.".to_string(),
                MessageInvocationType::Always,
            ),
            Message::new(
                vec![Role::Jungle],
                "Do not flip - follow the 80% rule.".to_string(),
                MessageInvocationType::Always,
            ),
            Message::new(
                Role::all(),
                "Numbers advantage is the key.".to_string(),
                MessageInvocationType::Always,
            ),
            // Early Game.
            Message::new(
                vec![Role::Jungle],
                "Watch lanes more frequent! Especially between camps.".to_string(),
                MessageInvocationType::Interval(GameStage::Early.to_interval()),
            ),
            Message::new(
                vec![Role::Jungle],
                "Check the wave states and intentions of the laners.".to_string(),
                MessageInvocationType::Interval(GameStage::Early.to_interval()),
            ),
            Message::new(
                vec![Role::Jungle],
                "In dead time, gank for pressure if there is nothing else to do.".to_string(),
                MessageInvocationType::Interval(GameStage::Early.to_interval()),
            ),
            Message::new(
                vec![Role::Jungle],
                "Match the enemy jungler if you have an advantage.".to_string(),
                MessageInvocationType::Interval(GameStage::Early.to_interval()),
            ),
            Message::new(
                Role::all(),
                "Think about the options available to the enemy jungler.".to_string(),
                MessageInvocationType::Interval(GameStage::Early.to_interval()),
            ),
            Message::new(
                vec![Role::Jungle],
                "Play with the winning side.".to_string(),
                MessageInvocationType::Interval(GameStage::Early.to_interval()),
            ),
            // Mid and Late Game.
            Message::new(
                Role::all(),
                "Play for the towers.".to_string(),
                MessageInvocationType::Interval(GameStage::MidAndLate.to_interval()),
            ),
            Message::new(
                vec![Role::Jungle],
                "Be in sync with your team, don't farm when everyone is pressuring.".to_string(),
                MessageInvocationType::Interval(GameStage::MidAndLate.to_interval()),
            ),
            Message::new(
                vec![Role::Top, Role::Mid, Role::ADC, Role::Support],
                "Be in sync with your team, pressure map.".to_string(),
                MessageInvocationType::Interval(GameStage::MidAndLate.to_interval()),
            ),
            Message::new(
                Role::all(),
                "Push the waves, especially the mid waves, before doing anything else.".to_string(),
                MessageInvocationType::Interval(GameStage::MidAndLate.to_interval()),
            ),
            Message::new(
                Role::all(),
                "Abuse push into roam.".to_string(),
                MessageInvocationType::Interval(GameStage::MidAndLate.to_interval()),
            ),
            Message::new(
                Role::all(),
                "Try to not engage first.".to_string(),
                MessageInvocationType::Interval(GameStage::MidAndLate.to_interval()),
            ),
            Message::new(
                Role::all(),
                "Ward mid lane.".to_string(),
                MessageInvocationType::Interval(GameStage::MidAndLate.to_interval()),
            ),
            Message::new(
                Role::all(),
                "Siege into jungle vision control into objectives.".to_string(),
                MessageInvocationType::Interval(GameStage::MidAndLate.to_interval()),
            ),
            Message::new(
                Role::all(),
                "Check what's happening in the mid lane, be ready to cover.".to_string(),
                MessageInvocationType::Interval(GameStage::MidAndLate.to_interval()),
            ),
        ];

        messages
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_messages() {}
}
