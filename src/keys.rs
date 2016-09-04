use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::str;
use std::string::String;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;

#[derive(Debug)]
pub enum KeyMotion {
    Press,
    Release,
}

#[derive(Debug)]
pub struct KeyEvent {
    pub motion: KeyMotion,
    pub code: i32,
}

pub struct Keyboard {
    tx: SyncSender<KeyEvent>,
    pub rx: Receiver<KeyEvent>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        let (tx, rx) = sync_channel::<KeyEvent>(20);
        Keyboard { tx: tx, rx: rx }
    }

    pub fn start(&self) {
        let cmdstr = "xinput list | grep -Po 'id=\\K\\d+(?=.*slave\\s*keyboard)' | xargs -P0 -n1 \
                      xinput test";
        if let Ok(mut c) = Command::new("bash")
            .arg("-c")
            .arg(cmdstr)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
            println!("spawned");
            let tx2 = self.tx.clone();
            thread::spawn(move || {
                let mut stdout = BufReader::new(c.stdout.take().unwrap());
                // let mut buffer = String::new();
                // while buffered_stderr.read_line(&mut buffer).unwrap() > 0 {
                //     buffer.trim()
                //     buffer.clear();
                // }
                for line in stdout.lines() {
                    // println!("{}", line.unwrap());
                    let ke = parse_key_line(&line.unwrap()).unwrap();
                    tx2.send(ke).expect("send should send");
                }
            });
        } else {
            println!("Could not spawn command");
        }
    }
}

pub fn main() {
    let cmdstr = "xinput list | grep -Po 'id=\\K\\d+(?=.*slave\\s*keyboard)' | xargs -P0 -n1 \
                  xinput test";
    if let Ok(mut c) = Command::new("bash")
        .arg("-c")
        .arg(cmdstr)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() {
        println!("spawned");
        let mut stdout = BufReader::new(c.stdout.take().unwrap());
        // let mut buffer = String::new();
        // while buffered_stderr.read_line(&mut buffer).unwrap() > 0 {
        //     buffer.trim()
        //     buffer.clear();
        // }
        for line in stdout.lines() {
            // println!("{}", line.unwrap());
            let ke = parse_key_line(&line.unwrap()).unwrap();
            println!("{:?}", ke);
        }
    } else {
        println!("Could not spawn command");
    }
}

fn parse_key_line(line: &str) -> Result<KeyEvent, String> {
    let e = Err(format!("malformed key event: {}", line));
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return e;
    }
    if parts[0] != "key" {
        return e;
    }
    let motion = match parts[1] {
        "press" => KeyMotion::Press,
        "release" => KeyMotion::Release,
        _ => return e,
    };
    let code: i32 = match parts[2].parse() {
        Ok(c) => c,
        Err(_) => return e,
    };
    Ok(KeyEvent {
        motion: motion,
        code: code,
    })
}
