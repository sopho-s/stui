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
use std::vec;

fn linkSelectors(idlist: Rc<RefCell<Vec<i32>>>, nodelist: Rc<RefCell<Vec<Rc<RefCell<objecttypes>>>>>, selectorlist: Rc<RefCell<Vec<Rc<RefCell<objecttypes>>>>>, selectorwants: Rc<RefCell<Vec<Vec<i32>>>>) {
    for i in 0..selectorwants.as_ref().borrow().len() {
        let next = selectorwants.as_ref().borrow()[i][0];
        let previous = selectorwants.as_ref().borrow()[i][1];
        let nextindex = idlist.as_ref().borrow().iter().position(|&r| r == next).unwrap();
        let previousindex = idlist.as_ref().borrow().iter().position(|&r| r == previous).unwrap();
        let refer = &selectorlist.as_ref().borrow_mut()[i];
        let mut refer2 = refer.as_ref().borrow_mut();
        let selector = refer2.convertToSelector();
        let nextelem = Rc::clone(&nodelist.as_ref().borrow()[nextindex]);
        let previouselem = Rc::clone(&nodelist.as_ref().borrow()[previousindex]);
        selector.setElements(nextelem, previouselem);
    }
}

fn parseXML(doc: roxmltree::Node, idlist: Rc<RefCell<Vec<i32>>>, nodelist: Rc<RefCell<Vec<Rc<RefCell<objecttypes>>>>>, selectorlist: Rc<RefCell<Vec<Rc<RefCell<objecttypes>>>>>, selectorwants: Rc<RefCell<Vec<Vec<i32>>>>) -> Rc<RefCell<objecttypes>> {
    for node in doc.descendants().filter(|n| n.is_element()) {
        match node.tag_name().name() {
            "Box" => {
                let object = Box!(
                    parseXML(node.first_element_child().unwrap(), Rc::clone(&idlist), Rc::clone(&nodelist), Rc::clone(&selectorlist), Rc::clone(&selectorwants)),
                    node.attribute("hasborder").unwrap_or("false") == "true",
                    node.attribute("paddingleft").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingright").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingup").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingdown").unwrap_or("0").parse::<i32>().unwrap()
                );
                let thisobject = Rc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Rc::clone(&thisobject));
                }
                return thisobject;
            },
            "Text" => {
                let object = Text!(node.attribute("text").unwrap_or("").to_owned(),
                    node.attribute("length").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("height").unwrap_or("0").parse::<i32>().unwrap()
                );
                let thisobject = Rc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Rc::clone(&thisobject));
                }
                return thisobject;
            },
            "Row" => {
                let mut vec = vec![];
                for item in node.children() {
                    if item.is_element() {
                        vec.push(parseXML(item, Rc::clone(&idlist), Rc::clone(&nodelist), Rc::clone(&selectorlist), Rc::clone(&selectorwants)));
                    }
                }
                let object = objects::objecttypes::ROW(
                    objects::Row::new(
                        Some(vec),
                        Some(node.attribute("gap").unwrap_or("0").parse::<i32>().unwrap())
                    )
                );
                let thisobject = Rc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Rc::clone(&thisobject));
                }
                return thisobject;
            },
            "Column" => {
                let mut vec = vec![];
                for item in node.children() {
                    if item.is_element() {
                        vec.push(parseXML(item, Rc::clone(&idlist), Rc::clone(&nodelist), Rc::clone(&selectorlist), Rc::clone(&selectorwants)));
                    }
                }
                let object = objects::objecttypes::COLUMN(
                    objects::Column::new(
                        Some(vec),
                        Some(node.attribute("gap").unwrap_or("0").parse::<i32>().unwrap())
                    )
                );
                let thisobject = Rc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Rc::clone(&thisobject));
                }
                return thisobject;
            },
            "Input" => {
                let object = Input!(node.attribute("length").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("height").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("placeholder").unwrap_or("").to_owned()
                );
                let thisobject = Rc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Rc::clone(&thisobject));
                }
                return thisobject;
            },
            "Selector" => {
                let object = objects::objecttypes::SELECTOR(
                    objects::Selector::new(
                        Some(parseXML(node.first_element_child().unwrap(), Rc::clone(&idlist), Rc::clone(&nodelist), Rc::clone(&selectorlist), Rc::clone(&selectorwants))),
                        None,
                        None,
                        Some(node.attribute("isactive").unwrap_or("false").to_owned() == "true")
                    )
                );
                let thisobject = Rc::new(RefCell::new(object));
                selectorlist.borrow_mut().push(Rc::clone(&thisobject));
                selectorwants.borrow_mut().push(vec![node.attribute("next").unwrap_or("-1").parse::<i32>().unwrap(), node.attribute("previous").unwrap_or("-1").parse::<i32>().unwrap()]);
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Rc::clone(&thisobject));
                }
                return thisobject;
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
    let idlist = Rc::new(RefCell::new(vec![]));
    let nodelist = Rc::new(RefCell::new(vec![]));
    let selectorlist = Rc::new(RefCell::new(vec![]));
    let selectorwants = Rc::new(RefCell::new(vec![]));
    let parsedroot = parseXML(root, Rc::clone(&idlist), Rc::clone(&nodelist), Rc::clone(&selectorlist), Rc::clone(&selectorwants));
    linkSelectors(idlist, nodelist, selectorlist, selectorwants);
    return (*parsedroot.as_ref().borrow()).clone()
}