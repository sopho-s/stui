mod objects;
mod eventmanager;
use std::ptr;
use std::{thread, time::Duration};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use eventmanager::EventQueue;
use eventmanager::eventListener;


fn main() {
    let mut _box: objects::Box = objects::Box::new();
    let mut _input: objects::Input = objects::Input::new();
    _input.setHeight(2);
    _input.setLength(20);
    let mut _inputenum: objects::objecttypes = objects::objecttypes::INPUT(_input.clone());
    _box.setBorder(true);
    _box.changeItem(ptr::from_mut(&mut _inputenum));
    _box.setPadding(2);
    let mut _boxenum: objects::objecttypes = objects::objecttypes::BOX(_box.clone());
    let duration = Duration::from_millis(200);
    let mut i = 0;
    while true {
        _boxenum.newKeyboardInput('a');
        print!("{}\n\r", _boxenum.toString());
        i += 1;
        thread::sleep(duration);
        let (sendbool, recvbool): (Sender<bool>, Receiver<bool>) = channel();
        let (sendevent, recvevent): (Sender<EventQueue>, Receiver<EventQueue>) = channel();
        thread::spawn(
            move || {
                eventListener(recvbool, sendevent);
            }
        );
    }
}
