pub mod objects;
pub mod eventmanager;
pub mod xmlconverter;
pub mod util;
use std::{thread, time::Duration};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use eventmanager::EventQueue;
use eventmanager::eventListener;
use eventmanager::event;
use eventmanager::Key;
use crossterm::terminal::disable_raw_mode;
use crate::xmlconverter::parseDocument;

fn main() {
    let mut root = parseDocument("/home/rt/Dev/stui/gui.xml");
    let duration = Duration::from_millis(100);
    let (sendint, recvint): (Sender<i32>, Receiver<i32>) = channel();
    let (sendevent, recvevent): (Sender<EventQueue>, Receiver<EventQueue>) = channel();
    thread::spawn(
        move || {
            eventListener(recvint, sendevent);
        }
    );
    while true {
        print!("{}\n\r", root.toString());
        thread::sleep(duration);
        sendint.send(0);
        let mut queue = recvevent.recv().unwrap();
        while !queue.isEmpty() {
            let item = queue.pop();
            match item {
                event::KEYEVENT(c) => {
                        match c.clone() {
                            Key::ESCAPEKEY(c) => {
                                disable_raw_mode();
                                return;
                            },
                            _ => {},
                        }
                        root.newKeyboardInput(c);
                    },
                _ => {},
            }
        }
        root.Reset();
        print!("{}", root.getResetString());
    }
}
