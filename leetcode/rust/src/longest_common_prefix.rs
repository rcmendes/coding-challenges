pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }

        let mut prefix = &*strs[0];
        for word in &strs[1..] {
            while !word.starts_with(prefix) {
                let length = prefix.len() - 1;
                prefix = &prefix[..length];
                if prefix.is_empty() {
                    return "".to_string();
                }
            }
        }

        prefix.to_string()
    }

    #[allow(dead_code)]
    pub fn longest_common_prefix2(strings: Vec<&str>) -> &str {
        use std::cmp;

        if strings.is_empty() {
            return "";
        }
        let str0 = strings[0];
        let str0bytes = str0.as_bytes();
        let mut len = str0.len();
        for str in &strings[1..] {
            len = cmp::min(
                len,
                str.as_bytes()
                    .iter()
                    .zip(str0bytes)
                    .take_while(|&(a, b)| a == b)
                    .count(),
            );
        }
        &strings[0][..len]
    }

    #[allow(dead_code)]
    pub fn longest_common_prefix3(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }

        let first_word = &strs[0];

        if strs.len() == 1 {
            return first_word.to_owned();
        }

        let mut found = true;
        let mut result = "";
        let mut chars_count = 1;
        while found && first_word.len() > 0 {
            let substring = &first_word[..chars_count];

            for i in 1..strs.len() {
                let word = &strs[i];

                if word.len() < chars_count {
                    found = false;
                    break;
                }
                let word_substring = &word[..chars_count];
                if substring != word_substring {
                    found = false;
                    break;
                }
            }
            if !found {
                result = &substring[..(substring.len() - 1)];
                break;
            }

            if chars_count == first_word.len() {
                result = substring;
                break;
            }

            chars_count += 1;
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let input = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let result = Solution::longest_common_prefix(input);

        println!("[{}]", result);

        assert_eq!("fl".to_owned(), result);
    }

    #[test]
    fn test_2() {
        let input2 = vec!["c".to_string(), "c".to_string()];
        let result2 = Solution::longest_common_prefix(input2);

        println!("[{}]", result2);

        assert_eq!("c".to_owned(), result2);
    }

    #[test]
    fn test_3() {
        let input3 = vec!["c".to_owned(), "acc".to_owned(), "ccc".to_owned()];
        let result3 = Solution::longest_common_prefix(input3);

        println!("[{}]", result3);

        assert_eq!("".to_owned(), result3);
    }
}
