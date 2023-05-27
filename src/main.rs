use std::{
    thread,
    time::{SystemTime, Duration},
};

use rusty_audio::Audio;

fn main() {
    let mut audio = Audio::new();
    let files = [ "grande"
                , "hour"
                , "minute"
                ];
    files
    .iter()
    .for_each(|f| audio.add(f, f.to_string() + ".wav"));
    let mut last_chime = (true, 0, 0, 0);
    loop {
        let time = get_time((0, 0, 0));
        let chime = get_bells(time);
        if chime.0 && !last_chime.0 {play_sound(1,       "grande", &mut audio)}
        if chime.1 != last_chime.1  {play_sound(chime.1, "hour",   &mut audio)}
        if chime.2 != last_chime.2  {play_sound(chime.2, "minute", &mut audio)}
        last_chime = chime;
        thread::sleep(Duration::from_secs(1));
    }
}

fn get_time(offset: (i64, i64, i64)) -> (u64, u64, u64) {
    let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .expect("Time before Unix epoch is not supported")
    .as_secs()
    .saturating_add_signed(offset.0 * 3600 + offset.1 * 60 + offset.2);
    let h = (now / 3600) % 24;
    let m = (now / 60  ) % 60;
    let s = (now       ) % 60;
    println!("{h}:{m}:{s}");
    (h, m, s)
}

fn get_bells(time: (u64, u64, u64)) -> (bool, u64, u64, u64) {
    let h = time.0 % 12;
    let pm = h == 0;   
    let m = time.1 / 15;
    (pm, h, m, 0)
}

fn play_sound(n: u64, file: &str, audio: &mut Audio) {
    for _ in 0..n {
        audio.play(file);
        audio.wait();
    }
}
