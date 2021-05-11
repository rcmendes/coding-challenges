static UNITS: &[&'static str] = &["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
static TENS: &[&'static str] = &["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
static HUNDREDS: &[&'static str] = &["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
static THOUSANDS: &[&'static str] = &["", "M", "MM", "MMM", "", "", "", "", "", ""];

/// Converts a number to a string representating roman numeral.
pub fn num_as_roman(num: i32) -> String {
    let mut roman_number = String::from("");

    let mut unit = 1000;

    loop {
        let digit = ((num / unit) % 10) as usize;
        if unit < 9 {
            roman_number.push_str(UNITS[digit]);
            break;
        } else if unit < 99 {
            roman_number.push_str(TENS[digit]);
        } else if unit < 999 {
            roman_number.push_str(HUNDREDS[digit]);
        } else {
            roman_number.push_str(THOUSANDS[digit]);
        }

        unit = unit / 10;
    }

    roman_number
}

fn num_as_roman_from_code_wars(mut num: i32) -> String {
    let mut letters = String::new();
    let symbols = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    for &(n, symbol) in symbols.iter() {
        while num >= n {
            letters.push_str(symbol);
            num -= n;
        }
    }
    letters
}

#[cfg(test)]
mod tests {
    use super::num_as_roman;

    #[test]
    fn returns_expected() {
        assert_eq!(num_as_roman(182), "CLXXXII");
        assert_eq!(num_as_roman(1990), "MCMXC");
        assert_eq!(num_as_roman(1875), "MDCCCLXXV");
    }
}
