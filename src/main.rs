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
    let (mut root, mut signals) = parseDocument("./gui.xml");
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
        let tmpqueue = recvevent.try_recv();
        if tmpqueue.is_ok() {
            let mut queue = tmpqueue.unwrap();
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
        }
        root.Reset();
        print!("{}", root.getResetString());
        let mut progress = root.getObjectByName("progress");
        if progress.len() != 0 {
            let mut progressref = progress[0].borrow_mut();
            let mut currentprogressvalue = progressref.convertToProgress().getValue();
            if currentprogressvalue > 10.0 {
                currentprogressvalue = -1.0;
            }
            progressref.convertToProgress().setValue(currentprogressvalue + 1.0);
        }
        for (name, signal) in signals.as_ref().borrow().iter() {
            let (formname, formdata) = signal.try_recv().unwrap_or(("".to_owned(), "".to_owned()));
            if formname.len() > 0 {
                ;
            }
        }
    }
}
