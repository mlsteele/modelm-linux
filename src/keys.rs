use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::str;
use std::string::String;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum KeyMotion {
    Press,
    Release,
}

#[derive(Debug)]
pub struct KeyEvent {
    pub motion: KeyMotion,
    // Whether the key was already in this state.
    pub code: i32,
    pub already: bool,
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
            let tx2 = self.tx.clone();
            thread::spawn(move || {
                // Map from key code to its last state.
                let mut state: HashMap<i32, KeyMotion> = HashMap::new();

                let mut stdout = BufReader::new(c.stdout.take().unwrap());
                for line in stdout.lines() {
                    let mut ke = parse_key_line(&line.unwrap()).unwrap();

                    let argh = KeyMotion::Release;
                    let prev = state.get(&ke.code.clone()).or(Some(&argh)).unwrap().clone();

                    if ke.motion == prev {
                        ke.already = true;
                    }

                    state.insert(ke.code.clone(), ke.motion.clone());
                    tx2.send(ke).expect("send should send");
                }
            });
        } else {
            println!("Could not spawn command");
        }
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
        already: false,
    })
}
