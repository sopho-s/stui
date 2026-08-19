use crate::objects;
use crate::Box;
use crate::Text;
use crate::Input;
use crate::objects::objecttypes;
use crate::objects::Colour;
use crate::objects::Effect;
use crate::Button;
use std::sync::Arc;
use std::cell::RefCell;
use std::fs;
use std::vec;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc::channel;

fn linkSelectors(idlist: Arc<RefCell<Vec<i32>>>, nodelist: Arc<RefCell<Vec<Arc<RefCell<objecttypes>>>>>, selectorlist: Arc<RefCell<Vec<Arc<RefCell<objecttypes>>>>>, selectorwants: Arc<RefCell<Vec<Vec<i32>>>>) {
    for i in 0..selectorwants.as_ref().borrow().len() {
        let refer = &selectorlist.as_ref().borrow_mut()[i];
        let mut refer2 = refer.as_ref().borrow_mut();
        let selector = refer2.convertToSelector();
        let mut elemvec = vec![];
        for t in 0..4 {
            let currelem;
            let selectorwant = selectorwants.as_ref().borrow()[i][t];
            if selectorwant != -1 {
                let currindex = idlist.as_ref().borrow().iter().position(|&r| r == selectorwant).unwrap();
                currelem = Some(Arc::clone(&nodelist.as_ref().borrow()[currindex]));
            } else {
                currelem = None;
            }
            elemvec.push(currelem);
        }
        selector.setElements(elemvec[0].clone(), elemvec[1].clone(), elemvec[2].clone(), elemvec[3].clone());
    }
}

fn linkButtons(idlist: Arc<RefCell<Vec<i32>>>, nodelist: Arc<RefCell<Vec<Arc<RefCell<objecttypes>>>>>, buttonlist: Arc<RefCell<Vec<Arc<RefCell<objecttypes>>>>>, buttonwants: Arc<RefCell<Vec<i32>>>) {
    for i in 0..buttonwants.as_ref().borrow().len() {
        let refer = &buttonlist.as_ref().borrow_mut()[i];
        let buttonwant = buttonwants.as_ref().borrow()[i];
        let currindex = idlist.as_ref().borrow().iter().position(|&r| r == buttonwant).unwrap();
        let mut refer2 = refer.as_ref().borrow_mut();
        let button = refer2.convertToButton();
        button.setElement(Some(Arc::clone(&nodelist.as_ref().borrow()[currindex])));
    }
}

