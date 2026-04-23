mod objects;
use std::ptr;

fn main() {
    let mut _row: objects::Row = objects::Row::new();
    let mut _box: objects::Box = objects::Box::new();
    let mut _text: objects::Text = objects::Text::new();
    _text.changeText("this is a text in a box".to_string());
    let _textenum: objects::objecttypes = objects::objecttypes::TEXT(_text.clone());
    let mut _text0: objects::Text = objects::Text::new();
    _text0.changeText("this is a text".to_string());
    let _textenum0: objects::objecttypes = objects::objecttypes::TEXT(_text0.clone());
    _box.setBorder(true);
    _box.changeItem(ptr::from_ref(&_textenum));
    _box.setPadding(2);
    let _boxenum: objects::objecttypes = objects::objecttypes::BOX(_box.clone());
    _row.addItem(ptr::from_ref(&_textenum));
    _row.addItem(ptr::from_ref(&_boxenum));
    _row.setGap(20);
    let _rowenum: objects::objecttypes = objects::objecttypes::ROW(_row.clone());
    let mut _box1: objects::Box = objects::Box::new();
    _box1.setBorder(true);
    _box1.changeItem(ptr::from_ref(&_rowenum));
    let _boxenum1: objects::objecttypes = objects::objecttypes::BOX(_box1.clone());
    println!("{}", _boxenum1.toString());
}
