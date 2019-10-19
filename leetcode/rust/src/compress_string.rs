use std::collections::HashMap;

pub fn compress(chars: &mut Vec<char>) -> i32 {
    if chars.is_empty() {
        return 0;
    }

    let mut curr_letter = &chars[0];

    let mut result = vec![*curr_letter];

    let mut count = 1;
    for i in 1..chars.len() {
        let letter = &chars[i];
        if curr_letter == letter {
            count += 1;
        }

        if count > 1 && (curr_letter != letter || i + 1 == chars.len()) {
            let count_as_str = count.to_string();
            let mut count_as_chars = count_as_str.chars();
            while let Some(count_char) = count_as_chars.next() {
                result.push(count_char);
            }
        }

        if curr_letter != letter {
            curr_letter = letter;
            count = 1;
            result.push(*curr_letter);
        }
    }

    chars.clear();
    chars.append(&mut result);

    return chars.len() as i32;
}

#[allow(dead_code)]
pub fn compress_partial(chars: &mut Vec<char>) -> i32 {
    let mut chars_count = HashMap::<char, usize>::new();

    for letter in chars.iter() {
        let count = *chars_count.get(&letter).unwrap_or(&mut (0 as usize));
        chars_count.insert(*letter, count + 1);
    }

    chars.clear();
    for (letter, count) in &chars_count {
        chars.push(*letter);
        if *count > 1 {
            let count_as_str = count.to_string();
            let mut count_as_chars = count_as_str.chars();
            while let Some(count_char) = count_as_chars.next() {
                chars.push(count_char);
            }
        }
    }

    return chars.len() as i32;
}

#[allow(dead_code)]
pub fn compress2(chars: &mut Vec<char>) -> i32 {
    let mut chars_count = HashMap::<char, usize>::new();

    for letter in chars.iter() {
        let count = *chars_count.get(&letter).unwrap_or(&mut (0 as usize));
        chars_count.insert(*letter, count + 1);
    }

    let mut result: usize = 0;
    for count in chars_count.values() {
        if chars_count.keys().len() == 1 {
            if *count == 1 as usize {
                result = 1;
            } else {
                result = 1 + count.to_string().chars().count();
            }
            break;
        } else {
            if *count == 1 as usize {
                result += 1;
            } else {
                result += 1 + count.to_string().chars().count();
            }
        }
    }

    return result as i32;
}

#[cfg(test)]
mod tests {
    use super::compress;

    #[test]
    fn test_1() {
        let mut input = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];

        let result = compress(&mut input);

        assert_eq!(6, result);
        assert_eq!(['a', '2', 'b', '2', 'c', '3'], input.as_slice());
    }

    #[test]
    fn test_2() {
        let mut input = vec!['a'];

        let result = compress(&mut input);

        assert_eq!(1, result);
        assert_eq!(['a'], input.as_slice());
    }
}
