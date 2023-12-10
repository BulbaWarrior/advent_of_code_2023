use std::io::stdin;

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

    let stars = input
        .clone()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(col, ch)| (ch == '*').then_some((col, row)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let numbers = input
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
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let res: usize = stars
        .into_iter()
        .flat_map(|star| {
            let (col, row) = star;
            let neighbors = numbers
                .iter()
                .filter(|part| {
                    let vertically = row.abs_diff(part.row) <= 1;
                    let (left, right) = part.cols;
                    let horizontally = col >= left.checked_sub(1).unwrap_or(0) && col <= right;
                    vertically && horizontally
                })
                .collect::<Vec<_>>();
            if neighbors.len() == 2 {
                Some(
                    neighbors
                        .into_iter()
                        .map(|part| part.val)
                        .product::<usize>(),
                )
            } else {
                None
            }
        })
        .sum();
    println!("{res}");
}
