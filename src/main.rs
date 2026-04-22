mod objects;
use std::ptr;

fn main() {
    let mut _box: objects::Box = objects::Box::new();
    let mut _text: objects::Text = objects::Text::new();
    _text.changeText("this is a text in a box".to_string());
    let _textenum: objects::objecttypes = objects::objecttypes::TEXT(_text.clone());
    _box.changeItem(ptr::from_ref(&_textenum));
    _box.setPadding(10);
    let _boxenum: objects::objecttypes = objects::objecttypes::BOX(_box.clone());
    println!("{}", _boxenum.toString());
}
