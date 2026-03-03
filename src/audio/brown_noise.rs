use rand::Rng;
use rodio::{Sink, Source};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

static IS_PLAYING: AtomicBool = AtomicBool::new(false);

struct BrownNoise {
    last_l: f32,
    last_r: f32,
    smooth_l: f32,
    smooth_r: f32,
    next_is_right: bool,
}

impl BrownNoise {
    fn new() -> Self {
        Self {
            last_l: 0.0,
            last_r: 0.0,
            smooth_l: 0.0,
            smooth_r: 0.0,
            next_is_right: false,
        }
    }
}

impl Iterator for BrownNoise {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.next_is_right {
            self.next_is_right = false;
            return Some(self.safe_output(self.smooth_r));
        }

        let white_l: f32 = rand::rng().random_range(-0.7..0.7);
        let white_r: f32 = rand::rng().random_range(-0.7..0.7);

        self.last_l = (self.last_l + (white_l * 0.05)) * 0.99;
        self.last_r = (self.last_r + (white_r * 0.05)) * 0.99;

        let alpha = 0.05;
        self.smooth_l += alpha * (self.last_l - self.smooth_l);
        self.smooth_r += alpha * (self.last_r - self.smooth_r);

        self.next_is_right = true;
        Some(self.safe_output(self.smooth_l))
    }
}

impl BrownNoise {
    // ganho e proteção de clipping
    fn safe_output(&self, input: f32) -> f32 {
        let gain = 0.6; // ganho base
        let x = input * gain;
        // soft clipping: comprime o sinal conforme ele chega perto de 1.0
        x / (1.0 + x.abs())
    }
}

impl Source for BrownNoise {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        2
    } // stereo
    fn sample_rate(&self) -> u32 {
        44100
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

enum AudioCommand {
    Play,
    Stop,
    SetVolume(f32),
}

static AUDIO_SENDER: std::sync::OnceLock<Sender<AudioCommand>> = std::sync::OnceLock::new();

fn spawn_audio_thread() -> Sender<AudioCommand> {
    let (tx, rx) = channel::<AudioCommand>();

    thread::spawn(move || {
        let (_stream, stream_handle) =
            rodio::OutputStream::try_default().expect("Failed to open audio output");
        let mut current_sink: Option<Sink> = None;
        let mut current_volume: f32 = 0.6;

        loop {
            match rx.recv() {
                Ok(AudioCommand::Play) => {
                    if let Some(sink) = current_sink.take() {
                        drop(sink);
                    }
                    let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");
                    sink.set_volume(current_volume);
                    sink.append(BrownNoise::new());
                    sink.play();
                    current_sink = Some(sink);
                }
                Ok(AudioCommand::Stop) => {
                    if let Some(sink) = current_sink.take() {
                        sink.stop();
                    }
                }
                Ok(AudioCommand::SetVolume(vol)) => {
                    current_volume = vol;
                    if let Some(ref sink) = current_sink {
                        sink.set_volume(vol);
                    }
                }
                Err(_) => break,
            }
        }
    });

    tx
}

pub fn play() {
    let sender = AUDIO_SENDER.get_or_init(spawn_audio_thread);
    let _ = sender.send(AudioCommand::Play);
}

pub fn stop() {
    if let Some(sender) = AUDIO_SENDER.get() {
        let _ = sender.send(AudioCommand::Stop);
    }
}

pub fn set_volume(volume: f64) {
    let sender = AUDIO_SENDER.get_or_init(spawn_audio_thread);
    let _ = sender.send(AudioCommand::SetVolume(volume as f32));
}

pub fn toggle() {
    let playing = IS_PLAYING.fetch_xor(true, Ordering::SeqCst);
    if playing {
        stop();
    } else {
        play();
    }
}
