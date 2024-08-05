mod game_session;
mod game_stage;
mod game_stage_interval;
mod league_api;
mod league_rest_api;
mod message;
mod tts;

use game_session::GameSession;
use message::{Message, Role};
use tts::*;

fn main() -> anyhow::Result<()> {
    let mut tts = TTS::default();
    tts.use_random_pitch(true);
    tts.use_random_voice(true);

    // TODO: add a message validator to prevent messages that can be played 0 times during a game session.
    // E.g. messages with too short an interval, placed too far down in priority.
    let messages = Message::generate_default_messages();
    let mut session = GameSession::new(&mut tts, &messages, 30, Role::Jungle);
    session.start_session_loop();

    Ok(())
}
