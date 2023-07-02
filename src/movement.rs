use cpal::Device;
use serde::{Serialize, Deserialize};

mod sonnerie;
use sonnerie::Sonnerie;

pub type Time  = (u8, u8);
type Chime = (bool, Time);

#[derive(Serialize, Deserialize)]
struct MovementConfig {
    grand:       Option<Vec<u8>>,
    hours:       Option<Vec<u8>>,
    hours_div:   Option<u8>,
    minutes:     Option<Vec<u8>>,
    minutes_div: Option<u8>,
    twelve_hour: bool,
    multichime:  bool,
    sonnerie:    String,
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
            sonnerie:    "coucou".to_string(),
        }
    }
}

pub struct Movement {
    am_pm:    u8,
    grand:    Vec<u8>,
    hours:    Vec<u8>,
    minutes:  Vec<u8>,
    multi:    Multichime,
    past:     Chime,
    sonnerie: Sonnerie,
}

struct Multichime {
    enable: bool,
    h_x:    u8,
    m_x:    u8,
}

impl Movement {
    pub fn new(config_name: &str) -> Self {
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

        let h_x =
            if let Some(h_div) = config.hours_div {h_div} else {1};
        let m_x =
            if let Some(m_div) = config.minutes_div {m_div} else {1};
        let multi =  Multichime {
            enable: config.multichime,
            h_x,
            m_x,
        };

        let sonnerie = Sonnerie::new(&config.sonnerie);

        Movement {
            am_pm,
            grand,
            hours,
            minutes,
            multi,
            past:   (false, (0, 0)),
            sonnerie,
        }
    }

    pub fn sonne(&mut self, time: Time, device: &Device) {
        let h = if time.0 != 0 {time.0} else {24};
        let m = time.1;

        let chimes =
            ( self.grand.contains(&h)
            , ( if self.hours.contains(&h) {h} else {0}
              , if self.minutes.contains(&m) {m} else {0}
              )
            );

        let mut bells = (0, 0, 0);

        if chimes.0 && !self.past.0 {
            self.past.0 = chimes.0;
            bells.0 = 1;
        }
        if chimes.1.0 != self.past.1.0 {
            self.past.1.0 = chimes.1.0;
            bells.1 = if self.multi.enable {
                chimes.1.0 % self.am_pm / self.multi.h_x
            } else {1};
        }
        if chimes.1.1 != self.past.1.1 {
            self.past.1.1 = chimes.1.1;
            bells.2 = if self.multi.enable {
                chimes.1.1 / self.multi.m_x
            } else {1};
        }

        let mut data = Vec::new();
        for _g in [0..bells.0] {data.push(self.sonnerie.grand.clone()) }
        for _h in [0..bells.1] {data.push(self.sonnerie.hour.clone())  }
        for _m in [0..bells.2] {data.push(self.sonnerie.minute.clone())}

        use cpal::traits::{DeviceTrait, StreamTrait};

        let config =
            device
            .supported_output_configs()
            .expect("Failed getting audio configs")
            .next()
            .expect("How is there no output configurations?")
            .with_sample_rate(cpal::SampleRate(44100))
            .into();
        let stream =
            device
            .build_output_stream(&config,
                                 move |data: &mut [i16], _| {},
                                 move |err| {eprintln!("Cringe: {err}")},
                                 None)
            .expect("Failed to build sound stream");

        stream.play().expect("Failed to chime");
    }
}

impl std::fmt::Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let multi = if self.multi.enable {""} else {"not "};
        write!(f,
"
Movement will chime the following:
    * Grand:
      {:?}
    * Hours:
      {:?}
    * Minutes:
      {:?}

Chimes will {}be repeated
",
               self.grand,
               self.hours,
               self.minutes,
               multi
        )
    }
}

