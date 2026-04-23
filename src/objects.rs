use std::cmp::max;
use std::ptr;
use std::str;
mod util;
use util::concatenate;
use util::createNLengthString;
use util::createNLengthStringNL;

fn padToHeight(a: String, aw: i32, h: i32) -> String {
    if h == 0 {
        return a;
    }
    return a + "\n" + &createNLengthStringNL(h, &(createNLengthString(aw, " ")));
}
fn padToWidth(a: String, w: i32) -> String {
    if (w == 0) {
        return a;
    }
    let mut returnstring = a.clone();
    let mut asplit = a.split("\n");
    let mut i = 0;
    for line in asplit.clone() {
        returnstring.push_str(&createNLengthString(w, " "));
        if (i != asplit.clone().count() - 1) {
            returnstring.push_str("\n");
        }
        i += 1;
    }
    return returnstring;
}

fn joinLongerShorterRowWise(a: String, b: String, gap: String) -> String {
    let mut fullstring = "".to_string();
    let mut asplit = a.split("\n");
    let mut bsplit = b.split("\n");
    let mut i = 0;
    for line in bsplit.clone() {
        if i == bsplit.clone().count() as i32 - 1 {

        fullstring = concatenate(
            fullstring,
                concatenate(
                    concatenate(asplit.next().unwrap_or("").to_owned(), gap.clone()),
                    line.to_owned(),
                )
        );
        } else {
        fullstring = concatenate(
            fullstring,
            concatenate(
                concatenate(
                    concatenate(asplit.next().unwrap_or("").to_owned(), gap.clone()),
                    line.to_owned(),
                ),
                "\n".to_string(),
            ),
        );
    }
    i += 1;
    }
    for line in asplit {
        fullstring = concatenate(fullstring, line.to_owned());//, "\n".to_string()));
    }
    return fullstring;
}

fn joinShorterLongerRowWise(a: String, b: String, gap: String) -> String {
    let mut fullstring = "".to_string();
    let mut asplit = a.split("\n");
    let mut bsplit = b.split("\n");
    let mut i = 0;
    for line in asplit.clone() {
        if i == asplit.clone().count() as i32 - 1 {
        fullstring = concatenate(
            fullstring,
                concatenate(
                    concatenate(line.to_owned(), gap.clone()),
                    bsplit.next().unwrap_or("").to_owned(),
                )
        );
        } else {
        fullstring = concatenate(
            fullstring,
            concatenate(
                concatenate(
                    concatenate(line.to_owned(), gap.clone()),
                    bsplit.next().unwrap_or("").to_owned(),
                ),
                "\n".to_string(),
            ),
        );
    }
    i += 1;
    }
    for line in bsplit {
        fullstring = concatenate(fullstring, line.to_owned());//, "\n".to_string()));
    }
    return fullstring;
}

fn joinRowWise(_as: String, ah: i32, aw: i32, b: objecttypes, gap: String) -> String {
    let bh = b.getHeight();
    let mut fullstring = "".to_string();
    let mut _bs = b.toString();
    if (ah > bh) {
        let newbs = padToHeight(_bs.clone(), b.getLength(), ah - b.getHeight());
        println!("newas: \"{}\"", newbs);
        fullstring = joinLongerShorterRowWise(_as.clone(), newbs, gap);
    } else {
        let newas = padToHeight(_as.clone(), aw, b.getHeight() - ah);
        println!("newbs: \"{}\"", newas);
        fullstring = joinShorterLongerRowWise(newas, _bs.clone(), gap);
    }
    println!("this is as: '{}'\nthis is bs: '{}'\nthis is fullstring: '{}'", _as, _bs, fullstring);
    return fullstring;
}

fn createBoxLid(width: i32) -> String {
    return "╭".to_owned() + &createNLengthString(width - 2, "─") + "╮";
}
fn createBoxBottom(width: i32) -> String {
    return "╰".to_owned() + &createNLengthString(width - 2, "─") + "╯";
}