fn parseXML(doc: roxmltree::Node, idlist: Arc<RefCell<Vec<i32>>>, nodelist: Arc<RefCell<Vec<Arc<RefCell<objecttypes>>>>>, selectorlist: Arc<RefCell<Vec<Arc<RefCell<objecttypes>>>>>, selectorwants: Arc<RefCell<Vec<Vec<i32>>>>, buttonlist: Arc<RefCell<Vec<Arc<RefCell<objecttypes>>>>>, buttonwants: Arc<RefCell<Vec<i32>>>, signals: Arc<RefCell<Vec<(String, Receiver<(String, String)>)>>>) -> Arc<RefCell<objecttypes>> {
    for node in doc.descendants().filter(|n| n.is_element()) {
        let colour;
        if node.attribute("colour").unwrap_or("") != "" {
            colour = Some(Colour::new(node.attribute("colour").unwrap().to_owned()));
        } else {
            colour = None;
        }
        let effect = Some(Effect::new(colour));
        match node.tag_name().name() {
            "Box" => {
                let mut object = Box!(
                    parseXML(node.first_element_child().unwrap(), Arc::clone(&idlist), Arc::clone(&nodelist), Arc::clone(&selectorlist), Arc::clone(&selectorwants), Arc::clone(&buttonlist), Arc::clone(&buttonwants), Arc::clone(&signals)),
                    node.attribute("hasborder").unwrap_or("false") == "true",
                    node.attribute("paddingleft").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingright").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingup").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingdown").unwrap_or("0").parse::<i32>().unwrap(),
                    effect
                );
                let name = node.attribute("name").unwrap_or("");
                object.setName(name);
                let thisobject = Arc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Arc::clone(&thisobject));
                }
                return thisobject;
            },
            "Text" => {
                let mut object = Text!(node.attribute("text").unwrap_or("").to_owned(),
                    node.attribute("length").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("height").unwrap_or("0").parse::<i32>().unwrap(),
                    effect
                );
                let name = node.attribute("name").unwrap_or("");
                object.setName(name);
                let thisobject = Arc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Arc::clone(&thisobject));
                }
                return thisobject;
            },
            "Row" => {
                let mut vec = vec![];
                for item in node.children() {
                    if item.is_element() {
                        vec.push(parseXML(item, Arc::clone(&idlist), Arc::clone(&nodelist), Arc::clone(&selectorlist), Arc::clone(&selectorwants), Arc::clone(&buttonlist), Arc::clone(&buttonwants), Arc::clone(&signals)));
                    }
                }
                let mut object = objects::objecttypes::ROW(
                    objects::Row::new(
                        Some(vec),
                        Some(node.attribute("gap").unwrap_or("0").parse::<i32>().unwrap()),
                        effect
                    )
                );
                let name = node.attribute("name").unwrap_or("");
                object.setName(name);
                let thisobject = Arc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Arc::clone(&thisobject));
                }
                return thisobject;
            },
            "Column" => {
                let mut vec = vec![];
                for item in node.children() {
                    if item.is_element() {
                        vec.push(parseXML(item, Arc::clone(&idlist), Arc::clone(&nodelist), Arc::clone(&selectorlist), Arc::clone(&selectorwants), Arc::clone(&buttonlist), Arc::clone(&buttonwants), Arc::clone(&signals)));
                    }
                }
                let mut object = objects::objecttypes::COLUMN(
                    objects::Column::new(
                        Some(vec),
                        Some(node.attribute("gap").unwrap_or("0").parse::<i32>().unwrap()),
                        effect
                    )
                );
                let name = node.attribute("name").unwrap_or("");
                object.setName(name);
                let thisobject = Arc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Arc::clone(&thisobject));
                }
                return thisobject;
            },
            "Input" => {
                let name = node.attribute("name").unwrap_or("").to_owned();
                let mut object = Input!(node.attribute("length").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("height").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("placeholder").unwrap_or("").to_owned(),
                    effect,
                    name
                );
                let thisobject = Arc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Arc::clone(&thisobject));
                }
                return thisobject;
            },
            "Selector" => {
                let activecolour;
                if node.attribute("activecolour").unwrap_or("") != "" {
                    activecolour = Some(Colour::new(node.attribute("activecolour").unwrap().to_owned()));
                } else {
                    activecolour = None;
                }
                let activeeffect = Some(Effect::new(activecolour));
                let mut object = objects::objecttypes::SELECTOR(
                    objects::Selector::new(
                        Some(parseXML(node.first_element_child().unwrap(), Arc::clone(&idlist), Arc::clone(&nodelist), Arc::clone(&selectorlist), Arc::clone(&selectorwants), Arc::clone(&buttonlist), Arc::clone(&buttonwants), Arc::clone(&signals))),
                        None,
                        None,
                        None,
                        None,
                        Some(node.attribute("isactive").unwrap_or("false").to_owned() == "true"),
                        effect,
                        activeeffect
                    )
                );
                let name = node.attribute("name").unwrap_or("");
                object.setName(name);
                let thisobject = Arc::new(RefCell::new(object));
                selectorlist.borrow_mut().push(Arc::clone(&thisobject));
                selectorwants.borrow_mut().push(vec![
                    node.attribute("right").unwrap_or("-1").parse::<i32>().unwrap(),
                    node.attribute("left").unwrap_or("-1").parse::<i32>().unwrap(),
                    node.attribute("up").unwrap_or("-1").parse::<i32>().unwrap(),
                    node.attribute("down").unwrap_or("-1").parse::<i32>().unwrap()
                    ]);
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Arc::clone(&thisobject));
                }
                return thisobject;
            },
            "Form" => {
                let name = node.attribute("name").unwrap_or("").to_owned();
                let (sender, reciever) = channel::<(String, String)>();
                let mut object = objects::objecttypes::FORM(
                    objects::Form::new(
                        Some(parseXML(node.first_element_child().unwrap(), Arc::clone(&idlist), Arc::clone(&nodelist), Arc::clone(&selectorlist), Arc::clone(&selectorwants), Arc::clone(&buttonlist), Arc::clone(&buttonwants), Arc::clone(&signals))),
                        sender,
                        name.clone()
                    )
                );
                signals.as_ref().borrow_mut().push((name, reciever));
                let thisobject = Arc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Arc::clone(&thisobject));
                }
                return thisobject;
            },
            "Button" => {
                let mut object = Button!(node.attribute("text").unwrap_or("").to_owned(),
                    node.attribute("length").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("height").unwrap_or("0").parse::<i32>().unwrap(),
                    None,
                    effect.unwrap()
                );
                let name = node.attribute("name").unwrap_or("");
                object.setName(name);
                let thisobject = Arc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Arc::clone(&thisobject));
                }
                buttonlist.borrow_mut().push(Arc::clone(&thisobject));
                buttonwants.borrow_mut().push(node.attribute("link").unwrap_or("-1").parse::<i32>().unwrap());
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Arc::clone(&thisobject));
                }
                return thisobject;
            },
            "Hidden" => {
                let mut object = objects::objecttypes::HIDDEN(
                    objects::Hidden::new(
                        Some(parseXML(node.first_element_child().unwrap(), Arc::clone(&idlist), Arc::clone(&nodelist), Arc::clone(&selectorlist), Arc::clone(&selectorwants), Arc::clone(&buttonlist), Arc::clone(&buttonwants), Arc::clone(&signals)))
                    )
                );
                let name = node.attribute("name").unwrap_or("");
                object.setName(name);
                let thisobject = Arc::new(RefCell::new(object));
                return thisobject;
            },
            "Progress" => {
                let mut object = objects::objecttypes::PROGRESS(
                    objects::Progress::new(
                        node.attribute("min").unwrap_or("0").parse::<f32>().unwrap(),
                        node.attribute("max").unwrap_or("0").parse::<f32>().unwrap(),
                        node.attribute("height").unwrap_or("0").parse::<i32>().unwrap(),
                        node.attribute("length").unwrap_or("0").parse::<i32>().unwrap(),
                        node.attribute("preset").unwrap_or("0").parse::<i8>().unwrap(),
                        node.attribute("showpercent").unwrap_or("0").parse::<i8>().unwrap(),
                        Some(node.attribute("value").unwrap_or("0").parse::<f32>().unwrap()),
                    )
                );
                let name = node.attribute("name").unwrap_or("");
                object.setName(name);
                let thisobject = Arc::new(RefCell::new(object));
                return thisobject;
            },
            _ => panic!("Invalid XML")
        }
    }
    panic!("no nodes");
}

pub fn parseDocument(filename: &str) -> (objecttypes, Arc<RefCell<Vec<(String, Receiver<(String, String)>)>>>) {
    let document = fs::read_to_string(filename).unwrap();
    let doc = roxmltree::Document::parse(&document).unwrap();
    let root = doc.root_element();
    let idlist = Arc::new(RefCell::new(vec![]));
    let nodelist = Arc::new(RefCell::new(vec![]));
    let selectorlist = Arc::new(RefCell::new(vec![]));
    let selectorwants = Arc::new(RefCell::new(vec![]));
    let buttonlist = Arc::new(RefCell::new(vec![]));
    let buttonwants = Arc::new(RefCell::new(vec![]));
    let signals = Arc::new(RefCell::new(vec![]));
    let parsedroot = parseXML(root, Arc::clone(&idlist), Arc::clone(&nodelist), Arc::clone(&selectorlist), Arc::clone(&selectorwants), Arc::clone(&buttonlist), Arc::clone(&buttonwants), Arc::clone(&signals));
    linkSelectors(idlist.clone(), nodelist.clone(), selectorlist, selectorwants);
    linkButtons(idlist, nodelist, buttonlist, buttonwants);
    return ((*parsedroot.as_ref().borrow()).clone(), signals);
}