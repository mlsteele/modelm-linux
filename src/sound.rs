use ears::{Sound, AudioController};

pub fn main() {
    let path = "/home/miles/code/vendor/modelm-linux/resources/modelm/1_.wav";
    println!("Loading: {}", path);
    if let Some(mut s) = Sound::new(path) {
        println!("Loaded sound");
        s.play();
        while s.is_playing() {}
    } else {
        println!("Could not load sound");
    }
}
