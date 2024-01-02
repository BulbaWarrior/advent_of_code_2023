use std::io::{BufRead, Lines};
use std::ops::RangeInclusive;
use std::{io::stdin, str::FromStr, usize};

#[derive(Debug)]
struct MappingRange {
    source: usize,
    destination: usize,
    len: usize,
}

impl FromStr for MappingRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let &[destination, source, len] = &s
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()[..]
        else {
            unreachable!()
        };
        Ok(Self {
            source,
            destination,
            len,
        })
    }
}

#[derive(Debug)]
struct Mapping {
    name: String,
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn map(&self, x: usize) -> usize {
        for range in &self.ranges {
            // dbg!(range, x);
            if (range.source..(range.source + range.len)).contains(&x) {
                return x - range.source + range.destination;
            }
        }
        return x;
    }

    fn map_range(&self, r: RangeInclusive<usize>) -> Vec<RangeInclusive<usize>> {
        todo!()
    }
}

fn main() {
    let mut input = stdin().lines();
    let mut seeds = input
        .next()
        .map(|line| {
            line.map(|line| {
                let Some(seeds): Option<Vec<usize>> = line.split_once(':').map(|(_, s)| {
                    s.trim()
                        .split_whitespace()
                        .flat_map(|seed| seed.parse())
                        .collect()
                }) else {
                    unreachable!()
                };
                let pairs = seeds.iter().step_by(2).zip(seeds.iter().skip(1).step_by(2));
                let seeds: Vec<usize> = pairs
                    .flat_map(|(&start, &len)| start..(start + len))
                    .collect();
                seeds
            })
        })
        .unwrap()
        .unwrap();
    input.next(); // skip the first newline
    let mappings: Vec<Mapping> = Blocks::from(input)
        .filter(|b| b.len() != 0)
        .map(|block| {
            let name = block[0].to_owned();
            let ranges = block[1..]
                .into_iter()
                .map(|range| range.parse().unwrap())
                .collect::<Vec<MappingRange>>();
            Mapping { name, ranges }
        })
        .collect();

    for mapping in mappings {
        seeds = seeds.into_iter().map(|seed| mapping.map(seed)).collect();
    }

    println!("{}", seeds.into_iter().min().unwrap())
}

struct Blocks<B> {
    lines: Lines<B>,
    current: Vec<String>,
}

impl<B: BufRead> From<Lines<B>> for Blocks<B> {
    fn from(value: Lines<B>) -> Self {
        Self {
            lines: value,
            current: vec![],
        }
    }
}

impl<B: BufRead> Iterator for Blocks<B> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(Ok(x)) if x.as_str() == "" => {
                let next = std::mem::replace(&mut self.current, vec![]);
                Some(next)
            }
            Some(Ok(x)) => {
                self.current.push(x);
                self.next()
            }
            Some(Err(err)) => panic!("{err}"),
            None => {
                if self.current.len() > 0 {
                    let next = std::mem::replace(&mut self.current, vec![]);
                    Some(next)
                } else {
                    None
                }
            }
        }
    }
}
