use crate::message::{Message, MessageInvocationType, Role};
use chrono::{DateTime, Local, TimeDelta, TimeZone};
use rand::{rngs::ThreadRng, seq::IteratorRandom};
use std::{cell::RefCell, thread};
use tts::Tts;

pub struct GameSession<'a> {
    rng: RefCell<ThreadRng>,
    tts: &'a mut Tts,
    messages: &'a Vec<Message>,
    started_at: DateTime<Local>,
    last_message_at: DateTime<Local>,
    message_interval: TimeDelta,
    player_role: Role,
}

impl<'a> GameSession<'a> {
    pub fn new(
        tts: &'a mut Tts,
        messages: &'a Vec<Message>,
        message_interval: i64,
        player_role: Role,
    ) -> Self {
        Self {
            rng: RefCell::new(rand::thread_rng()),
            tts,
            messages,
            started_at: Local::now(),
            last_message_at: Local.timestamp_opt(0, 0).unwrap(),
            message_interval: TimeDelta::new(message_interval, 0).unwrap(),
            player_role,
        }
    }

    pub fn start_session_loop(&mut self) {
        let messages = self.valid_message_sequence();
        let mut messages_count = messages.len();
        let mut message_iter = messages.into_iter();
        let mut message_history = Vec::<Message>::new();

        loop {
            if Local::now() - self.last_message_at < self.message_interval {
                thread::sleep(std::time::Duration::from_millis(1 * 1000));
                continue;
            }

            let mut new_messages = self.valid_message_sequence();
            // New messages available, update message pool.
            if messages_count != new_messages.len() {
                // TODO: improve validation - there is a chance that message pools will be different even though they are the same size.
                println!(
                    "[!] Initial message pool: {}; Fresh message pool: {}.",
                    messages_count,
                    new_messages.len()
                );
                println!("[+] Replacing message pool!");
                messages_count = new_messages.len();

                // Filter recent messages for this iteration.
                for message in &message_history {
                    println!("[-] Filtering message: {message:?}.");
                    new_messages.retain(|msg| msg != message)
                }

                message_history.clear();
                message_iter = new_messages.into_iter();
            }

            let message = match message_iter.next() {
                Some(message) => message,
                None => {
                    println!("[~] Refreshing message pool.");
                    message_history.clear();
                    message_iter = self.valid_message_sequence().into_iter();
                    message_iter.next().unwrap()
                }
            };
            message_history.push(message.clone());

            self.last_message_at = Local::now();

            println!(
                "Session duration: {}; Playing: [{:?}].",
                self.duration(),
                message,
            );
            self.play_message(&message.text);
        }
    }

    fn duration(&self) -> i64 {
        (Local::now() - self.started_at).num_seconds()
    }

    fn valid_message_sequence(&self) -> Vec<Message> {
        self.messages
            .iter()
            .filter(|&message| {
                message.roles.contains(&self.player_role)
                    && match message.invocation_type {
                        MessageInvocationType::Always => true,
                        MessageInvocationType::Interval(interval) => {
                            interval.is_duration_fits(self.duration())
                        }
                    }
            })
            .map(|message| message.clone())
            .collect::<Vec<_>>()
    }

    fn random_valid_message(&self) -> Message {
        let messages = self.valid_message_sequence();
        let mut rng = self.rng.borrow_mut();
        let message = messages.iter().choose(&mut *rng).unwrap();

        (*message).clone()
    }

    fn play_message(&mut self, message: &str) {
        self.tts.speak(message, false).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::game_stage::GameStage;

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_game_session() -> Result<()> {
        let mut tts = Tts::default()?;
        let messages = vec![
            Message::new(
                Role::all(),
                format!("{:?}", MessageInvocationType::Always),
                MessageInvocationType::Always,
            ),
            Message::new(
                Role::all(),
                format!("{:?}", GameStage::Start),
                MessageInvocationType::Interval(GameStage::Start.to_interval()),
            ),
            Message::new(
                Role::all(),
                format!("{:?}", GameStage::Early),
                MessageInvocationType::Interval(GameStage::Early.to_interval()),
            ),
            Message::new(
                Role::all(),
                format!("{:?}", GameStage::MidAndLate),
                MessageInvocationType::Interval(GameStage::MidAndLate.to_interval()),
            ),
        ];

        let mut session = GameSession::new(&mut tts, &messages, 5, Role::Jungle);

        session.started_at = Local::now();
        assert_eq!(session.duration(), 0);
        let messages = session
            .valid_message_sequence()
            .into_iter()
            .map(|message| message.text)
            .collect::<Vec<_>>();
        assert_eq!(
            messages,
            vec![
                format!("{:?}", MessageInvocationType::Always),
                format!("{:?}", GameStage::Start),
            ]
        );

        let offset = TimeDelta::new(GameStage::Early.to_interval().start_timestamp, 0).unwrap();
        session.started_at = Local::now() - offset;
        assert_eq!(session.duration(), offset.num_seconds());
        let messages = session
            .valid_message_sequence()
            .into_iter()
            .map(|message| message.text)
            .collect::<Vec<_>>();
        assert_eq!(
            messages,
            vec![
                format!("{:?}", MessageInvocationType::Always),
                format!("{:?}", GameStage::Early),
            ]
        );

        let offset =
            TimeDelta::new(GameStage::MidAndLate.to_interval().start_timestamp, 0).unwrap();
        session.started_at = Local::now() - offset;
        assert_eq!(session.duration(), offset.num_seconds());
        let messages = session
            .valid_message_sequence()
            .into_iter()
            .map(|message| message.text)
            .collect::<Vec<_>>();
        assert_eq!(
            messages,
            vec![
                format!("{:?}", MessageInvocationType::Always),
                format!("{:?}", GameStage::MidAndLate),
            ]
        );

        Ok(())
    }
}
