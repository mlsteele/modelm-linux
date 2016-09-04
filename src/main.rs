extern crate ears;

mod sound;
mod keys;

fn main() {
    ears::init();

    println!("Hello, world!");

    sound::main();

    let keyboard = keys::Keyboard::new();
    keyboard.start();

    loop {
        let ke = keyboard.rx.recv().unwrap();
        println!("{:?}", ke);
    }
}
