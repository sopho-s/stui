use crossterm::event::{Event, KeyCode, KeyEvent, read};
use crossterm::terminal::enable_raw_mode;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Clone, Debug)]
pub enum event {
    KEYEVENT(Key),
}

#[derive(Clone, Debug)]
pub enum Key {
    BASICKEY(String),
    ESCAPEKEY(String),
    DELETEKEY(String),
    MOVEMENTKEY(String),
}

#[derive(Clone, Debug)]
pub struct EventQueue {
    events: Vec<event>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue { events: vec![] }
    }

    pub fn push(&mut self, item: event) {
        self.events.push(item);
    }

    pub fn pop(&mut self) -> event {
        let queueclone = self.events.clone();
        let event = queueclone.get(0).unwrap();
        self.events.remove(0 as usize);
        event.clone()
    }

    pub fn isEmpty(&self) -> bool {
        if self.events.len() == 0 {
            return true;
        }
        false
    }
}

pub fn eventListener(rx: Receiver<i32>, tx: Sender<EventQueue>) {
    let mut eventqueue = EventQueue { events: vec![] };
    let s = "".to_string();
    enable_raw_mode().unwrap();
    while true {
        let recvval = rx.try_recv().unwrap_or(-1);
        if recvval == 0 {
            tx.send(eventqueue.clone());
            eventqueue = EventQueue { events: vec![] };
        } else if recvval == 1 {
            return;
        } else {
            if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
                match code {
                    KeyCode::Char(c) => {
                        eventqueue.push(event::KEYEVENT(Key::BASICKEY(c.to_string())))
                    }
                    KeyCode::Up => {
                        eventqueue.push(event::KEYEVENT(Key::MOVEMENTKEY("up".to_string())))
                    }
                    KeyCode::Down => {
                        eventqueue.push(event::KEYEVENT(Key::MOVEMENTKEY("down".to_string())))
                    }
                    KeyCode::Left => {
                        eventqueue.push(event::KEYEVENT(Key::MOVEMENTKEY("left".to_string())))
                    }
                    KeyCode::Right => {
                        eventqueue.push(event::KEYEVENT(Key::MOVEMENTKEY("right".to_string())))
                    }
                    KeyCode::Enter => {
                        eventqueue.push(event::KEYEVENT(Key::ESCAPEKEY("\n".to_string())))
                    }
                    KeyCode::Delete => {
                        eventqueue.push(event::KEYEVENT(Key::DELETEKEY("delete".to_string())))
                    }
                    KeyCode::Backspace => {
                        eventqueue.push(event::KEYEVENT(Key::DELETEKEY("delete".to_string())))
                    }
                    KeyCode::Esc => {
                        eventqueue.push(event::KEYEVENT(Key::ESCAPEKEY("escape".to_string())))
                    }
                    _ => {}
                }
            }
        }
    }
}
