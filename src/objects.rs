use std::cmp::max;
use std::ptr;
use std::str;
use crate::util;
use sdl2::libc::printf;
use util::concatenate;
use util::createNLengthString;
use util::createNLengthStringNL;
use std::rc::Rc;
use std::cell::RefCell;

fn padToHeight(a: String, aw: i32, h: i32) -> String {
    if h == 0 {
        return a;
    }
    return a + "\n\r" + &createNLengthStringNL(h, &(createNLengthString(aw, " ")));
}
fn padToWidth(a: String, w: i32) -> String {
    if (w == 0) {
        return a;
    }
    let mut returnstring = "".to_string();
    let mut asplit = a.split("\n\r");
    let mut i = 0;
    for line in asplit.clone() {
        returnstring.push_str(line);
        returnstring.push_str(&createNLengthString(w - line.chars().count() as i32, " "));
        if (i != asplit.clone().count() - 1) {
            returnstring.push_str("\n\r");
        }
        i += 1;
    }
    return returnstring;
}

fn joinLongerShorterRowWise(a: String, b: String, gap: String) -> String {
    let mut fullstring = "".to_string();
    let mut asplit = a.split("\n\r");
    let mut bsplit = b.split("\n\r");
    let mut i = 0;
    for line in bsplit.clone() {
        if i == bsplit.clone().count() as i32 - 1 {
            fullstring = concatenate(
                fullstring,
                concatenate(
                    concatenate(asplit.next().unwrap_or("").to_owned(), gap.clone()),
                    line.to_owned(),
                ),
            );
        } else {
            fullstring = concatenate(
                fullstring,
                concatenate(
                    concatenate(
                        concatenate(asplit.next().unwrap_or("").to_owned(), gap.clone()),
                        line.to_owned(),
                    ),
                    "\n\r".to_string(),
                ),
            );
        }
        i += 1;
    }
    for line in asplit {
        fullstring = concatenate(fullstring, line.to_owned()); //, "\n\r".to_string()));
    }
    return fullstring;
}

fn joinShorterLongerRowWise(a: String, b: String, gap: String) -> String {
    let mut fullstring = "".to_string();
    let mut asplit = a.split("\n\r");
    let mut bsplit = b.split("\n\r");
    let mut i = 0;
    for line in asplit.clone() {
        if i == asplit.clone().count() as i32 - 1 {
            fullstring = concatenate(
                fullstring,
                concatenate(
                    concatenate(line.to_owned(), gap.clone()),
                    bsplit.next().unwrap_or("").to_owned(),
                ),
            );
        } else {
            fullstring = concatenate(
                fullstring,
                concatenate(
                    concatenate(
                        concatenate(line.to_owned(), gap.clone()),
                        bsplit.next().unwrap_or("").to_owned(),
                    ),
                    "\n\r".to_string(),
                ),
            );
        }
        i += 1;
    }
    for line in bsplit {
        fullstring = concatenate(fullstring, line.to_owned()); //, "\n\r".to_string()));
    }
    return fullstring;
}

fn joinRowWise(_as: String, ah: i32, aw: i32, b: objecttypes, gap: String) -> String {
    let bh = b.getHeight();
    let mut fullstring = "".to_string();
    let mut _bs = b.toString();
    if (ah > bh) {
        let newbs = padToHeight(_bs.clone(), b.getLength(), ah - b.getHeight());
        fullstring = joinLongerShorterRowWise(_as.clone(), newbs, gap);
    } else {
        let newas = padToHeight(_as.clone(), aw, b.getHeight() - ah);
        fullstring = joinShorterLongerRowWise(newas, _bs.clone(), gap);
    }
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
    length: i32,
    height: i32,
}

#[derive(Clone, Debug)]
pub struct TextChange {
    text: String,
    length: i32,
    height: i32,
}

#[macro_export]
macro_rules! Text {
    ($text:expr, $length:expr, $height:expr) => {
        objects::objecttypes::TEXT(crate::objects::Text::new(Some($text), Some($length), Some($height)))
    };
}

