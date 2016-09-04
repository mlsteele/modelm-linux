extern crate ears;
extern crate rand;

mod sound;
mod keys;

use std::io;
use std::thread;
use rand::Rng;
use rand::distributions::{Range, IndependentSample};
use ears::{Sound, AudioController};
use keys::KeyMotion;

fn main() {
    match progn() {
        Ok(_) => (),
        Err(err) => println!("Fatal: {}", err),
    }
}

fn progn() -> Result<(), Box<std::error::Error>> {
    ears::init();

    println!("Hello, world!");

    let mut sounds = try!(sound::load_dir("resources/modelm"));
    if sounds.len() < 1 {
        return try!(Err(io::Error::new(io::ErrorKind::Other, "no sounds loaded")));
    }

    let keyboard = keys::Keyboard::new();
    keyboard.start();

    loop {
        let ke = keyboard.rx.recv().unwrap();
        println!("{:?}", ke);
        if let KeyMotion::Press = ke.motion {
            let range = Range::new(0, sounds.len());
            let i = range.ind_sample(&mut rand::thread_rng());
            let s = &mut sounds[i];
            s.play();
        }
    }
}
