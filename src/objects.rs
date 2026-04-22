use std::str;
use std::ptr;

#[derive(Clone, Debug)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new() -> Text {
        return Text {text: "".to_string()};
    }
    pub fn toString(&self) -> String {
        return self.text.clone();
    }
    pub fn changeText(&mut self, text: String) {
        self.text = text;
    }
}

#[derive(Clone, Debug)]
pub struct Box {
    item: *const objecttypes,
    padding_left: i32,
    padding_right: i32,
    padding_up: i32,
    padding_down: i32,
}

impl Box {
    pub fn new() -> Box {
        return Box {
            item: ptr::null(),
            padding_left: 0,
            padding_right: 0,
            padding_up: 0,
            padding_down: 0,
        };
    }
    pub fn toString(&self) -> String {
        let mut returnstring: String = "".to_string();
        let mut leftpad = "".to_string();
        for i in 0..self.padding_left {
            leftpad += " ";
        }
        let mut rightpad = "".to_string();
        for i in 0..self.padding_right {
            rightpad += " ";
        }
        for i in 0..self.padding_up {
            returnstring += "\n";
        }
        unsafe {
            returnstring += &((leftpad + &((self.item).as_ref().unwrap().toString().clone())) + &rightpad);
        }
        for i in 0..self.padding_down {
            returnstring += "\n";
        }
        return returnstring;
    }
    pub fn changeItem(&mut self, item: *const objecttypes) {
        self.item = item;
    }
    pub fn setPadding(&mut self, padding: i32) {
        self.padding_down = padding;
        self.padding_up = padding;
        self.padding_right = padding;
        self.padding_left = padding;
    }
}


#[derive(Clone, Debug)]
pub enum objecttypes {
    TEXT(Text),
    BOX(Box),
}

impl objecttypes {

    pub fn toString(&self) -> String {
        match self {
            objecttypes::TEXT(c) => c.toString(),
            objecttypes::BOX(c) => c.toString(),
            _ => panic!("method on object not supported"),
        }
    }
}