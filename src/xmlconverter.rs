use crate::objects;
use crate::Box;
use crate::Text;
use crate::Input;
use crate::objects::objecttypes;
use crate::objects::Colour;
use crate::objects::Effect;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;
use std::vec;

fn linkSelectors(idlist: Rc<RefCell<Vec<i32>>>, nodelist: Rc<RefCell<Vec<Rc<RefCell<objecttypes>>>>>, selectorlist: Rc<RefCell<Vec<Rc<RefCell<objecttypes>>>>>, selectorwants: Rc<RefCell<Vec<Vec<i32>>>>) {
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
                currelem = Some(Rc::clone(&nodelist.as_ref().borrow()[currindex]));
            } else {
                currelem = None;
            }
            elemvec.push(currelem);
        }
        selector.setElements(elemvec[0].clone(), elemvec[1].clone(), elemvec[2].clone(), elemvec[3].clone());
    }
}

fn parseXML(doc: roxmltree::Node, idlist: Rc<RefCell<Vec<i32>>>, nodelist: Rc<RefCell<Vec<Rc<RefCell<objecttypes>>>>>, selectorlist: Rc<RefCell<Vec<Rc<RefCell<objecttypes>>>>>, selectorwants: Rc<RefCell<Vec<Vec<i32>>>>) -> Rc<RefCell<objecttypes>> {
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
                let object = Box!(
                    parseXML(node.first_element_child().unwrap(), Rc::clone(&idlist), Rc::clone(&nodelist), Rc::clone(&selectorlist), Rc::clone(&selectorwants)),
                    node.attribute("hasborder").unwrap_or("false") == "true",
                    node.attribute("paddingleft").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingright").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingup").unwrap_or("0").parse::<i32>().unwrap(),
                    node.attribute("paddingdown").unwrap_or("0").parse::<i32>().unwrap(),
                    effect
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
                    node.attribute("height").unwrap_or("0").parse::<i32>().unwrap(),
                    effect
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
                        Some(node.attribute("gap").unwrap_or("0").parse::<i32>().unwrap()),
                        effect
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
                        Some(node.attribute("gap").unwrap_or("0").parse::<i32>().unwrap()),
                        effect
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
                    node.attribute("placeholder").unwrap_or("").to_owned(),
                    effect
                );
                let thisobject = Rc::new(RefCell::new(object));
                if node.attribute("id").unwrap_or("-1") != "-1" {
                    idlist.as_ref().borrow_mut().push(node.attribute("id").unwrap().parse::<i32>().unwrap());
                    nodelist.as_ref().borrow_mut().push(Rc::clone(&thisobject));
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
                let object = objects::objecttypes::SELECTOR(
                    objects::Selector::new(
                        Some(parseXML(node.first_element_child().unwrap(), Rc::clone(&idlist), Rc::clone(&nodelist), Rc::clone(&selectorlist), Rc::clone(&selectorwants))),
                        None,
                        None,
                        None,
                        None,
                        Some(node.attribute("isactive").unwrap_or("false").to_owned() == "true"),
                        effect,
                        activeeffect
                    )
                );
                let thisobject = Rc::new(RefCell::new(object));
                selectorlist.borrow_mut().push(Rc::clone(&thisobject));
                selectorwants.borrow_mut().push(vec![
                    node.attribute("right").unwrap_or("-1").parse::<i32>().unwrap(),
                    node.attribute("left").unwrap_or("-1").parse::<i32>().unwrap(),
                    node.attribute("up").unwrap_or("-1").parse::<i32>().unwrap(),
                    node.attribute("down").unwrap_or("-1").parse::<i32>().unwrap()
                    ]);
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