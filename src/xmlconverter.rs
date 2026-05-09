use roxmltree::Document;
use sdl2::libc::PACKET_AUXDATA;
use crate::objects;
use crate::Box;
use crate::Text;
use crate::Input;
use crate::objects::objecttypes;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;

fn parseXML(doc: roxmltree::Node) -> objecttypes {
    for node in doc.descendants().filter(|n| n.is_element()) {
        println!("{}", node.tag_name().name());
        match node.tag_name().name() {
            "Box" => {
                println!("{}", node.first_child().unwrap().tag_name().name());
                let object = Box!(
                    Rc::new(RefCell::new(parseXML(node.first_element_child().unwrap()))),
                    node.attribute("hasborder").unwrap_or("false") == "true",
                    node.attribute("paddingleft").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingright").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingup").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingdown").unwrap_or("0").parse::<i32>().unwrap()
                );
                return object;
            },
            "Text" => {
                let object = Text!(node.attribute("text").unwrap_or("").to_owned(),
                    node.attribute("length").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("height").unwrap_or("0").parse::<i32>().unwrap()
                );
                return object;
            },
            "Row" => {
                let mut vec = vec![];
                for item in node.children() {
                    if item.is_element() {
                        vec.push(Rc::new(RefCell::new(parseXML(item))));
                    }
                }
                let object = objects::objecttypes::ROW(
                    objects::Row::new(
                        Some(vec),
                        Some(node.attribute("gap").unwrap_or("").parse::<i32>().unwrap())
                    )
                );
                return object;
            },
            "Column" => {
                let mut vec = vec![];
                for item in node.children() {
                    if item.is_element() {
                        vec.push(Rc::new(RefCell::new(parseXML(item))));
                    }
                }
                let object = objects::objecttypes::COLUMN(
                    objects::Column::new(
                        Some(vec),
                        Some(node.attribute("gap").unwrap_or("").parse::<i32>().unwrap())
                    )
                );
                return object;
            },
            "Input" => {
                let object = Input!(node.attribute("length").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("height").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("placeholder").unwrap_or("").to_owned()
                );
                return object;
            },
            _ => panic!("Invalid XML")
        }
    }
    panic!("no nodes");
}

pub fn parseDocument(filename: &str) -> objecttypes {
    let document = fs::read_to_string(filename).unwrap();
    let doc = roxmltree::Document::parse(&document).unwrap();
    let root = doc.root_element();
    parseXML(root)
}