impl Text {
    pub fn new(text: Option<String>, length: Option<i32>, height: Option<i32>) -> Text {
        return Text {
            text: text.unwrap_or("".to_string()),
            length: length.unwrap_or(0),
            height: height.unwrap_or(0),
        };
    }
    pub fn toString(&self) -> String {
        let mut tempholder = Text::new(None, None, None);
        tempholder.changeText(self.wrapText());
        return padToHeight(
            padToWidth(tempholder.clone().text, self.length),
            self.length,
            self.height - tempholder.getHeight(),
        );
    }
    fn wrapText(&self) -> String {
        let mut _text = self.text.clone();
        if _text.chars().count() as i32 <= self.length {
            return _text;
        }
        let mut returnstring = "".to_string();
        let mut currheight = 0;
        while _text.chars().count() as i32 > self.length {
            let left = _text.split_off(self.length as usize);
            currheight += 1;
            if currheight == self.height {
                returnstring.push_str(&_text);
                return returnstring;
            } else {
                returnstring.push_str(&concatenate(_text, "\n\r".to_string()));
            }
            _text = left;
        }
        returnstring += &_text;
        return returnstring;
    }

    pub fn changeText(&mut self, text: String) {
        let mut resultstring = "".to_string();
        let textsplit = text.split("\n\r");
        let mut maxlen: i32 = 0;
        for line in textsplit.clone() {
            if line.chars().count() as i32 > maxlen {
                maxlen = line.chars().count() as i32;
            }
        }
        let mut i = 0;
        for line in textsplit.clone() {
            resultstring.push_str(&padToWidth(line.to_string(), maxlen));
            if i != textsplit.clone().count() - 1 {
                resultstring.push_str("\n\r");
            }
            self.height += 1;
            i += 1;
        }
        self.text = resultstring;
        self.length = maxlen;
    }
    pub fn getHeight(&self) -> i32 {
        self.height
    }
    pub fn getLength(&self) -> i32 {
        self.length
    }

    pub fn newKeyboardInput(&mut self, input: char) {}
    pub fn Reset(&mut self) {
        ;
    }
}

#[derive(Clone, Debug)]
pub struct Box {
    item: Rc<RefCell<objecttypes>>,
    hasborder: bool,
    paddingleft: i32,
    paddingright: i32,
    paddingup: i32,
    paddingdown: i32,
}

#[derive(Clone, Debug)]
pub struct BoxChange {
    hasborder: bool,
    paddingleft: i32,
    paddingright: i32,
    paddingup: i32,
    paddingdown: i32,
}

#[macro_export]
macro_rules! Box {
    ($item:expr, $hasborder:expr, $paddingleft:expr, $paddingright:expr, $paddingup:expr, $paddingdown:expr) => {
        objects::objecttypes::BOX(objects::Box::new(
            $item,
            Some($hasborder),
            Some($paddingleft),
            Some($paddingright),
            Some($paddingup),
            Some($paddingdown),
        ))
    };
}

impl Box {
    pub fn new(
        item: Rc<RefCell<objecttypes>>,
        hasborder: Option<bool>,
        paddingleft: Option<i32>,
        paddingright: Option<i32>,
        paddingup: Option<i32>,
        paddingdown: Option<i32>,
    ) -> Box {
        return Box {
            item: item,
            hasborder: hasborder.unwrap_or(false),
            paddingleft: paddingleft.unwrap_or(0),
            paddingright: paddingright.unwrap_or(0),
            paddingup: paddingup.unwrap_or(0),
            paddingdown: paddingdown.unwrap_or(0),
        };
    }
    pub fn toString(&self) -> String {
        let mut returnstring: String = "".to_string();
        let mut leftpad = createNLengthString(self.paddingleft, " ");
        let mut rightpad = createNLengthString(self.paddingright, " ");
        if self.hasborder {
            leftpad = concatenate("│".to_owned(), leftpad.clone());
            rightpad = concatenate(rightpad.clone(), "│".to_owned());
        }
        let mut midpad: String;
        midpad = createNLengthString(self.item.borrow_mut().getLength(), " ");
        returnstring += &createNLengthString(
            self.paddingup,
            &(leftpad.clone() + &midpad + &rightpad + "\n\r"),
        );
        let mut itemclone = self.item.borrow_mut().toString().clone();
        itemclone = itemclone.replace("\r", "");
        let itemsplit = itemclone.split("\n");
        for item in itemsplit {
            returnstring += &(leftpad.clone() + &(item) + &rightpad + "\n\r");
        }
        returnstring += &createNLengthString(
            self.paddingdown,
            &(leftpad.clone() + &midpad + &rightpad + "\n\r"),
        );
        if self.hasborder {
            returnstring = concatenate(
                concatenate(
                    concatenate(createBoxLid(self.getLength()), "\n\r".to_owned()),
                    returnstring,
                ),
                createBoxBottom(self.getLength()),
            );
        }
        return returnstring;
    }

