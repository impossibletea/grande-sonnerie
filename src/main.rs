use std::{
    fs::File,
    io::BufReader,
};

use rusty_audio::Audio;
use serde::{Serialize, Deserialize};
use time::{UtcOffset, OffsetDateTime};

mod sonnerie;
use sonnerie::Sonnerie;

#[derive(Serialize, Deserialize)]
struct Config {
}

fn main() {
    let file = "test.wav";
    let offset = UtcOffset::from_hms(7, 0, 0).unwrap_or(UtcOffset::UTC);
    let mut audio = Audio::new();
    audio.add("chime", file);

    let mut last_chime: Option<(u8, u8)> = None;

    let (h, m) = local_time(offset);
    last_chime = Some((h, m));
    play_sound(h, "chime", audio);
}

fn local_time(offset: UtcOffset) -> (u8, u8) {
    let time = OffsetDateTime::now_utc()
        .to_offset(offset);
    (time.hour(), time.minute())
}

fn play_sound(n: u8, file: &str, mut audio: Audio) {
    for _ in 0..n {
        audio.play(file);
        audio.wait();
    }
}