#[derive(Clone, Debug)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new() -> Text {
        return Text {
            text: "".to_string(),
        };
    }
    pub fn toString(&self) -> String {
        return self.text.clone();
    }
    pub fn changeText(&mut self, text: String) {
        self.text = text;
    }
    pub fn getHeight(&self) -> i32 {
        1
    }
    pub fn getLength(&self) -> i32 {
        self.text.len() as i32
    }
}

#[derive(Clone, Debug)]
pub struct Box {
    item: *const objecttypes,
    hasborder: bool,
    padding_left: i32,
    padding_right: i32,
    padding_up: i32,
    padding_down: i32,
}

impl Box {
    pub fn new() -> Box {
        return Box {
            item: ptr::null(),
            hasborder: false,
            padding_left: 0,
            padding_right: 0,
            padding_up: 0,
            padding_down: 0,
        };
    }
    pub fn toString(&self) -> String {
        let mut returnstring: String = "".to_string();
        let mut leftpad = createNLengthString(self.padding_left, " ");
        let mut rightpad = createNLengthString(self.padding_right, " ");
        if (self.hasborder) {
            leftpad = concatenate("│".to_owned(), leftpad.clone());
            rightpad = concatenate(rightpad.clone(), "│".to_owned());
        }
        let mut midpad: String;
        unsafe {
            midpad = createNLengthString((self.item).as_ref().unwrap().getLength(), " ");
        }
        returnstring += &createNLengthString(
            self.padding_up,
            &(leftpad.clone() + &midpad + &rightpad + "\n"),
        );
        unsafe {
            let itemclone = (self.item).as_ref().unwrap().toString().clone();
            let itemsplit = itemclone.split("\n");
            for item in itemsplit {
                returnstring += &(leftpad.clone()
                    + &(item)
                    + &rightpad
                    + "\n");
            }
        }
        returnstring += &createNLengthString(
            self.padding_down,
            &(leftpad.clone() + &midpad + &rightpad + "\n"),
        );
        if self.hasborder {
            returnstring = concatenate(
                    concatenate(
                        concatenate(createBoxLid(self.getLength() ), "\n".to_owned()),
                        returnstring,
                    ),
                createBoxBottom(self.getLength()),
            );
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
    pub fn setBorder(&mut self, truth: bool) {
        self.hasborder = truth;
    }
    pub fn getHeight(&self) -> i32 {
        unsafe {
            if self.hasborder {
                (self.item).as_ref().unwrap().getHeight() + self.padding_down + self.padding_up + 2
            }
            else {
                (self.item).as_ref().unwrap().getHeight() + self.padding_down + self.padding_up
            }
        }
    }
    pub fn getLength(&self) -> i32 {
        unsafe {
            if self.hasborder {
                (self.item).as_ref().unwrap().getLength() + self.padding_left + self.padding_right + 2
            }
            else {
                (self.item).as_ref().unwrap().getLength() + self.padding_left + self.padding_right 
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Row {
    items: Vec<*const objecttypes>,
    gap: i32,
}

impl Row {
    pub fn new() -> Row {
        return Row {
            items: vec![],
            gap: 0,
        };
    }
    pub fn toString(&self) -> String {
        let mut returnstring = "".to_string();
        if self.items.len() == 0 {
            return returnstring;
        }
        let mut maxlen: i32 = 0;
        let mut maxwidth: i32 = 0;
        let gap = createNLengthString(self.gap, " ");
        unsafe {
            returnstring = (self.items.get(0).unwrap()).as_ref().unwrap().toString();
            maxlen = (self.items.get(0).unwrap()).as_ref().unwrap().getHeight();
            maxwidth = (self.items.get(0).unwrap()).as_ref().unwrap().getLength();
            for item in 1..self.items.len() {
                returnstring = joinRowWise(
                    returnstring,
                    maxlen,
                    maxwidth,
                    (self.items.get(item).unwrap()).as_ref().unwrap().clone(),
                    gap.clone(),
                );
                if maxlen
                    < (self.items.get(item).unwrap())
                        .as_ref()
                        .unwrap()
                        .getHeight()
                {
                    maxlen = (self.items.get(item).unwrap())
                        .as_ref()
                        .unwrap()
                        .getHeight();
                }
                if maxwidth
                    < (self.items.get(item).unwrap())
                        .as_ref()
                        .unwrap()
                        .getLength()
                {
                    maxwidth = (self.items.get(item).unwrap())
                        .as_ref()
                        .unwrap()
                        .getLength();
                }
            }
        }
        return returnstring;
    }

    pub fn setGap(&mut self, gap: i32) {
        self.gap = gap;
    }

    pub fn addItem(&mut self, item: *const objecttypes) {
        self.items.push(item);
    }

    pub fn getHeight(&self) -> i32 {
        let mut maxheight = 0;
        unsafe {
            for item in self.items.iter() {
                if maxheight < item.as_ref().unwrap().getHeight() {
                    maxheight = item.as_ref().unwrap().getHeight()
                }
            }
        }
        return maxheight;
    }

    pub fn getLength(&self) -> i32 {
        let mut width = 0;
        unsafe {
            for item in self.items.iter() {
                width += item.as_ref().unwrap().getLength()
            }
        }
        width += self.gap * (self.items.len() as i32 - 1);
        return width;
    }
}

#[derive(Clone, Debug)]
pub struct Column {
    items: Vec<*const objecttypes>,
    gap: i32,
}

impl Column {
    pub fn new() -> Column {
        return Column {
            items: vec![],
            gap: 0,
        };
    }

    pub fn setGap(&mut self, gap: i32) {
        self.gap = gap;
    }

    pub fn addItem(&mut self, item: *const objecttypes) {
        self.items.push(item);
    }

    pub fn toString(&self) -> String {
        let mut returnstring = "".to_string();
        if self.items.len() == 0 {
            return returnstring;
        }
        let mut maxwidth = self.getLength();
        for index in 0..self.items.len() {
            
            unsafe {
                let item = self.items.get(index).unwrap().as_ref().unwrap();
                returnstring = concatenate(returnstring, padToWidth(item.toString(), maxwidth - item.getLength()));
            }
            if index != self.items.len() - 1 {
                returnstring += "\n";
            }
        }
        return returnstring;
    }

    pub fn getHeight(&self) -> i32 {
        let mut height = 0;
        unsafe {
            for item in self.items.iter() {
                height += item.as_ref().unwrap().getHeight()
            }
        }
        height += self.gap * self.items.len() as i32;
        return height;
    }

    pub fn getLength(&self) -> i32 {
        let mut maxwidth = 0;
        unsafe {
            for item in self.items.iter() {
                if maxwidth < item.as_ref().unwrap().getLength() {
                    maxwidth = item.as_ref().unwrap().getLength()
                }
            }
        }
        return maxwidth;
    }
}

#[derive(Clone, Debug)]
pub enum objecttypes {
    TEXT(Text),
    BOX(Box),
    ROW(Row),
    COLUMN(Column),
}

impl objecttypes {
    pub fn toString(&self) -> String {
        match self {
            objecttypes::TEXT(c) => c.toString(),
            objecttypes::BOX(c) => c.toString(),
            objecttypes::ROW(c) => c.toString(),
            objecttypes::COLUMN(c) => c.toString(),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn getHeight(&self) -> i32 {
        match self {
            objecttypes::TEXT(c) => c.getHeight(),
            objecttypes::BOX(c) => c.getHeight(),
            objecttypes::ROW(c) => c.getHeight(),
            objecttypes::COLUMN(c) => c.getHeight(),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn getLength(&self) -> i32 {
        match self {
            objecttypes::TEXT(c) => c.getLength(),
            objecttypes::BOX(c) => c.getLength(),
            objecttypes::ROW(c) => c.getLength(),
            objecttypes::COLUMN(c) => c.getLength(),
            _ => panic!("method on object not supported"),
        }
    }
}