    pub fn changeItem(&mut self, item: Rc<RefCell<objecttypes>>) {
        self.item = item;
    }

    pub fn setPadding(&mut self, padding: i32) {
        self.paddingdown = padding;
        self.paddingup = padding;
        self.paddingright = padding;
        self.paddingleft = padding;
    }

    pub fn setPaddingLeft(&mut self, paddingleft: i32) {
        self.paddingleft = paddingleft;
    }

    pub fn setPaddingRight(&mut self, paddingright: i32) {
        self.paddingright = paddingright;
    }

    pub fn setPaddingUp(&mut self, paddingup: i32) {
        self.paddingup = paddingup;
    }

    pub fn setPaddingDown(&mut self, paddingdown: i32) {
        self.paddingdown = paddingdown;
    }

    pub fn setBorder(&mut self, truth: bool) {
        self.hasborder = truth;
    }

    pub fn getHeight(&self) -> i32 {
        if self.hasborder {
            self.item.borrow_mut().getHeight() + self.paddingdown + self.paddingup + 2
        } else {
            self.item.borrow_mut().getHeight() + self.paddingdown + self.paddingup
        }
    }
    pub fn getLength(&self) -> i32 {
        if self.hasborder {
            self.item.borrow_mut().getLength() + self.paddingleft + self.paddingright + 2
        } else {
            self.item.borrow_mut().getLength() + self.paddingleft + self.paddingright
        }
    }

    pub fn newKeyboardInput(&mut self, input: char) {
        self.item.borrow_mut().newKeyboardInput(input);
    }
    pub fn Reset(&mut self) {
        self.item.borrow_mut().Reset();
    }
}

#[derive(Clone, Debug)]
pub struct Row {
    items: Vec<Rc<RefCell<objecttypes>>>,
    gap: i32,
}

#[derive(Clone, Debug)]
pub struct RowChange {
    gap: i32,
}




#[macro_export]
macro_rules! Row {
    // items as a list, plus gap
    ($($item:expr),+ $(,)? ; $gap:expr) => {
        objects::objecttypes::ROW(
            objects::Row::new(
                Some(vec![ $($item),+ ]),
                Some($gap),
            )
        )
    };
}

