use std::{ thread
         , time::{SystemTime, Duration}
         };

use rusty_audio::Audio;
use serde::{Serialize, Deserialize};

fn main() {
    let (config, path) = load_config();
    let mut audio = Audio::new();
    load_audio(&mut audio, &path, &config.sonnerie);

    let mut last_chime = (true, 0, 0, 0);
    loop {
        let time = get_time(config.offset);
        let chime = get_bells(time);
        if chime.0 && !last_chime.0 {play_sound(1,       "grande", &mut audio)}
        if chime.1 != last_chime.1  {play_sound(chime.1, "hour",   &mut audio)}
        if chime.2 != last_chime.2  {play_sound(chime.2, "minute", &mut audio)}
        last_chime = chime;
        thread::sleep(Duration::from_secs(1));
    }
}

fn load_config() -> (Config, std::path::PathBuf) {
    let app_name = "grande-sonnerie";
    let config_name = "config";
    let config: Config =
        confy::load(app_name, config_name)
        .unwrap_or_default();
    let mut path =
        confy::get_configuration_file_path(app_name, config_name)
        .unwrap_or_default();
    path.pop();
    (config, path)
}

fn load_audio(audio: &mut Audio, path: &std::path::PathBuf, theme: &str) {
    [ "grande"
    , "hour"
    , "minute"
    ]
    .iter()
    .for_each(|x| audio.add(x, path.join(theme).join(x.to_string() + ".wav")));
}

fn get_time(offset: [i64; 3]) -> (u64, u64, u64) {
    let now =
        SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time before Unix epoch is not supported")
        .as_secs()
        .saturating_add_signed(offset[0] * 3600 + offset[1] * 60 + offset[2]);
    let h = (now / 3600) % 24;
    let m = (now / 60  ) % 60;
    let s = (now       ) % 60;
    println!("{h}:{m}:{s}");
    (h, m, s)
}

fn get_bells(time: (u64, u64, u64)) -> (bool, u64, u64, u64) {
    let h = time.0 % 12;
    let g = h == 0;   
    let m = time.1 / 15;
    (g, h, m, 0)
}

fn play_sound(n: u64, file: &str, audio: &mut Audio) {
    (0..n).for_each(|_| {
        audio.play(file);
        audio.wait();
    });
}

#[derive(Serialize, Deserialize)]
struct Config { offset   : [i64; 3]
              , sonnerie : String
              }

impl std::default::Default for Config {
    fn default() -> Self {
        Config { offset   : [0; 3]
               , sonnerie : "coucou".to_string()
               }
    }
}
