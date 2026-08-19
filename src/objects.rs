use crate::util;
use util::concatenate;
use util::createNLengthString;
use util::createNLengthStringNL;
use std::sync::Arc;
use std::cell::RefCell;
use crate::eventmanager::Key;
use std::sync::mpsc::Sender;
use core::f64;
use format_num::NumberFormat;

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
        returnstring.push_str(&createNLengthString(w - console::strip_ansi_codes(line).chars().count() as i32, " "));
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

fn joinRowWise(_as: String, ah: i32, aw: i32, _bs: String, bh: i32, bw: i32, gap: String) -> String {
    let mut fullstring = "".to_string();
    if (ah > bh) {
        let newbs = padToHeight(_bs.clone(), bw, ah - bh);
        fullstring = joinLongerShorterRowWise(_as.clone(), newbs, gap);
    } else {
        let newas = padToHeight(_as.clone(), aw, bh - ah);
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
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    pub fn new(colour: String) -> Colour {
        if &colour[0..1] == "#" {
            let r = u8::from_str_radix(&colour[1..3], 16).unwrap();
            let g = u8::from_str_radix(&colour[3..5], 16).unwrap();
            let b = u8::from_str_radix(&colour[5..7], 16).unwrap();
            return Colour {
                r : r,
                g : g,
                b : b
            };
        }
        return Colour {
            r : 0 as u8,
            g : 0 as u8,
            b : 0 as u8
        };
    }
}


fn colourBackgroundInner(text: String, colour: Colour) -> String {
    let mut textsplit: Vec<&str> = text.split("\n\r").collect();
    let mut result = "".to_owned();
    for line in 0..textsplit.len() {
        result += &((format!("\x1b[48;2;{};{};{}m", colour.r, colour.g, colour.b).to_owned() + textsplit[line]).to_owned() + &format!("\x1b[48;2;{};{};{}m", colour.r, colour.g, colour.b));

        if line < textsplit.len() - 1 {
            result += "\n\r";
        }
    }
    return result;
}

fn colourBackgroundOuter(text: String, colour: Colour) -> String {
    let mut textsplit: Vec<&str> = text.split("\n\r").collect();
    let mut result = "".to_owned();
    for line in 0..textsplit.len() {
        result += &(format!("\x1b[48;2;{};{};{}m", colour.r, colour.g, colour.b).to_owned() + &((textsplit[line]).to_owned() + "\x1b[0m"));
        if line < textsplit.len() - 1 {
            result += "\n\r";
        }
    }
    return result;
}

#[derive(Clone, Debug)]
pub struct Effect {
    background: Option<Colour>
}

impl Effect {
    pub fn new(colour: Option<Colour>) -> Effect {
        Effect {
            background: colour
        }
    }

    pub fn applyEffectInner(&self, string: String) -> String {
        let mut returnstring = string.clone();
        if self.background.is_some() {
            returnstring = colourBackgroundInner(string, self.background.clone().unwrap());
        }
        return returnstring;
    }

    pub fn applyEffectOuter(&self, string: String) -> String {
        let mut returnstring = string.clone();
        if self.background.is_some() {
            returnstring = colourBackgroundOuter(string, self.background.clone().unwrap());
        }
        return returnstring;
    }
}

#[derive(Clone, Debug)]
pub struct Text {
    text: String,
    length: i32,
    height: i32,
    effect: Option<Effect>,
    name: String
}

#[derive(Clone, Debug)]
pub struct TextChange {
    text: String,
    length: i32,
    height: i32,
}

#[macro_export]
macro_rules! Text {
    ($text:expr, $length:expr, $height:expr, $effect:expr) => {
        objects::objecttypes::TEXT(crate::objects::Text::new(Some($text), Some($length), Some($height), $effect))
    };
}

impl Text {
    pub fn new(text: Option<String>, length: Option<i32>, height: Option<i32>, effect: Option<Effect>) -> Text {
        return Text {
            text: text.unwrap_or("".to_string()),
            length: length.unwrap_or(0),
            height: height.unwrap_or(0),
            effect: effect,
            name: "".to_owned(),
        };
    }
    pub fn toString(&self) -> String {
        let mut tempholder = Text::new(None, None, None, None);
        tempholder.changeText(self.wrapText());

        let returnstring = padToHeight(
            padToWidth(tempholder.clone().text, self.length),
            self.length,
            self.height - tempholder.getHeight(),
        );
        if self.effect.is_none() {
            return returnstring;
        } else {
            return self.effect.clone().unwrap().applyEffectOuter(self.effect.clone().unwrap().applyEffectInner(returnstring));
        }
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

    pub fn newKeyboardInput(&mut self, input: Key) {}
    pub fn Reset(&mut self) {
        ;
    }
    pub fn getFormData(&mut self) -> Option<String> {
        return None;
    }

    pub fn getObjectByName(&mut self, name: &str) -> Vec<Arc<RefCell<objecttypes>>> {
        vec![]
    }
}

#[derive(Clone, Debug)]
pub struct Box {
    item: Arc<RefCell<objecttypes>>,
    hasborder: bool,
    paddingleft: i32,
    paddingright: i32,
    paddingup: i32,
    paddingdown: i32,
    effect: Option<Effect>,
    name: String
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
    ($item:expr, $hasborder:expr, $paddingleft:expr, $paddingright:expr, $paddingup:expr, $paddingdown:expr, $effect:expr) => {
        objects::objecttypes::BOX(objects::Box::new(
            $item,
            Some($hasborder),
            Some($paddingleft),
            Some($paddingright),
            Some($paddingup),
            Some($paddingdown),
            $effect
        ))
    };
}

impl Box {
    pub fn new(
        item: Arc<RefCell<objecttypes>>,
        hasborder: Option<bool>,
        paddingleft: Option<i32>,
        paddingright: Option<i32>,
        paddingup: Option<i32>,
        paddingdown: Option<i32>,
        effect: Option<Effect>
    ) -> Box {
        return Box {
            item: item,
            hasborder: hasborder.unwrap_or(false),
            paddingleft: paddingleft.unwrap_or(0),
            paddingright: paddingright.unwrap_or(0),
            paddingup: paddingup.unwrap_or(0),
            paddingdown: paddingdown.unwrap_or(0),
            effect: effect,
            name: "".to_owned(),
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
            let mut inbetweenitem = item.to_owned();
            if self.effect.is_some() {
                inbetweenitem = self.effect.clone().unwrap().applyEffectInner(inbetweenitem.clone());
            }
            returnstring += &(leftpad.clone() + &(inbetweenitem) + &rightpad + "\n\r");
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
        if self.effect.is_none() {
            return returnstring;
        } else {
            return self.effect.clone().unwrap().applyEffectOuter(returnstring);
        }
    }

    pub fn changeItem(&mut self, item: Arc<RefCell<objecttypes>>) {
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

    pub fn newKeyboardInput(&mut self, input: Key) {
        self.item.borrow_mut().newKeyboardInput(input);
    }

    pub fn Reset(&mut self) {
        self.item.borrow_mut().Reset();
    }

    pub fn getFormData(&mut self) -> Option<String> {
        return self.item.as_ref().borrow_mut().getFormData();
    }

    pub fn getObjectByName(&mut self, name: &str) -> Vec<Arc<RefCell<objecttypes>>> {
        let mut objects = vec![];
        if &self.item.as_ref().borrow_mut().getName() == name {
            objects.push(Arc::clone(&self.item));
        }
        objects.extend(self.item.as_ref().borrow_mut().getObjectByName(name));
        return objects;
    }
}

#[derive(Clone, Debug)]
pub struct Row {
    items: Vec<Arc<RefCell<objecttypes>>>,
    gap: i32,
    effect: Option<Effect>,
    name: String
}

#[derive(Clone, Debug)]
pub struct RowChange {
    gap: i32,
}




#[macro_export]
macro_rules! Row {
    // items as a list, plus gap
    ($($item:expr),+ $(,)? ; $gap:expr, $effect:expr) => {
        objects::objecttypes::ROW(
            objects::Row::new(
                Some(vec![ $($item),+ ]),
                Some($gap),
                $effect
            )
        )
    };
}

impl Row {
    pub fn new(items: Option<Vec<Arc<RefCell<objecttypes>>>>, gap: Option<i32>, effect: Option<Effect>) -> Row {
        return Row {
            items: items.unwrap_or(vec![]),
            gap: gap.unwrap_or(0),
            effect: effect,
            name: "".to_owned(),
        };
    }
    pub fn toString(&self) -> String {
        let mut returnstring = "".to_string();
        if self.items.len() == 0 {
            return returnstring;
        }
        let mut maxlen: i32 = (self.items.get(0).unwrap()).borrow().getHeight();
        let mut maxwidth: i32 = (self.items.get(0).unwrap()).borrow().getLength();
        let gap = createNLengthString(self.gap, " ");
        returnstring = (self.items.get(0).unwrap()).borrow().toString();
        for item in 1..self.items.len() {
            let mut inbetweenitem = self.items[item].clone().as_ref().borrow().toString();
            if inbetweenitem == "\x00".to_owned() {
                continue;
            }
            if self.effect.is_some() {
                inbetweenitem = self.effect.clone().unwrap().applyEffectInner(inbetweenitem.clone());
            }
            returnstring = joinRowWise(
                returnstring,
                maxlen,
                maxwidth,
                inbetweenitem.clone(),
                (self.items.get(item).unwrap()).borrow().getHeight(),
                (self.items.get(item).unwrap()).borrow().getLength(),
                gap.clone(),
            );
            if maxlen < (self.items.get(item).unwrap()).borrow().getHeight() {
                maxlen = (self.items.get(item).unwrap()).borrow().getHeight();
            }
            if maxwidth < (self.items.get(item).unwrap()).borrow().getLength() {
                maxwidth = (self.items.get(item).unwrap()).borrow().getLength();
            }
        }
        if self.effect.is_none() {
            return returnstring;
        } else {
            return self.effect.clone().unwrap().applyEffectOuter(returnstring);
        }
    }

    pub fn setGap(&mut self, gap: i32) {
        self.gap = gap;
    }

    pub fn addItem(&mut self, item: Arc<RefCell<objecttypes>>) {
        self.items.push(item);
    }

    pub fn getHeight(&self) -> i32 {
        let maxheight = self.items.iter().map(|x| x.borrow().getHeight()).max().unwrap();
        return maxheight;
    }

    pub fn getLength(&self) -> i32 {
        let mut width = self.items.iter().map(|x| x.borrow_mut().getLength()).sum();
        width += self.gap * (self.items.len() as i32 - 1);
        return width;
    }

    pub fn newKeyboardInput(&mut self, input: Key) {
        for item in self.items.clone() {
            item.borrow_mut().newKeyboardInput(input.clone());
        }
    }

    pub fn Reset(&mut self) {
        for item in self.items.clone() {
            item.borrow_mut().Reset();
        }
    }

    pub fn getFormData(&mut self) -> Option<String> {
        let mut returnstring = "".to_owned();
        for i in 0..self.items.len() {
            let result = self.items[i].as_ref().borrow_mut().getFormData();
            if !result.is_none() {
                if returnstring != "".to_owned() {
                    returnstring += "&";
                }
                returnstring += &(result.unwrap());
            }
        }
        if returnstring == "".to_owned() {
            return None;
        }
        return Some(returnstring);
    }

    pub fn getObjectByName(&mut self, name: &str) -> Vec<Arc<RefCell<objecttypes>>> {
        let mut objects = vec![];
        for i in 0..self.items.len() {
            if &self.items[i].as_ref().borrow_mut().getName() == name {
                objects.push(Arc::clone(&self.items[i]));
            }
            objects.extend(self.items[i].as_ref().borrow_mut().getObjectByName(name));
        }
        return objects;
    }
}

#[derive(Clone, Debug)]
pub struct Column {
    items: Vec<Arc<RefCell<objecttypes>>>,
    gap: i32,
    effect: Option<Effect>,
    name: String
}

#[derive(Clone, Debug)]
pub struct ColumnChange {
    gap: i32,
}

#[macro_export]
macro_rules! Column {
    // items as a list, plus gap
    ($($item:expr),+ $(,)? ; $gap:expr, $effect:expr) => {
        objects::objecttypes::COLUMN(
            objects::Column::new(
                Some(vec![ $( $item ),+ ]),
                Some($gap),
                $effect
            )
        )
    };
}


impl Column {
    pub fn new(items: Option<Vec<Arc<RefCell<objecttypes>>>>, gap: Option<i32>, effect: Option<Effect>) -> Column {
        return Column {
            items: items.unwrap_or(vec![]),
            gap: gap.unwrap_or(0),
            effect: effect,
            name: "".to_owned(),
        };
    }

    pub fn setGap(&mut self, gap: i32) {
        self.gap = gap;
    }

    pub fn addItem(&mut self, item: Arc<RefCell<objecttypes>>) {
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
            let mut inbetweenitem = item.toString();
            if inbetweenitem == "\x00".to_owned() {
                continue;
            }
            if self.effect.is_some() {
                inbetweenitem = self.effect.clone().unwrap().applyEffectInner(inbetweenitem.clone());
            }
            returnstring = concatenate(returnstring, padToWidth(inbetweenitem, maxwidth));
            if index != self.items.len() - 1 {
                returnstring += "\n\r";
                for i in 0..self.gap {
                    returnstring += &(createNLengthString(maxwidth, " ") + "\n\r");
                }
            }
        }
        if self.effect.is_none() {
            return returnstring;
        } else {
            return self.effect.clone().unwrap().applyEffectOuter(returnstring);
        }
    }

    pub fn getHeight(&self) -> i32 {
        let mut height = self.items.iter().map(|x| x.borrow().getHeight()).sum();
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

    pub fn newKeyboardInput(&mut self, input: Key) {
        for item in self.items.clone() {
            item.borrow_mut().newKeyboardInput(input.clone());
        }
    }

    pub fn Reset(&mut self) {
        for item in self.items.clone() {
            item.borrow_mut().Reset();
        }
    }

    pub fn getFormData(&mut self) -> Option<String> {
        let mut returnstring = "".to_owned();
        for i in 0..self.items.len() {
            let result = self.items[i].as_ref().borrow_mut().getFormData();
            if !result.is_none() {
                if returnstring != "".to_owned() {
                    returnstring += "&";
                }
                returnstring += &(result.unwrap());
            }
        }
        if returnstring == "".to_owned() {
            return None;
        }
        return Some(returnstring);
    }

    pub fn getObjectByName(&mut self, name: &str) -> Vec<Arc<RefCell<objecttypes>>> {
        let mut objects = vec![];
        for i in 0..self.items.len() {
            if &self.items[i].as_ref().borrow_mut().getName() == name {
                objects.push(Arc::clone(&self.items[i]));
            }
            objects.extend(self.items[i].as_ref().borrow_mut().getObjectByName(name));
        }
        return objects;
    }
}
#[derive(Clone, Debug)]
pub struct Input {
    length: i32,
    height: i32,
    text: String,
    placeholder: String,
    effect: Option<Effect>,
    name: String,
}

#[derive(Clone, Debug)]
pub struct InputChange {
    length: i32,
    height: i32,
    placeholder: String,
}

#[macro_export]
macro_rules! Input {
    ($length:expr, $height:expr, $placeholder:expr, $effect:expr, $name:expr) => {
        objects::objecttypes::INPUT(objects::Input::new(Some($length), Some($height), Some($placeholder), $effect, $name))
    };
}

impl Input {
    pub fn new(length: Option<i32>, height: Option<i32>, placeholder: Option<String>, effect: Option<Effect>, name: String) -> Input {
        return Input {
            length: length.unwrap_or(0),
            height: height.unwrap_or(0),
            text: "".to_string(),
            placeholder: placeholder.clone().unwrap_or("".to_string()),
            effect: effect,
            name: name
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
        let mut inbetweenitem = tempholder.clone();
        let (adjustedheight, _) = self.getAdjustedSize(tempholder);
        let returnstring = padToHeight(
            padToWidth(inbetweenitem, self.length),
            self.length,
            self.height - adjustedheight,
        );
        if self.effect.is_none() {
            return returnstring;
        } else {
            return self.effect.clone().unwrap().applyEffectOuter(self.effect.clone().unwrap().applyEffectOuter(returnstring));
        }
    }

    fn getAdjustedSize(&self, adjstr: String) -> (i32, i32) {
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
        return (height, maxlen);
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

    pub fn newKeyboardInput(&mut self, input: Key) {
        match input {
            Key::BASICKEY(c) => {self.text += &c;},
            Key::DELETEKEY(c) => {self.text.pop();},
            _ => return,
        }
    }
    pub fn Reset(&mut self) {
        ;
    }
    pub fn getFormData(&mut self) -> Option<String> {
        return Some(((self.name.clone() + ":") + &self.text));
    }

    pub fn getObjectByName(&mut self, name: &str) -> Vec<Arc<RefCell<objecttypes>>> {
        vec![]
    }
}

#[derive(Clone, Debug)]
pub struct Selector {
    item: Arc<RefCell<objecttypes>>,
    right: Option<Arc<RefCell<objecttypes>>>,
    left: Option<Arc<RefCell<objecttypes>>>,
    up: Option<Arc<RefCell<objecttypes>>>,
    down: Option<Arc<RefCell<objecttypes>>>,
    isactive: bool,
    wasjustset: bool,
    effect: Option<Effect>,
    activeeffect: Option<Effect>,
    name: String
}

impl Selector {
    pub fn new(item: Option<Arc<RefCell<objecttypes>>>, right: Option<Arc<RefCell<objecttypes>>>, left: Option<Arc<RefCell<objecttypes>>>, up: Option<Arc<RefCell<objecttypes>>>, down: Option<Arc<RefCell<objecttypes>>>, isactive: Option<bool>, effect: Option<Effect>, activeeffect: Option<Effect>) -> Selector {
        return Selector {
            item: item.unwrap(),
            right: right,
            left: left,
            up: up,
            down: down,
            isactive: isactive.unwrap_or(false),
            wasjustset: false,
            effect: effect,
            activeeffect: activeeffect,
            name: "".to_owned()
        };
    }

    pub fn setElements(&mut self, right: Option<Arc<RefCell<objecttypes>>>, left: Option<Arc<RefCell<objecttypes>>>, up: Option<Arc<RefCell<objecttypes>>>, down: Option<Arc<RefCell<objecttypes>>>) {
        self.right = right;
        self.left = left; 
        self.up = up;
        self.down = down; 
    }
    
    pub fn toString(&self) -> String {
        let returnstring = self.item.as_ref().borrow().toString();
        if (self.effect.is_none() && !self.isactive) || (self.activeeffect.is_none() && self.isactive)  {
            return returnstring;
        } else if self.isactive {
            return self.activeeffect.clone().unwrap().applyEffectOuter(self.activeeffect.clone().unwrap().applyEffectInner(returnstring));
        } else {
            return self.effect.clone().unwrap().applyEffectOuter(self.effect.clone().unwrap().applyEffectInner(returnstring));
        }
    }

    pub fn getHeight(&self) -> i32 {
        self.item.as_ref().borrow().getHeight()
    }

    pub fn getLength(&self) -> i32 {
        self.item.as_ref().borrow().getLength()
    }

    pub fn newKeyboardInput(&mut self, input: Key) {
        if self.isactive && !self.wasjustset {
            match input.clone() {
                Key::MOVEMENTKEY(c) => {
                    if c == "left" {
                        self.Left();
                    } else if c == "right" {
                        self.Right();
                    } else if c == "up" {
                        self.Up();
                    } else if c == "down" {
                        self.Down();
                    }
                },
                _ => {},
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

    fn Right(&mut self) {
        if self.right.as_ref().is_some() {
            self.isactive = false;
            self.right.as_ref().unwrap().as_ref().borrow_mut().convertToSelector().activate();
        }
    }

    fn Left(&mut self) {
        if self.left.as_ref().is_some() {
            self.isactive = false;
            self.left.as_ref().unwrap().as_ref().borrow_mut().convertToSelector().activate();
        }
    }

    fn Up(&mut self) {
        if self.up.as_ref().is_some() {
            self.isactive = false;
            self.up.as_ref().unwrap().as_ref().borrow_mut().convertToSelector().activate();
        }
    }

    fn Down(&mut self) {
        if self.down.as_ref().is_some() {
            self.isactive = false;
            self.down.as_ref().unwrap().as_ref().borrow_mut().convertToSelector().activate();
        }
    }

    pub fn Reset(&mut self) {
        self.wasjustset = false;
        self.item.as_ref().borrow_mut().Reset()
    }

    pub fn getFormData(&mut self) -> Option<String> {
        return self.item.as_ref().borrow_mut().getFormData();
    }

    pub fn getObjectByName(&mut self, name: &str) -> Vec<Arc<RefCell<objecttypes>>> {
        let mut objects = vec![];
        if &self.item.as_ref().borrow_mut().getName() == name {
            objects.push(Arc::clone(&self.item));
        }
        objects.extend(self.item.as_ref().borrow_mut().getObjectByName(name));
        return objects;
    }
}

#[macro_export]
macro_rules! Form {
    ($item:expr, $signal:expr, $name:expr) => {
        objects::objecttypes::FORM(objects::Form::new(
            $item,
            $signal,
            $name
        ))
    };
}

#[derive(Clone, Debug)]
pub struct Form {
    item: Arc<RefCell<objecttypes>>,
    signal: Sender<(String, String)>,
    name: String,
}

impl Form {
    pub fn new(
        item: Option<Arc<RefCell<objecttypes>>>,
        signal: Sender<(String, String)>,
        name: String,
    ) -> Form {
        return Form {
            item: item.unwrap(),
            signal: signal,
            name: name,
        };
    }
    pub fn toString(&self) -> String {
        return self.item.as_ref().borrow().toString()
    }

    pub fn getHeight(&self) -> i32 {
        self.item.borrow_mut().getHeight()
    }
    pub fn getLength(&self) -> i32 {
        self.item.borrow_mut().getLength()
    }

    pub fn getName(&self) -> String {
        self.name.clone()
    }

    pub fn newKeyboardInput(&mut self, input: Key) {
        match input {
            Key::INTERACTION(_) => {self.signal.send((self.name.clone(), self.item.as_ref().borrow_mut().getFormData().unwrap_or("".to_owned())));},
            _ => {self.item.borrow_mut().newKeyboardInput(input);},
        }
    }

    pub fn Reset(&mut self) {
        self.item.borrow_mut().Reset();
    }

    pub fn getFormData(&mut self) -> Option<String> {
        return self.item.as_ref().borrow_mut().getFormData();
    }

    pub fn getObjectByName(&mut self, name: &str) -> Vec<Arc<RefCell<objecttypes>>> {
        let mut objects = vec![];
        if &self.item.as_ref().borrow_mut().getName() == name {
            objects.push(Arc::clone(&self.item));
        }
        objects.extend(self.item.as_ref().borrow_mut().getObjectByName(name));
        return objects;
    }
}

#[derive(Clone, Debug)]
pub struct Button {
    text: String,
    length: i32,
    height: i32,
    item: Option<Arc<RefCell<objecttypes>>>,
    effect: Option<Effect>,
    name: String
}

#[derive(Clone, Debug)]
pub struct ButtonChange {
    text: String,
    length: i32,
    height: i32,
}

#[macro_export]
macro_rules! Button {
    ($text:expr, $length:expr, $height:expr, $item:expr, $effect:expr) => {
        objects::objecttypes::BUTTON(crate::objects::Button::new(Some($text), Some($length), Some($height), $item, Some($effect)))
    };
}

impl Button {
    pub fn new(text: Option<String>, length: Option<i32>, height: Option<i32>, item: Option<Arc<RefCell<objecttypes>>>, effect: Option<Effect>) -> Button {
        return Button {
            text: text.unwrap_or("".to_string()),
            length: length.unwrap_or(0),
            height: height.unwrap_or(0),
            item: item,
            effect: effect,
            name: "".to_owned(),
        };
    }
    pub fn toString(&self) -> String {
        let mut tempholder = Text::new(None, None, None, None);
        tempholder.changeText(self.wrapText());

        let returnstring = padToHeight(
            padToWidth(tempholder.clone().text, self.length),
            self.length,
            self.height - tempholder.getHeight(),
        );
        if self.effect.is_none() {
            return returnstring;
        } else {
            return self.effect.clone().unwrap().applyEffectOuter(self.effect.clone().unwrap().applyEffectInner(returnstring));
        }
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

    pub fn newKeyboardInput(&mut self, input: Key) {
        match input {
            Key::ENTERKEY(_) => {self.item.clone().unwrap().borrow_mut().newKeyboardInput(Key::INTERACTION("button".to_owned()))},
            _ => {;},
        }
        
    }
    pub fn Reset(&mut self) {
        ;
    }
    pub fn getFormData(&mut self) -> Option<String> {
        return None;
    }
    pub fn setElement(&mut self, item: Option<Arc<RefCell<objecttypes>>>) {
        self.item = item;
    }

    pub fn getObjectByName(&mut self, name: &str) -> Vec<Arc<RefCell<objecttypes>>> {
        vec![]
    }
}

#[macro_export]
macro_rules! Hidden {
    ($item:expr) => {
        objects::objecttypes::HIDDEN(objects::Hidden::new(
            $item
        ))
    };
}

#[derive(Clone, Debug)]
pub struct Hidden {
    item: Arc<RefCell<objecttypes>>,
    name: String
}

impl Hidden {
    pub fn new(
        item: Option<Arc<RefCell<objecttypes>>>
    ) -> Hidden {
        return Hidden {
            item: item.unwrap(),
            name: "".to_owned(),
        };
    }
    pub fn toString(&self) -> String {
        return "\x00".to_owned();
    }

    pub fn getHeight(&self) -> i32 {
        0
    }
    pub fn getLength(&self) -> i32 {
        0
    }

    pub fn newKeyboardInput(&mut self, input: Key) {
        self.item.as_ref().borrow_mut().newKeyboardInput(input);
    }
    pub fn Reset(&mut self) {
        self.item.as_ref().borrow_mut().Reset();
    }
    pub fn getFormData(&mut self) -> Option<String> {
        return self.item.as_ref().borrow_mut().getFormData();
    }

    pub fn getObjectByName(&mut self, name: &str) -> Vec<Arc<RefCell<objecttypes>>> {
        let mut objects = vec![];
        if &self.item.as_ref().borrow_mut().getName() == name {
            objects.push(Arc::clone(&self.item));
        }
        objects.extend(self.item.as_ref().borrow_mut().getObjectByName(name));
        return objects;
    }
}

#[macro_export]
macro_rules! Progress {
    ($min:expr, $max:expr, $height:expr, $length:expr, $preset:expr, $showpercent:expr, $startval:expr) => {
        objects::objecttypes::Progress(objects::Hidden::new(
            $min,
            $max,
            $height,
            $length,
            $preset,
            $showpercent,
            Some($startval)
        ))
    };
}

#[derive(Clone, Debug)]
pub struct Progress {
    min: f32,
    max: f32,
    height: i32,
    length: i32,
    preset: i8,
    showpercent: i8,
    value: f32,
    name: String
}

impl Progress {
    pub fn new(
        min: f32, max: f32, height: i32, length: i32, preset: i8, showpercent: i8, startval: Option<f32>
    ) -> Progress {
        return Progress {
            min: min,
            max: max,
            height: height,
            length: length,
            preset: preset,
            showpercent: showpercent, 
            value: startval.unwrap_or(min),
            name: "".to_owned(),
        };
    }

    pub fn toString(&self) -> String {
        let presetfull = ["█▉▊▋▌▍▎▏ ", "█▓▒░ ", "█▇▆▅▄▃▂▁ ", "⣿⣷⣶⣦⣤⣄⣀⡀ "];
        let filling = presetfull[self.preset as usize];
        let barsize = (self.max - self.min) as f32 / (self.length as f32);
        let mut tmpvalue = self.value;
        if self.value > self.max {
            tmpvalue = self.max;
        }
        let mut bar = "".to_owned();
        while tmpvalue > 0 as f32 {
            if tmpvalue > barsize {
                bar.push(filling.chars().nth(0 as usize).unwrap());
                tmpvalue -= barsize;
            } else {
                let len = (filling.chars().count() - 1);
                bar.push(filling.chars().nth((len as f32 - (tmpvalue / barsize * (len as f32)).floor() - 1.0) as usize).unwrap());
                tmpvalue = 0.0;
            }
        }
        bar = padToWidth(bar, self.length);
        bar = createNLengthStringNL(self.height, &bar);
        if self.showpercent > 0 {
            let num = NumberFormat::new();
            bar += &num.format(&(".".to_owned() + &(self.showpercent.to_string().to_owned() + "s")), (self.value - self.min) / (self.max - self.min) * 100.0);
            bar += "%";
        }
        return bar;
    }

    pub fn getHeight(&self) -> i32 {
        self.height
    }

    pub fn getLength(&self) -> i32 {
        if self.showpercent > 0 {
            let num = NumberFormat::new();
            let mut tmp = num.format(&(".".to_owned() + &(self.showpercent.to_string().to_owned() + "s")), (self.value - self.min) / (self.max - self.min) * 100.0);
            tmp += "%";
            return self.length + tmp.len() as i32;
        }
        self.length
    }

    pub fn getValue(&self) -> f32 {
        self.value
    }

    pub fn setValue(&mut self, value: f32) {
        self.value = value;
    }

    pub fn newKeyboardInput(&mut self, input: Key) {
        ;
    }
    pub fn Reset(&mut self) {
        ;
    }
    pub fn getFormData(&mut self) -> Option<String> {
        None
    }

    pub fn getObjectByName(&mut self, name: &str) -> Vec<Arc<RefCell<objecttypes>>> {
        vec![]
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
    FORM(Form),
    BUTTON(Button),
    HIDDEN(Hidden),
    PROGRESS(Progress),
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
            objecttypes::FORM(c) => c.toString(),
            objecttypes::BUTTON(c) => c.toString(),
            objecttypes::HIDDEN(c) => c.toString(),
            objecttypes::PROGRESS(c) => c.toString(),
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
            objecttypes::FORM(c) => c.getHeight(),
            objecttypes::BUTTON(c) => c.getHeight(),
            objecttypes::HIDDEN(c) => c.getHeight(),
            objecttypes::PROGRESS(c) => c.getHeight(),
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
            objecttypes::FORM(c) => c.getLength(),
            objecttypes::BUTTON(c) => c.getLength(),
            objecttypes::HIDDEN(c) => c.getLength(),
            objecttypes::PROGRESS(c) => c.getLength(),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn newKeyboardInput(&mut self, input: Key) {
        match self {
            objecttypes::TEXT(c) => c.newKeyboardInput(input),
            objecttypes::BOX(c) => c.newKeyboardInput(input),
            objecttypes::ROW(c) => c.newKeyboardInput(input),
            objecttypes::COLUMN(c) => c.newKeyboardInput(input),
            objecttypes::INPUT(c) => c.newKeyboardInput(input),
            objecttypes::SELECTOR(c) => c.newKeyboardInput(input),
            objecttypes::FORM(c) => c.newKeyboardInput(input),
            objecttypes::BUTTON(c) => c.newKeyboardInput(input),
            objecttypes::HIDDEN(c) => c.newKeyboardInput(input),
            objecttypes::PROGRESS(c) => c.newKeyboardInput(input),
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

    pub fn convertToButton(&mut self) -> &mut Button {
        match self {
            objecttypes::BUTTON(c) => c,
            _ => panic!("method on object not supported"),
        }
    }

    pub fn convertToProgress(&mut self) -> &mut Progress {
        match self {
            objecttypes::PROGRESS(c) => c,
            _ => panic!("method on object not supported"),
        }
    }

    pub fn getFormData(&mut self) -> Option<String> {
        match self {
            objecttypes::TEXT(c) => None,
            objecttypes::BOX(c) => c.getFormData(),
            objecttypes::ROW(c) => c.getFormData(),
            objecttypes::COLUMN(c) => c.getFormData(),
            objecttypes::INPUT(c) => c.getFormData(),
            objecttypes::SELECTOR(c) => c.getFormData(),
            objecttypes::FORM(c) => c.getFormData(),
            objecttypes::BUTTON(c) => c.getFormData(),
            objecttypes::HIDDEN(c) => c.getFormData(),
            objecttypes::PROGRESS(c) => c.getFormData(),
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
            objecttypes::FORM(c) => c.Reset(),
            objecttypes::BUTTON(c) => c.Reset(),
            objecttypes::HIDDEN(c) => c.Reset(),
            objecttypes::PROGRESS(c) => c.Reset(),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn getObjectByName(&mut self, name: &str) -> Vec<Arc<RefCell<objecttypes>>>  {
        match self {
            objecttypes::TEXT(c) => c.getObjectByName(name),
            objecttypes::BOX(c) => c.getObjectByName(name),
            objecttypes::ROW(c) => c.getObjectByName(name),
            objecttypes::COLUMN(c) => c.getObjectByName(name),
            objecttypes::INPUT(c) => c.getObjectByName(name),
            objecttypes::SELECTOR(c) => c.getObjectByName(name),
            objecttypes::FORM(c) => c.getObjectByName(name),
            objecttypes::BUTTON(c) => c.getObjectByName(name),
            objecttypes::HIDDEN(c) => c.getObjectByName(name),
            objecttypes::PROGRESS(c) => c.getObjectByName(name),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn setName(&mut self, name: &str) {
        match self {
            objecttypes::TEXT(c) => {c.name = name.to_owned();},
            objecttypes::BOX(c) => {c.name = name.to_owned();},
            objecttypes::ROW(c) => {c.name = name.to_owned();},
            objecttypes::COLUMN(c) => {c.name = name.to_owned();},
            objecttypes::INPUT(c) => {c.name = name.to_owned();},
            objecttypes::SELECTOR(c) => {c.name = name.to_owned();},
            objecttypes::FORM(c) => {c.name = name.to_owned();},
            objecttypes::BUTTON(c) => {c.name = name.to_owned();},
            objecttypes::HIDDEN(c) => {c.name = name.to_owned();},
            objecttypes::PROGRESS(c) => {c.name = name.to_owned();},
            _ => panic!("method on object not supported"),
        }
    }

    pub fn getName(&mut self) -> String {
        match self {
            objecttypes::TEXT(c) => c.name.clone(),
            objecttypes::BOX(c) => c.name.clone(),
            objecttypes::ROW(c) => c.name.clone(),
            objecttypes::COLUMN(c) => c.name.clone(),
            objecttypes::INPUT(c) => c.name.clone(),
            objecttypes::SELECTOR(c) => c.name.clone(),
            objecttypes::FORM(c) => c.name.clone(),
            objecttypes::BUTTON(c) => c.name.clone(),
            objecttypes::HIDDEN(c) => c.name.clone(),
            objecttypes::PROGRESS(c) => c.name.clone(),
            _ => panic!("method on object not supported"),
        }
    }
}