impl Row {
    pub fn new(items: Option<Vec<Rc<RefCell<objecttypes>>>>, gap: Option<i32>) -> Row {
        return Row {
            items: items.unwrap_or(vec![]),
            gap: gap.unwrap_or(0),
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
        returnstring = (self.items.get(0).unwrap()).borrow_mut().toString();
        maxlen = (self.items.get(0).unwrap()).borrow_mut().getHeight();
        maxwidth = (self.items.get(0).unwrap()).borrow_mut().getLength();
        for item in 1..self.items.len() {
            returnstring = joinRowWise(
                returnstring,
                maxlen,
                maxwidth,
                (self.items.get(item).unwrap()).borrow_mut().clone(),
                gap.clone(),
            );
            if maxlen
                < (self.items.get(item).unwrap()).borrow_mut()
                    .getHeight()
            {
                maxlen = (self.items.get(item).unwrap()).borrow_mut()
                    .getHeight();
            }
            if maxwidth
                < (self.items.get(item).unwrap()).borrow_mut()
                    .getLength()
            {
                maxwidth = (self.items.get(item).unwrap()).borrow_mut()
                    .getLength();
            }
        }
        return returnstring;
    }

    pub fn setGap(&mut self, gap: i32) {
        self.gap = gap;
    }

    pub fn addItem(&mut self, item: Rc<RefCell<objecttypes>>) {
        self.items.push(item);
    }

    pub fn getHeight(&self) -> i32 {
        let mut maxheight = 0;
        for item in self.items.iter() {
            if maxheight < item.borrow_mut().getHeight() {
                maxheight = item.borrow_mut().getHeight()
            }
        }
        return maxheight;
    }

    pub fn getLength(&self) -> i32 {
        let mut width = 0;
        for item in self.items.iter() {
            width += item.borrow_mut().getLength()
        }
        width += self.gap * (self.items.len() as i32 - 1);
        return width;
    }

    pub fn newKeyboardInput(&mut self, input: char) {
        for item in self.items.clone() {
            item.borrow_mut().newKeyboardInput(input);
        }
    }

    pub fn Reset(&mut self) {
        for item in self.items.clone() {
            item.borrow_mut().Reset();
        }
    }
}

#[derive(Clone, Debug)]
pub struct Column {
    items: Vec<Rc<RefCell<objecttypes>>>,
    gap: i32,
}

#[derive(Clone, Debug)]
pub struct ColumnChange {
    gap: i32,
}

#[macro_export]
macro_rules! Column {
    // items as a list, plus gap
    ($($item:expr),+ $(,)? ; $gap:expr) => {
        objects::objecttypes::COLUMN(
            objects::Column::new(
                Some(vec![ $( $item ),+ ]),
                Some($gap),
            )
        )
    };
}


impl Column {
    pub fn new(items: Option<Vec<Rc<RefCell<objecttypes>>>>, gap: Option<i32>) -> Column {
        return Column {
            items: items.unwrap_or(vec![]),
            gap: gap.unwrap_or(0),
        };
    }

    pub fn setGap(&mut self, gap: i32) {
        self.gap = gap;
    }

    pub fn addItem(&mut self, item: Rc<RefCell<objecttypes>>) {
        self.items.push(item);
    }

    pub fn toString(&self) -> String {
        let mut returnstring = "".to_string();
        if self.items.len() == 0 {
            return returnstring;
        }
        let maxwidth = self.getLength();
        for index in 0..self.items.len() {
            let item = self.items.get(index).unwrap().borrow();
            returnstring = concatenate(returnstring, padToWidth(item.toString(), maxwidth));
            if index != self.items.len() - 1 {
                returnstring += "\n\r";
                for i in 0..self.gap {
                    returnstring += &(createNLengthString(maxwidth, " ") + "\n\r");
                }
            }
        }
        return returnstring;
    }

    pub fn getHeight(&self) -> i32 {
        let mut height = 0;
        for item in self.items.iter() {
            height += item.borrow().getHeight()
        }
        height += self.gap * self.items.len() as i32;
        return height;
    }

    pub fn getLength(&self) -> i32 {
        let mut maxwidth = 0;
        for item in self.items.iter() {
            if maxwidth < item.borrow().getLength() {
                maxwidth = item.borrow().getLength()
            }
        }
        return maxwidth;
    }

    pub fn newKeyboardInput(&mut self, input: char) {
        for item in self.items.clone() {
            item.borrow_mut().newKeyboardInput(input);
        }
    }
    pub fn Reset(&mut self) {
        for item in self.items.clone() {
            item.borrow_mut().Reset();
        }
    }
}
#[derive(Clone, Debug)]
pub struct Input {
    length: i32,
    height: i32,
    text: String,
    placeholder: String,
}

#[derive(Clone, Debug)]
pub struct InputChange {
    length: i32,
    height: i32,
    placeholder: String,
}

#[macro_export]
macro_rules! Input {
    ($length:expr, $height:expr, $placeholder:expr) => {
        objects::objecttypes::INPUT(objects::Input::new(Some($length), Some($height), Some($placeholder)))
    };
}

impl Input {
    pub fn new(length: Option<i32>, height: Option<i32>, placeholder: Option<String>) -> Input {
        return Input {
            length: length.unwrap_or(0),
            height: height.unwrap_or(0),
            text: "".to_string(),
            placeholder: placeholder.clone().unwrap_or("".to_string()),
        };
    }

