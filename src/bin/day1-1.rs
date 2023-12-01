use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let res: u32 = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| {
            let first = s
                .chars()
                .find(|c| c.is_ascii_digit())
                .expect("no digits in string");
            let last = s
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .expect("no digits in string");
            let num: String = [first, last].into_iter().collect();
            let num: u32 = num
                .parse()
                .expect("could not parse two digits into a number");
            num
        })
        .sum();
    println!("{res}");
    Ok(())
}
