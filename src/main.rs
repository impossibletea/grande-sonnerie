use std::{
    thread,
    time::{SystemTime, Duration},
};

use rusty_audio::Audio;

fn main() {
    let files = [
        "grande1",
        "grande2",
        "hour",
        "minute"
    ];
    let mut audio = Audio::new();
    files.iter()
        .for_each(|f| audio.add(f, f.to_string() + ".wav"));
    let mut last_chime = (true, 0, 0, 0);
    loop {
        let time = get_time((0, 0, 0));
        let chime = get_bells(time);
        println!("{chime:?}");
        if chime.0 != last_chime.0 {play_sound(1, "grande1", &mut audio)}
        if chime.1 != last_chime.1 {play_sound(chime.1, "hour", &mut audio)};
        if chime.2 != last_chime.2 {play_sound(chime.2, "minute", &mut audio)};
        last_chime = chime;
        thread::sleep(Duration::from_secs(5));
    }
}

fn get_time(offset: (u64, u64, u64)) -> (u8, u8, u8) {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time before Unix epoch is not supported")
        .as_secs();
    let h = (now / 3600 + offset.0) % 24;
    let m = (now / 60   + offset.1) % 60;
    let s = (now        + offset.2) % 60;
    (h as u8, m as u8, s as u8)
}

fn get_bells(time: (u8, u8, u8)) -> (bool, u8, u8, u8) {
    let (pm, h) = match time.0 {
        0       => (false, 12),
        1..=11  => (false, time.0),
        12      => (true,  time.0),
        13..=23 => (true,  time.0 - 12),
        _       => (false, 0)
    };
    let m = time.1 / 15;
    (pm, h, m, 0)
}

fn play_sound(n: u8, file: &str, audio: &mut Audio) {
    for _ in 0..n {
        audio.play(file);
        audio.wait();
    }
}
