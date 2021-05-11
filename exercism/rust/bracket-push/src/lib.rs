pub fn brackets_are_balanced(string: &str) -> bool {
    let mut balanced = true;
    let bytes = string.trim().as_bytes();

    let mut stack = Vec::new();
    for (_, &item) in bytes.iter().enumerate() {
        match item {
            b'{' | b'[' | b'(' => stack.push(item),
            b'}' => {
                let last_element = stack.pop().unwrap_or_default();
                if last_element != b'{' {
                    balanced = false;
                    break;
                }
            }
            b']' => {
                let last_element = stack.pop().unwrap_or_default();
                if last_element != b'[' {
                    balanced = false;
                    break;
                }
            }
            b')' => {
                let last_element = stack.pop().unwrap_or_default();
                if last_element != b'(' {
                    balanced = false;
                    break;
                }
            },
            _ => println!("{}", item)
        }
    }

    balanced && stack.len() == 0
}
