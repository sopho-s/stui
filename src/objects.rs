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
    let mut returnstring = "".to_string();
    let mut asplit = a.split("\n");
    let mut i = 0;
    for line in asplit.clone() {
        returnstring.push_str(line);
        returnstring.push_str(&createNLengthString(w - line.len() as i32, " "));
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

impl Text {
    pub fn new() -> Text {
        return Text {
            text: "".to_string(),
            length: 0,
            height: 0,
        };
    }
    pub fn toString(&self) -> String {
        return self.text.clone();
    }
    pub fn changeText(&mut self, text: String) {
        let mut resultstring = "".to_string();
        let textsplit = text.split("\n");
        let mut maxlen: i32 = 0;
        for line in textsplit.clone() {
            if line.len() as i32 > maxlen {
                maxlen = line.len() as i32;
            }
        }
        let mut i = 0;
        for line in textsplit.clone() {
            resultstring.push_str(&padToWidth(line.to_string(), maxlen));
            if i != textsplit.clone().count() - 1 {
                resultstring.push_str("\n");
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

    pub fn newKeyboardInput(&mut self, input: char) {
        ;
    }
}

#[derive(Clone, Debug)]
pub struct Box {
    item: *mut objecttypes,
    hasborder: bool,
    paddingleft: i32,
    paddingright: i32,
    paddingup: i32,
    paddingdown: i32,
}

impl Box {
    pub fn new() -> Box {
        return Box {
            item: ptr::null_mut(),
            hasborder: false,
            paddingleft: 0,
            paddingright: 0,
            paddingup: 0,
            paddingdown: 0,
        };
    }
    pub fn toString(&self) -> String {
        let mut returnstring: String = "".to_string();
        let mut leftpad = createNLengthString(self.paddingleft, " ");
        let mut rightpad = createNLengthString(self.paddingright, " ");
        if (self.hasborder) {
            leftpad = concatenate("│".to_owned(), leftpad.clone());
            rightpad = concatenate(rightpad.clone(), "│".to_owned());
        }
        let mut midpad: String;
        unsafe {
            midpad = createNLengthString((self.item).as_ref().unwrap().getLength(), " ");
        }
        returnstring += &createNLengthString(
            self.paddingup,
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
            self.paddingdown,
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

    pub fn changeItem(&mut self, item: *mut objecttypes) {
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
        unsafe {
            if self.hasborder {
                (self.item).as_ref().unwrap().getHeight() + self.paddingdown + self.paddingup + 2
            }
            else {
                (self.item).as_ref().unwrap().getHeight() + self.paddingdown + self.paddingup
            }
        }
    }
    pub fn getLength(&self) -> i32 {
        unsafe {
            if self.hasborder {
                (self.item).as_ref().unwrap().getLength() + self.paddingleft + self.paddingright + 2
            }
            else {
                (self.item).as_ref().unwrap().getLength() + self.paddingleft + self.paddingright 
            }
        }
    }

    pub fn newKeyboardInput(&mut self, input: char) {
        unsafe {
            self.item.as_mut().unwrap().newKeyboardInput(input);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Row {
    items: Vec<*mut objecttypes>,
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

    pub fn addItem(&mut self, item: *mut objecttypes) {
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

    pub fn newKeyboardInput(&mut self, input: char) {
        unsafe {
            for item in self.items.clone() {
                item.as_mut().unwrap().newKeyboardInput(input);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Column {
    items: Vec<*mut objecttypes>,
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

    pub fn addItem(&mut self, item: *mut objecttypes) {
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
                returnstring = concatenate(returnstring, padToWidth(item.toString(), maxwidth));
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

    pub fn newKeyboardInput(&mut self, input: char) {
        unsafe {
            for item in self.items.clone() {
                item.as_mut().unwrap().newKeyboardInput(input);
            }
        }
    }
}


#[derive(Clone, Debug)]
pub struct Background {
    item: *mut objecttypes,
    colour: String,
}

impl Background {
    pub fn new() -> Background {
        return Background {
            item: ptr::null_mut(),
            colour: "\x1b[101m".to_string(),
        };
    }

    pub fn setItem(&mut self, item: *mut objecttypes) {
        self.item = item;
    }

    pub fn toString(&self) -> String {
        unsafe {
            concatenate(concatenate(self.colour.clone(), self.item.as_ref().unwrap().toString()), "\x1b[0m".to_owned())
        }
    }

    pub fn getLength(&self) -> i32 {
        unsafe {
            self.item.as_ref().unwrap().getLength()
        }
    }

    pub fn getHeight(&self) -> i32 {
        unsafe {
            self.item.as_ref().unwrap().getHeight()
        }
    }

    pub fn newKeyboardInput(&mut self, input: char) {
        unsafe {
            self.item.as_mut().unwrap().newKeyboardInput(input);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    length: i32,
    height: i32,
    text: String,
}

impl Input {
    pub fn new() -> Input {
        return Input {
            length: 0,
            height: 0,
            text: "".to_string(),
        };
    }

    pub fn setLength(&mut self, length: i32) {
        self.length = length;
    }

    pub fn setHeight(&mut self, height: i32) {
        self.height = height;
    }

    pub fn toString(&self) -> String {
        let mut tempholder = Text::new();
        tempholder.changeText(self.wrapText());
        return padToHeight(padToWidth(tempholder.toString(), self.length), self.length, self.height - tempholder.getHeight());
    }

    pub fn getLength(&self) -> i32 {
        unsafe {
            self.length
        }
    }

    pub fn getHeight(&self) -> i32 {
        unsafe {
            self.height
        }
    }

    fn wrapText(&self) -> String {
        if self.text.len() as i32 <= self.length {
            return self.text.clone()
        }
        let mut returnstring = "".to_string();
        let mut tmpstring = self.text.clone();
        let mut currheight = 0;
        while tmpstring.len() as i32 > self.length {
            let left = tmpstring.split_off(self.length as usize);
            currheight += 1;
            if currheight == self.height {
                returnstring.push_str(&tmpstring);
                return returnstring;
            } else {
                returnstring.push_str(&concatenate(tmpstring, "\n".to_string()));
            }
            tmpstring = left;
        }
        returnstring += &tmpstring;
        return returnstring
    }

    pub fn newKeyboardInput(&mut self, input: char) {
        self.text.push(input);
    }
}

#[derive(Clone, Debug)]
pub enum objecttypes {
    TEXT(Text),
    BOX(Box),
    ROW(Row),
    COLUMN(Column),
    BACKGROUND(Background),
    INPUT(Input),
}

impl objecttypes {
    pub fn toString(&self) -> String {
        match self {
            objecttypes::TEXT(c) => c.toString(),
            objecttypes::BOX(c) => c.toString(),
            objecttypes::ROW(c) => c.toString(),
            objecttypes::COLUMN(c) => c.toString(),
            objecttypes::BACKGROUND(c) => c.toString(),
            objecttypes::INPUT(c) => c.toString(),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn getHeight(&self) -> i32 {
        match self {
            objecttypes::TEXT(c) => c.getHeight(),
            objecttypes::BOX(c) => c.getHeight(),
            objecttypes::ROW(c) => c.getHeight(),
            objecttypes::COLUMN(c) => c.getHeight(),
            objecttypes::BACKGROUND(c) => c.getHeight(),
            objecttypes::INPUT(c) => c.getHeight(),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn getLength(&self) -> i32 {
        match self {
            objecttypes::TEXT(c) => c.getLength(),
            objecttypes::BOX(c) => c.getLength(),
            objecttypes::ROW(c) => c.getLength(),
            objecttypes::COLUMN(c) => c.getLength(),
            objecttypes::BACKGROUND(c) => c.getLength(),
            objecttypes::INPUT(c) => c.getLength(),
            _ => panic!("method on object not supported"),
        }
    }

    pub fn newKeyboardInput(&mut self, input: char) {
        match self {
            objecttypes::TEXT(c) => c.newKeyboardInput(input),
            objecttypes::BOX(c) => c.newKeyboardInput(input),
            objecttypes::ROW(c) => c.newKeyboardInput(input),
            objecttypes::COLUMN(c) => c.newKeyboardInput(input),
            objecttypes::BACKGROUND(c) => c.newKeyboardInput(input),
            objecttypes::INPUT(c) => c.newKeyboardInput(input),
            _ => panic!("method on object not supported"),
        }
    }
}
