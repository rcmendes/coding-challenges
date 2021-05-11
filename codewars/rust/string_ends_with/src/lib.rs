fn solution(word: &str, ending: &str) -> bool {
    let word_length = word.len();
    let ending_length = ending.len();

    if word_length < ending_length {
        return false;
    }

    &word[(word_length - ending_length)..word_length] == ending
}

#[cfg(test)]
mod tests {
    use super::solution;
    #[test]
    fn returns_expected() {
        assert_eq!(true, solution("abc", "c"));
        assert_eq!(false, solution("strawberry", "banana"));
    }
}
