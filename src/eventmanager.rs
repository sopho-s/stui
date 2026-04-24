use std::sync::mpsc::{Sender, Receiver};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Clone, Debug)]
pub enum event {
    KEYEVENT(Key)
}

impl event {
    pub fn toKeyEvent(&self) -> Key {
        match self {
            event::KEYEVENT(c) => c.clone(),
            _ => panic!("method on object not supported"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Key {
    BASICKEY(String),
    MOVEMENTKEY(String),
}

impl Key {
    pub fn toString(&self) -> String {
        match self {
            Key::BASICKEY(c) => c.clone(),
            Key::MOVEMENTKEY(c) => c.clone(),
            _ => panic!("method on object not supported"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EventQueue {
    events: Vec<event>,
}

impl EventQueue {
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

pub fn eventListener(rx: Receiver<bool>, tx: Sender<EventQueue>) {
    let mut eventqueue = EventQueue {
        events: vec![],
    };
    let mut s = "".to_string();
    enable_raw_mode().unwrap();
    while true {
        if (!rx.try_recv().unwrap_or(false)) {
            tx.send(eventqueue.clone());
            eventqueue = EventQueue {
                events: vec![],
            };
        } else {
            if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
                match code {
                    KeyCode::Char(c) => eventqueue.push(event::KEYEVENT(Key::BASICKEY(c.to_string()))),
                    KeyCode::Up => eventqueue.push(event::KEYEVENT(Key::MOVEMENTKEY("up".to_string()))),
                    KeyCode::Down => eventqueue.push(event::KEYEVENT(Key::MOVEMENTKEY("down".to_string()))),
                    KeyCode::Left => eventqueue.push(event::KEYEVENT(Key::MOVEMENTKEY("left".to_string()))),
                    KeyCode::Right => eventqueue.push(event::KEYEVENT(Key::MOVEMENTKEY("right".to_string()))),
                    KeyCode::Enter  => eventqueue.push(event::KEYEVENT(Key::BASICKEY("\n".to_string()))),
                    _               => {},
                }
            }
        }
    }
}