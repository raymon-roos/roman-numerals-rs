use std::{env, usize};

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
                let multiplier = (remainder - (remainder % val)) / val;
                total += multiplier * val;
                numeral.push_str(&symbol.repeat(multiplier));

                (numeral, total)
            },
        );

    numeral.0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn three() {
        assert_eq!("III", to_roman(3));
    }

    #[test]
    fn two_hundred_seven() {
        assert_eq!("CCVII", to_roman(207));
    }

    #[test]
    fn seven_hundred_eighty_nine() {
        assert_eq!("DCCLXXXIX", to_roman(789));
    }

    #[test]
    fn eight_hundred_eight() {
        assert_eq!("DCCCLXXXVIII", to_roman(888));
    }

    #[test]
    fn nineteen_hundred_eighteen() {
        assert_eq!("MCMXVIII", to_roman(1918));
    }
}
