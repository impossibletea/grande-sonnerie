use std::{
    fs::File,
    io::BufReader
};
use time::OffsetDateTime;
use rodio::{Decoder, OutputStream, Sink};

fn main() {
    let now = OffsetDateTime::now_local().unwrap().time();
    let (h, m) = (now.hour(), now.minute());
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open("test.wav").unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
