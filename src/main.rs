mod game_session;
mod game_stage;
mod game_stage_interval;
mod league_api;
mod league_rest_api;
mod message;

use std::io;

use game_session::GameSession;
use message::{Message, Role};
use tts::*;

fn main() -> Result<(), Error> {
    let mut tts = Tts::default()?;
    let features = tts.supported_features();

    if features.rate {
        tts.set_rate(tts.normal_rate())?;
    }
    if features.pitch {
        tts.set_pitch(tts.max_pitch())?;
    }
    if features.volume {
        tts.set_volume(tts.max_volume())?;
    }
    // if features.voice && features.get_voice {
    //     let voices = tts.voices()?;

    //     for v in &voices {
    //         tts.set_voice(v)?;
    //         tts.speak(format!("This is {}.", v.name()), false)?;
    //     }
    // }

    let messages = Message::generate_default_messages();
    let mut session = GameSession::new(&mut tts, &messages, 30, Role::Jungle);
    session.start_session_loop();

    tts.speak("Input text.", false)?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        tts.speak(input, false)?;
    }

    // Ok(())
}
