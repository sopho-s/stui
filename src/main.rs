mod objects;
mod eventmanager;
use std::ptr;
use std::{thread, time::Duration};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use crossterm::event::KeyEvent;
use eventmanager::EventQueue;
use eventmanager::eventListener;
use eventmanager::event;
use eventmanager::Key;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    let mut _inputenum = Input!(30, 2, "placeholder".to_string());
    let mut _boxenum: objects::objecttypes = Box!(
        _inputenum
        , true, 2, 2, 2, 2);
    let mut _rowenum: objects::objecttypes = Row!(
        (_inputenum,
        _inputenum),
         2);
    let mut _boxenum2: objects::objecttypes = Box!(
        _rowenum
        , true, 2, 2, 2, 2);
    let duration = Duration::from_millis(100);
    let (sendint, recvint): (Sender<i32>, Receiver<i32>) = channel();
    let (sendevent, recvevent): (Sender<EventQueue>, Receiver<EventQueue>) = channel();
    thread::spawn(
        move || {
            eventListener(recvint, sendevent);
        }
    );
    while true {
        print!("{}\n\r", _boxenum.toString());
        thread::sleep(duration);
        sendint.send(0);
        let mut queue = recvevent.recv().unwrap();
        while !queue.isEmpty() {
            let item = queue.pop();
            match item {
                event::KEYEVENT(c) => {
                    match c {
                        Key::BASICKEY(s) => {
                            let letters: Vec<char> = s.chars().collect();
                            for letter in letters {
                                _boxenum.newKeyboardInput(letter);
                            }
                        },
                        Key::DELETEKEY(s) => {
                            _boxenum.newKeyboardInput('\x08');
                        }
                        Key::ESCAPEKEY(s) => {
                            disable_raw_mode().unwrap();
                            return
                        }
                        _ => {},
                    }
                }
                _ => {},
            }
        }
    }
}
