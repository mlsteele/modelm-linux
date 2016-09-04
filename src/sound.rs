use std::vec;
use std::fs;
use std::io;

use ears::{Sound, AudioController};

pub fn load_dir(path: &str) -> Result<Vec<Sound>, io::Error> {
    let mut v: Vec<Sound> = Vec::new();
    for entry in try!(fs::read_dir(path)) {
        let entry = try!(entry);
        let path = entry.path();
        let path = match path.to_str() {
            Some(s) => s,
            None => return Err(io::Error::new(io::ErrorKind::Other, "bad path")),
        };
        if path.ends_with(".wav") {
            if let Some(s) = Sound::new(path) {
                v.push(s);
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, "Could not load sound"));
            }
        }
    }
    return Ok(v);
}

pub fn main() {
    let path = "/home/miles/code/modelm-linux/resources/modelm/1_.wav";
    println!("Loading: {}", path);
    if let Some(mut s) = Sound::new(path) {
        println!("Loaded sound");
        s.play();
        while s.is_playing() {}
    } else {
        println!("Could not load sound");
    }
}
