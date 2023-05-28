use std::{
    thread,
    time::{SystemTime, Duration}
};

use rusty_audio::Audio;
use serde::{Serialize, Deserialize};

type Time  = (u8, u8);
type Chime = (bool, Time);

fn main() {
    let config = Config::new();
    let mut movement = Movement::new(&config.movement);
    println!("{movement:?}");
    let mut audio = Audio::new();

    let path =
        confy::get_configuration_file_path("grande-sonnerie", None)
        .unwrap_or_default()
        .parent()
        .expect("Why is empty")
        .to_path_buf();
    ["grand", "hour", "minute"]
    .iter()
    .for_each(|sound| {
        let sound_path =
            path
            .join(&config.sonnerie)
            .join(sound.to_string() + ".wav");
        audio.add(sound, sound_path);
    });

    loop {
        let time = get_time(&config.offset);
        println!("{time:?}");
        let chimes = movement.got_chimes(time);
        movement.sonne(chimes, &mut audio);
        thread::sleep(Duration::from_secs(1));
    }
}

fn get_time(offset: &(i64, i64, i64)) -> Time {
    let now =
        SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time before Unix epoch is not supported")
        .as_secs()
        .saturating_add_signed(offset.0 * 3600 + offset.1 * 60 + offset.2);
    let h = (now / 3600) % 24;
    let m = (now / 60  ) % 60;
    (h as u8, m as u8)
}

fn play_sound(n: u8, file: &str, audio: &mut Audio) {
    (0..n).for_each(|_| {
        audio.play(file);
        audio.wait();
    });
}

#[derive(Serialize, Deserialize)]
struct Config {
    offset:   (i64, i64, i64),
    sonnerie: String,
    movement: String,
}

impl Config {
    fn new() -> Self {
        confy::load("grande-sonnerie", "config").unwrap_or_default()
    }
}

impl std::default::Default for Config {
    fn default() -> Self {
        Config {
            offset:   (0, 0, 0),
            sonnerie: "coucou".to_string(),
            movement: "casio".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MovementConfig {
    grand:       Option<Vec<u8>>,
    hours:       Option<Vec<u8>>,
    hours_div:   Option<u8>,
    minutes:     Option<Vec<u8>>,
    minutes_div: Option<u8>,
    twelve_hour: bool,
    multichime:  bool,
}

impl std::default::Default for MovementConfig {
    fn default() -> Self {
        MovementConfig {
            grand:       None,
            hours:       None,
            hours_div:   Some(1),
            minutes:     None,
            minutes_div: None,
            twelve_hour: false,
            multichime:  false,
        }
    }
}

#[derive(Debug)]
struct Movement {
    am_pm:   u8,
    grand:   Vec<u8>,
    hours:   Vec<u8>,
    minutes: Vec<u8>,
    multi:   bool,
    past:    Chime,
}

impl Movement {
    fn new(config_name: &str) -> Self {
        let config: MovementConfig =
            confy::load("grande-sonnerie", config_name)
            .unwrap_or_default();

        let am_pm: u8 = if config.twelve_hour {12} else {24};

        let mut grand: Vec<u8> = Vec::new();
        if let Some(vec) = config.grand {
            vec.iter().for_each(|x| grand.push(*x));
        }

        let mut hours: Vec<u8> = Vec::new();
        if let Some(vec) = config.hours {
            vec.iter().for_each(|x| hours.push(*x));
        }
        if let Some(div) = &config.hours_div {
            (0..=24).for_each(|x| hours.push((x / div) * div));
        }
        hours.sort();
        hours.dedup();
        if hours.contains(&0) {hours.remove(0);}

        let mut minutes: Vec<u8> = Vec::new();
        if let Some(vec) = config.minutes {
            vec.iter().for_each(|x| minutes.push(*x));
        }
        if let Some(div) = config.minutes_div {
            (0..=59).for_each(|x| minutes.push((x / div)*div));
        }
        minutes.sort();
        minutes.dedup();
        if minutes.contains(&0) {minutes.remove(0);}

        Movement {
            am_pm,
            grand,
            hours,
            minutes,
            multi:  config.multichime,
            past:   (false, (0, 0)),
        }
    }

    fn got_chimes(&self, time: Time) -> Chime {
        let h = time.0;
        let m = time.1;

        let g_cs =    self.grand.contains(&h);
        let h_cs = if self.hours.contains(&h) {h} else {0};
        let m_cs = if self.minutes.contains(&m) {m} else {0};

        println!("got: {:?}", (g_cs, (h_cs, m_cs)));
        (g_cs, (h_cs, m_cs))
    }

    fn sonne(&mut self, chimes: Chime, audio: &mut Audio) {
        println!("against: {:?}", self.past);
        if chimes.0 && !self.past.0 {
            play_sound(1, "grand", audio);
            self.past.0 = chimes.0;
        }
        if chimes.1.0 != self.past.1.0 {
            let n = if self.multi {chimes.1.0 % self.am_pm} else {1};
            play_sound(n, "hour",   audio);
            self.past.1.0 = chimes.1.0;
        }
        if chimes.1.1 != self.past.1.1 {
            let n = if self.multi {60 / chimes.1.1} else {1};
            play_sound(n, "minute", audio);
            self.past.1.1 = chimes.1.1;
        }
    }
}
