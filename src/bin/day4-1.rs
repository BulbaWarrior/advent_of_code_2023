use std::io::stdin;

fn main() {
    let res = stdin()
        .lines()
        .flatten()
        .map(|line| {
            let (_card_name, numbers) = line.split_once(':').unwrap();
            let (winning, got) = numbers.split_once('|').unwrap();
            let winning = winning
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>();
            let got = got
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>();
            let got_winnig = got.into_iter().filter(|x| winning.contains(x));
            let count = got_winnig.count() as u32;
            if count == 0 {
                0
            } else {
                2u32.pow(count as u32 - 1)
            }
        })
        .sum::<u32>();
    println!("{res}");
}
