use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use tts::{Features, Tts};

pub struct TTS {
    tts: Tts,
    features: Features,

    rng: ThreadRng,
    use_random_pitch: bool,
    use_random_voice: bool,
}

impl TTS {
    pub fn new(tts: Tts) -> Self {
        let features = tts.supported_features();
        Self {
            tts,
            features,

            rng: rand::thread_rng(),
            use_random_pitch: false,
            use_random_voice: false,
        }
    }

    pub fn play_message(&mut self, message: &str) {
        if self.use_random_pitch && self.features.pitch {
            let pitches = self.min_pitch()..=self.max_pitch();
            let pitch = self.rng.gen_range(pitches);

            self.set_pitch(pitch).unwrap();
        }
        if self.use_random_voice && self.features.voice && self.features.get_voice {
            let voices = self.voices().unwrap();
            let voice = voices.into_iter().choose(&mut self.rng).unwrap();

            self.set_voice(&voice).unwrap();
        }

        self.tts.speak(message, false).unwrap();
    }

    pub fn use_random_pitch(&mut self, use_random_pitch: bool) {
        self.use_random_pitch = use_random_pitch;
    }

    pub fn use_random_voice(&mut self, use_random_voice: bool) {
        self.use_random_voice = use_random_voice;
    }
}

impl Default for TTS {
    fn default() -> Self {
        let mut tts = TTS::new(Tts::default().unwrap());

        if tts.features.rate {
            let rate = tts.normal_rate();
            tts.set_rate(rate).unwrap();
        }
        if tts.features.pitch {
            let pitch = tts.max_pitch();
            tts.set_pitch(pitch).unwrap();
        }
        if tts.features.volume {
            let volume = tts.max_volume();
            tts.set_volume(volume).unwrap();
        }

        tts
    }
}

impl std::ops::Deref for TTS {
    type Target = Tts;

    fn deref(&self) -> &Self::Target {
        &self.tts
    }
}

impl std::ops::DerefMut for TTS {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tts
    }
}
