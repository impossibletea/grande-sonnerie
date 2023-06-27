use std::{
    thread,
    time::{SystemTime, Duration}
};

use cpal::traits::HostTrait;
use serde::{Serialize, Deserialize};

mod movement;

fn main() {
    let config = Config::new();
    let mut movement = movement::Movement::new(&config.movement);
    println!("{movement}");

    let device =
        cpal::default_host()
        .default_output_device()
        .expect("No output device found");

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
    });

    loop {
        let time = get_time(&config.offset);
        println!("{:0>2}:{:0>2}", time.0, time.1);
        movement.sonne(time, &device);
        thread::sleep(Duration::from_secs(1));
    }
}

fn get_time(offset: &(i64, i64, i64)) -> movement::Time {
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

