use std::process;
use std::process::{Command, Stdio};
use std::io;
use std::io::{BufReader, BufRead};
use std::str;
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

    pub fn start(&self) -> Result<(), io::Error> {
        let cmdstr = "xinput list | grep -Po 'id=\\K\\d+(?=.*slave.*)' | xargs -P0 -n1 \
                      xinput test";
        let c = try!(Command::new("bash")
            .arg("-c")
            .arg(cmdstr)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn());
        let tx2 = self.tx.clone();
        let stdouterr = io::Error::new(io::ErrorKind::Other, "Could not get process stdout");
        let stdout: process::ChildStdout = try!(c.stdout.ok_or(stdouterr));
        let stdout = BufReader::new(stdout);
        thread::spawn(move || {
            // Map from key code to its last state.
            let mut state: HashMap<i32, KeyMotion> = HashMap::new();

            for line in stdout.lines() {
                if let Ok(line) = line {
                    if let Err(err) = Keyboard::loop_inner(&line, &mut state, &tx2) {
                        warn!("{}", err);
                    }
                }
            }
        });
        Ok(())
    }

    fn loop_inner(line: &str,
                  state: &mut HashMap<i32, KeyMotion>,
                  tx2: &SyncSender<KeyEvent>)
                  -> Result<(), io::Error> {
        let mut ke = try!(parse_key_line(&line));

        let argh = KeyMotion::Release;
        let prev = state.get(&ke.code.clone()).or(Some(&argh)).unwrap().clone();

        if ke.motion == prev {
            ke.already = true;
        }

        state.insert(ke.code.clone(), ke.motion.clone());
        tx2.send(ke).expect("send should send");
        Ok(())
    }
}

fn parse_key_line(line: &str) -> Result<KeyEvent, io::Error> {
    let e = Err(io::Error::new(io::ErrorKind::Other,
                               format!("malformed key event: {}", line)));
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
