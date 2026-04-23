mod objects;
use std::ptr;

fn main() {
    let mut _box: objects::Box = objects::Box::new();
    let mut _text: objects::Text = objects::Text::new();
    _text.changeText("this is a text in a box\nwith 2 lines".to_string());
    let _textenum: objects::objecttypes = objects::objecttypes::TEXT(_text.clone());
    let mut _text0: objects::Text = objects::Text::new();
    _text0.changeText("this is a text".to_string());
    let _textenum0: objects::objecttypes = objects::objecttypes::TEXT(_text0.clone());
    _box.setBorder(true);
    _box.changeItem(ptr::from_ref(&_textenum));
    _box.setPadding(2);
    let mut _boxenum = objects::objecttypes::BOX(_box.clone());
    let mut _column = objects::Column::new();
    _column.addItem(ptr::from_ref(&_textenum));
    _column.addItem(ptr::from_ref(&_boxenum));
    _column.addItem(ptr::from_ref(&_textenum));
    let _columnenum: objects::objecttypes = objects::objecttypes::COLUMN(_column.clone());

    let mut _box2: objects::Box = objects::Box::new();
    _box2.setBorder(true);
    _box2.changeItem(ptr::from_ref(&_columnenum));
    _box2.setPadding(2);
    let mut _boxenum2 = objects::objecttypes::BOX(_box2.clone());

    let mut _row: objects::Row = objects::Row::new();
    _row.setGap(10);
    _row.addItem(ptr::from_ref(&_boxenum2));
    _row.addItem(ptr::from_ref(&_boxenum));
    let mut _rowenum = objects::objecttypes::ROW(_row.clone());

    let mut _box3: objects::Box = objects::Box::new();
    _box3.setBorder(true);
    _box3.changeItem(ptr::from_ref(&_rowenum));
    _box3.setPadding(0);
    let mut _boxenum3 = objects::objecttypes::BOX(_box3.clone());

    println!("{}", _box3.toString());
}
