pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut cursor: usize = 0;
    while cursor + len <= digits.len() {
        result.push(String::from(&digits[cursor..(cursor + len)]));
        cursor = cursor + 1;
    }

    result
}
