use std::io::stdin;

fn main() {
    let cards = stdin()
        .lines()
        .flatten()
        // ids start with zero for vec addressing
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
            let won = got_winnig.count();
            won
        })
        .collect::<Vec<_>>();

    let mut counts = Vec::new();
    for &card_won in cards.iter().rev() {
        match card_won {
            0 => counts.push(1),
            n => {
                // since we are going in reverse, these are cards below the winning cards
                let count = 1 + counts.iter().rev().take(n).sum::<usize>();
                counts.push(count);
            }
        }
    }
    let res = counts.into_iter().sum::<usize>();
    println!("{res}");
}
