use std::{error::Error, io::stdin, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let res: u32 = stdin()
        .lines()
        .map(|l| l.unwrap())
        // kinda inefficient in memory, but it has nice semantics
        .map(|s| s.parse::<Digits>().expect("infallible for the task"))
        .map(|digits| {
            let first = *digits.0.first().expect("no digits in string");
            let last = *digits.0.last().expect("no digits in string");
            let num = 10 * first + last;

            num as u32
        })
        .sum();
    println!("{res}");
    Ok(())
}

#[derive(Debug)]
struct Digits(Vec<u8>);

const DIGITS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_letters(s: &str) -> Option<(u8, &str)> {
    DIGITS.into_iter().zip(1..).find_map(|(digit, num)| {
        if s.starts_with(digit) {
            // let (_current, rest) = s.split_at(digit.len());
            Some((num, &s[1..]))
        } else {
            None
        }
    })
}

fn parse_digit(s: &str) -> Option<(u8, &str)> {
    let first = s.chars().next()?;
    if first.is_ascii_digit() {
        Some((first.to_string().parse().unwrap(), &s[1..]))
    } else {
        None
    }
}

fn parse_next(s: &str) -> Option<(u8, &str)> {
    let from_digit = parse_digit(s);
    if from_digit.is_some() {
        from_digit
    } else {
        parse_letters(s)
    }
}

impl Digits {
    fn parse_str(s: &str, parsed: &mut Vec<u8>) {
        if s.len() == 0 {
            return;
        }

        match parse_next(s) {
            Some((digit, rest)) => {
                parsed.push(digit);
                Self::parse_str(rest, parsed);
            }
            None => Self::parse_str(&s[1..], parsed),
        }
    }
}
impl FromStr for Digits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::new();
        Self::parse_str(s, &mut digits);
        Ok(Self(digits))
    }
}

#[cfg(test)]
mod tests {
    use crate::Digits;

    #[test]
    fn show_digits() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let res: Vec<_> = input
            .lines()
            .map(|l| l.parse::<Digits>().unwrap())
            .map(|digits| digits.0)
            .collect();
        let expected = vec![
            vec![2, 1, 9],
            vec![8, 2, 3],
            vec![1, 2, 3],
            vec![2, 1, 3, 4],
            vec![4, 9, 8, 7, 2],
            vec![1, 8, 2, 3, 4],
            vec![7, 6],
        ];
        assert_eq!(res, expected);
    }
}
