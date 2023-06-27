use serde::{Serialize, Deserialize};
use cpal::{Device, Sample};

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

pub struct Movement {
    am_pm:   u8,
    grand:   Vec<u8>,
    hours:   Vec<u8>,
    minutes: Vec<u8>,
    multi:   Multichime,
    past:    Chime,
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

        Movement {
            am_pm,
            grand,
            hours,
            minutes,
            multi,
            past:   (false, (0, 0)),
        }
    }

    pub fn sonne(&mut self, time: Time, device: &Device) {
        let h = if time.0 != 0 {time.0} else {24};
        let m = time.1;

        let chimes = (self.grand.contains(&h), (if self.hours.contains(&h) {h} else {0}, if self.minutes.contains(&m) {m} else {0}));

        if chimes.0 && !self.past.0 {
            self.past.0 = chimes.0;
        }
        if chimes.1.0 != self.past.1.0 {
            let n = if self.multi.enable {
                chimes.1.0 % self.am_pm / self.multi.h_x
            } else {1};
            self.past.1.0 = chimes.1.0;
        }
        if chimes.1.1 != self.past.1.1 {
            let n = if self.multi.enable {
                chimes.1.1 / self.multi.m_x
            } else {1};
            self.past.1.1 = chimes.1.1;
        }

        use cpal::{
            StreamConfig, SampleRate, BufferSize,
            traits::{DeviceTrait, StreamTrait},
        };

        let config = StreamConfig {
            channels:    2,
            sample_rate: SampleRate(44100),
            buffer_size: BufferSize::Default,
        };
        let stream =
            device
            .build_output_stream(&config,
                                 move |data: &mut [f32], _| {},
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