    pub fn setLength(&mut self, length: i32) {
        self.length = length;
    }

    pub fn setHeight(&mut self, height: i32) {
        self.height = height;
    }

    pub fn toString(&self) -> String {
        let mut tempholder = self.wrapText();
        return padToHeight(
            padToWidth(tempholder.clone(), self.length),
            self.length,
            self.height - self.getAdjustedHeight(tempholder),
        );
    }

    fn getAdjustedLength(&self, adjstr: String) -> i32 {
        let mut resultstring = "".to_string();
        let textsplit = adjstr.split("\n\r");
        let mut maxlen: i32 = 0;
        for line in textsplit.clone() {
            if line.chars().count() as i32 > maxlen {
                maxlen = line.chars().count() as i32;
            }
        }
        let mut i = 0;
        for line in textsplit.clone() {
            resultstring.push_str(&padToWidth(line.to_string(), maxlen));
            if i != textsplit.clone().count() - 1 {
                resultstring.push_str("\n\r");
            }
            i += 1;
        }
        return maxlen;
    }

    fn getAdjustedHeight(&self, adjstr: String) -> i32 {
        let mut resultstring = "".to_string();
        let textsplit = adjstr.split("\n\r");
        let mut maxlen: i32 = 0;
        for line in textsplit.clone() {
            if line.chars().count() as i32 > maxlen {
                maxlen = line.chars().count() as i32;
            }
        }
        let mut i = 0;
        let mut height = 0;
        for line in textsplit.clone() {
            resultstring.push_str(&padToWidth(line.to_string(), maxlen));
            if i != textsplit.clone().count() - 1 {
                resultstring.push_str("\n\r");
            }
            height += 1;
            i += 1;
        }
        return height;
    }

    pub fn getLength(&self) -> i32 {
        self.length
    }

    pub fn getHeight(&self) -> i32 {
        self.height
    }

    fn wrapText(&self) -> String {
        let mut text = "".to_string();
        if self.text.chars().count() == 0 {
            text = self.placeholder.clone();
        } else {
            text = self.text.clone();
        }
        if text.chars().count() as i32 <= self.length {
            return text;
        }
        let mut returnstring = "".to_string();
        let mut currheight = 0;
        while text.chars().count() as i32 > self.length {
            let left = text.split_off(self.length as usize);
            currheight += 1;
            if currheight == self.height {
                returnstring.push_str(&text);
                return returnstring;
            } else {
                returnstring.push_str(&concatenate(text, "\n\r".to_string()));
            }
            text = left;
        }
        returnstring += &text;
        return returnstring;
    }

    pub fn newKeyboardInput(&mut self, input: char) {
        if input == '\x08' {
            self.text.pop();
        } else {
            self.text.push(input);
        }
    }
    pub fn Reset(&mut self) {
        ;
    }
}

#[derive(Clone, Debug)]
pub struct Selector {
    item: Rc<RefCell<objecttypes>>,
    next: Option<Rc<RefCell<objecttypes>>>,
    previous: Option<Rc<RefCell<objecttypes>>>,
    isactive: bool,
    wasjustset: bool,
}

impl Selector {
    pub fn new(item: Option<Rc<RefCell<objecttypes>>>, next: Option<Rc<RefCell<objecttypes>>>, previous: Option<Rc<RefCell<objecttypes>>>, isactive: Option<bool>) -> Selector {
        return Selector {
            item: item.unwrap(),
            next: next,
            previous: previous,
            isactive: isactive.unwrap_or(false),
            wasjustset: false
        };
    }
    pub fn setElements(&mut self, next: Rc<RefCell<objecttypes>>, previous: Rc<RefCell<objecttypes>>) {
        self.next = Some(Rc::clone(&next));
        self.previous = Some(Rc::clone(&previous)); 
    }
    pub fn toString(&self) -> String {
        self.item.as_ref().borrow().toString()
    }

