/*
Convert a number to a string, the contents of which depend on the number's factors.

If the number has 3 as a factor, output 'Pling'.
If the number has 5 as a factor, output 'Plang'.
If the number has 7 as a factor, output 'Plong'.
If the number does not have 3, 5, or 7 as a factor, just pass the number's digits straight through.
Examples
28's factors are 1, 2, 4, 7, 14, 28.
In raindrop-speak, this would be a simple "Plong".
30's factors are 1, 2, 3, 5, 6, 10, 15, 30.
In raindrop-speak, this would be a "PlingPlang".
34 has four factors: 1, 2, 17, and 34.
In raindrop-speak, this would be "34".
*/
pub fn raindrops(n: u32) -> String {
    let mut answer = String::from("");

    let is_factor = |f| n % f == 0;

    if is_factor(3) {
        answer.push_str("Pling");
    }
    if is_factor(5) {
        answer.push_str("Plang");
    }
    if is_factor(7) {
        answer.push_str("Plong");
    }
    if answer.is_empty() {
        answer.push_str(format!("{}", n).as_str());
    }

    answer
}

fn _raindrops(n: u32) -> String {
    let mut answer = String::from("");

    let factors = (1..=n).filter(|f| n % f == 0).collect::<Vec<u32>>();

    if factors.contains(&3) {
        answer.push_str("Pling");
    }
    if factors.contains(&5) {
        answer.push_str("Plang");
    }
    if factors.contains(&7) {
        answer.push_str("Plong");
    }
    if answer.is_empty() {
        answer.push_str(format!("{}", n).as_str());
    }

    answer
}
