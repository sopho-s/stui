pub fn createNLengthString(n: i32, val: &str) -> String {
    let mut string: String = "".to_string();
    for i in 0..n {
        string += val;
    }
    return string;
}

pub fn concatenate(a: String, b: String) -> String {
    return a + &b;
}