    pub fn getHeight(&self) -> i32 {
        self.item.as_ref().borrow().getHeight()
    }

    pub fn getLength(&self) -> i32 {
        self.item.as_ref().borrow().getLength()
    }
    pub fn newKeyboardInput(&mut self, input: char) {
        if self.isactive && !self.wasjustset {
            if input == '\x00' {
                self.nex();
                return;
            }
            if input == '\x01' {
                self.prev();
                return;
            }
            self.item.as_ref().borrow_mut().newKeyboardInput(input);
        } else if self.wasjustset {
            self.wasjustset = false;
        }
    }
    pub fn activate(&mut self) {
        self.wasjustset = true;
        self.isactive = true;
    }
    pub fn nex(&mut self) {
        self.isactive = false;
        self.next.as_ref().unwrap().as_ref().borrow_mut().convertToSelector().activate();
    }
    pub fn prev(&mut self) {
        self.isactive = false;
        self.previous.as_ref().unwrap().as_ref().borrow_mut().convertToSelector().activate();
    }
    pub fn Reset(&mut self) {
        self.wasjustset = false;
        self.item.as_ref().borrow_mut().Reset()
    }
}

#[derive(Clone, Debug)]
pub enum objecttypes {
    TEXT(Text),
    BOX(Box),
    ROW(Row),
    COLUMN(Column),
    INPUT(Input),
    SELECTOR(Selector),
}

impl objecttypes {
    pub fn toString(&self) -> String {
        match self {
            objecttypes::TEXT(c) => c.toString(),
            objecttypes::BOX(c) => c.toString(),
            objecttypes::ROW(c) => c.toString(),
            objecttypes::COLUMN(c) => c.toString(),
            objecttypes::INPUT(c) => c.toString(),
            objecttypes::SELECTOR(c) => c.toString(),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn getHeight(&self) -> i32 {
        match self {
            objecttypes::TEXT(c) => c.getHeight(),
            objecttypes::BOX(c) => c.getHeight(),
            objecttypes::ROW(c) => c.getHeight(),
            objecttypes::COLUMN(c) => c.getHeight(),
            objecttypes::INPUT(c) => c.getHeight(),
            objecttypes::SELECTOR(c) => c.getHeight(),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn getLength(&self) -> i32 {
        match self {
            objecttypes::TEXT(c) => c.getLength(),
            objecttypes::BOX(c) => c.getLength(),
            objecttypes::ROW(c) => c.getLength(),
            objecttypes::COLUMN(c) => c.getLength(),
            objecttypes::INPUT(c) => c.getLength(),
            objecttypes::SELECTOR(c) => c.getLength(),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn newKeyboardInput(&mut self, input: char) {
        match self {
            objecttypes::TEXT(c) => c.newKeyboardInput(input),
            objecttypes::BOX(c) => c.newKeyboardInput(input),
            objecttypes::ROW(c) => c.newKeyboardInput(input),
            objecttypes::COLUMN(c) => c.newKeyboardInput(input),
            objecttypes::INPUT(c) => c.newKeyboardInput(input),
            objecttypes::SELECTOR(c) => c.newKeyboardInput(input),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn getResetString(&mut self) -> String {
        let height = self.getHeight();
        let width = self.getLength();
        let mut string = createNLengthString(width,"\x1b[1A\x1b[2K");
        return string;
    }

    pub fn convertToSelector(&mut self) -> &mut Selector {
        match self {
            objecttypes::SELECTOR(c) => c,
            _ => panic!("method on object not supported"),
        }
    }
    pub fn Reset(&mut self) {
        match self {
            objecttypes::TEXT(c) => c.Reset(),
            objecttypes::BOX(c) => c.Reset(),
            objecttypes::ROW(c) => c.Reset(),
            objecttypes::COLUMN(c) => c.Reset(),
            objecttypes::INPUT(c) => c.Reset(),
            objecttypes::SELECTOR(c) => c.Reset(),
            _ => panic!("method on object not supported"),
        }
    }
}
