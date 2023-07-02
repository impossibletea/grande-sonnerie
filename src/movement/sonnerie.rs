use std::{
    path::PathBuf,
    fs::File,
};

pub struct Sonnerie {
    pub grand:  Vec<i16>,
    pub hour:   Vec<i16>,
    pub minute: Vec<i16>,
}

impl Sonnerie {
    pub fn new(config: &str) -> Self {
        let path =
            confy::get_configuration_file_path("grande-sonnerie", None)
            .unwrap_or_default()
            .parent()
            .expect("Why is empty")
            .join(config);

        let [grand, hour, minute] =
            ["grand", "hour", "minute"]
            .map(|name| load_sound(name, &path));

        Self {
            grand,
            hour,
            minute,
        }
    }
}

fn load_sound(name: &str, path: &PathBuf) -> Vec<i16> {
    let sound_path =
        path
        .join(name.to_string() + ".wav");
    let mut sound = match File::open(&sound_path) {
        Ok(file) => file,
        Err(_)   => {
            eprintln!("Unable to load {} from {}",
                      name,
                      sound_path.display());
            return Vec::new()
        }
    };

    if let Ok((_, data)) = wav::read(&mut sound) {
        if let Ok(sixteens) = data.try_into_sixteen() {return sixteens}
    };

    match
        File::open(&sound_path)
        .map_err(|e| e.to_string())
        .and_then(|mut sound| wav::read(&mut sound)
        .map_err(|e| e.to_string()))
        .and_then(|(_, data)| data.try_into_sixteen()
        .map_err(|_| "Failed to convert to 16bit".to_string())) {
            Ok(samples) => samples,
            Err(e)      => {
                eprintln!("Cringe: {e}");
                Vec::new()
            }
        }
}
