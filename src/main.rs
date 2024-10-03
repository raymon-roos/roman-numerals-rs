use std::env;

const NUMERALS: [(&str, usize); 13] = [
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

fn main() {
    let i = env::args().nth(1).map_or(100, |arg| {
        arg.parse::<usize>()
            .expect("Argument should be an integer!")
    });

    println!("{}", to_roman(i));

    from_roman("MCMXVIII").ok();
}

fn to_roman(p: usize) -> String {
    let numeral: (String, usize) = NUMERALS
        .iter()
        .skip_while(|&(_symbol, value)| *value > p)
        .map(|&(symbol, value)| (symbol.to_string(), value))
        .fold(
            (String::from(""), 0),
            |(mut numeral, mut total), (symbol, val)| {
                let remainder = p - total;
                let multiplier = remainder / val;
                total += multiplier * val;
                numeral.push_str(&symbol.repeat(multiplier));

                (numeral, total)
            },
        );

    numeral.0
}

#[derive(Debug)]
struct ParseNumeralError {}

fn from_roman(input_str: &str) -> Result<usize, ParseNumeralError> {
    let input_symbols = input_str.chars().collect::<Vec<char>>();
    let input_roman_digits = input_symbols.chunk_by(|a, b| {
        matches!(
            (a, b),
            ('C', 'M') | ('C', 'D') | ('X', 'C') | ('X', 'L') | ('I', 'X') | ('I', 'V')
        )
    });

    let mut sum = 0;
    for input_roman_digit in input_roman_digits {
        let matching_roman_digit = NUMERALS.iter().find(|&(symbol, _val)| {
            match (input_roman_digit.first(), input_roman_digit.get(1)) {
                (Some(a), Some(b)) => *symbol == format!("{a}{b}"),
                (None, Some(b)) => *symbol == b.to_string(),
                (Some(a), None) => *symbol == a.to_string(),
                _ => false,
            }
        });

        match matching_roman_digit {
            Some((_symbol, value)) => sum += value,
            None => return Err(ParseNumeralError {}),
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn three_from_roman() {
        assert_eq!("III", to_roman(3));
    }

    #[test]
    fn three_to_roman() {
        assert_eq!(from_roman("III").unwrap(), 3);
    }

    #[test]
    fn two_hundred_seven_from_roman() {
        assert_eq!("CCVII", to_roman(207));
    }

    #[test]
    fn two_hundred_seven_to_roman() {
        assert_eq!(from_roman("CCVII").unwrap(), 207);
    }

    #[test]
    fn four_hundred_ninety_four_from_roman() {
        assert_eq!("CDXCIV", to_roman(494));
    }

    #[test]
    fn four_hundred_ninety_four_to_roman() {
        assert_eq!(from_roman("CDXCIV").unwrap(), 494);
    }

    #[test]
    fn seven_hundred_eighty_nine_from_roman() {
        assert_eq!("DCCLXXXIX", to_roman(789));
    }

    #[test]
    fn seven_hundred_eighty_nine_to_roman() {
        assert_eq!(from_roman("DCCLXXXIX").unwrap(), 789);
    }

    #[test]
    fn eight_hundred_eight_from_roman() {
        assert_eq!("DCCCLXXXVIII", to_roman(888));
    }

    #[test]
    fn eight_hundred_eight_to_roman() {
        assert_eq!(from_roman("DCCCLXXXVIII").unwrap(), 888);
    }

    #[test]
    fn nineteen_hundred_eighteen_to_roman() {
        assert_eq!(from_roman("MCMXVIII").unwrap(), 1918);
    }

    #[test]
    fn nineteen_hundred_eighteen_from_roman() {
        assert_eq!(from_roman("MCMXVIII").unwrap(), 1918);
    }
}
