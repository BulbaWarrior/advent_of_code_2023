use std::{io::stdin, str::FromStr};

#[derive(Debug, Clone)]
struct PartNumber {
    val: usize,
    row: usize,
    cols: (usize, usize),
}

fn main() {
    // coordinates of symbols
    let input = stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .into_iter();

    let symbols = input
        .clone()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(col, ch)| (ch != '.' && !ch.is_ascii_digit()).then_some((col, row)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // dbg!(&symbols);
    let res = input
        .enumerate()
        .flat_map(|(row, line)| {
            // for multiple numbers per line
            let mut offset = 0;
            line.split(|ch: char| !ch.is_ascii_digit())
                .enumerate()
                .flat_map(|(i, s)| {
                    let col = i + offset;
                    // filter out empty slices after dots and symbols (they will not parse)
                    let val = (s.len() != 0).then(|| s.parse::<usize>().ok())??;
                    offset += s.len();
                    Some(PartNumber {
                        val,
                        row,
                        cols: (col, col + s.len()),
                    })
                })
                // filter out parts that are not adjacent
                .flat_map(|part| {
                    symbols
                        .iter()
                        .copied()
                        // yes, this looks through all the symbols linearly, but I'm lazy \shrug
                        .find(|&(col, row)| {
                            let vertically = row.abs_diff(part.row) <= 1;
                            let (left, right) = part.cols;
                            let horizontally =
                                col >= left.checked_sub(1).unwrap_or(0) && col <= right;
                            vertically && horizontally
                        })
                        .and(Some(part))
                })
                // .map(|part| dbg!(part))
                .collect::<Vec<_>>()
        })
        // only valid parts that are adjecent to symbols at this point
        .map(|part| part.val)
        .sum::<usize>();
    println!("{res}");
}

struct PartsRow(Vec<(usize, usize)>);

impl FromStr for PartsRow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
