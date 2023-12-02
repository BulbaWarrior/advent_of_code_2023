use std::{error::Error, io::stdin, str::FromStr};

#[derive(Debug, Default, PartialEq, Eq)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_game, rest) = s.split_once("Game ").unwrap();
        let (id, rest) = rest.split_once(":").unwrap();
        let id = id.parse().unwrap();
        let mut game = Game { id, rounds: vec![] };
        rest.split(";").for_each(|round_s| {
            let mut round = Round::default();
            round_s.split(",").for_each(|col| {
                let col = col.trim();
                let (num, col) = col.split_once(" ").unwrap();
                let count: usize = num.parse().unwrap();
                match col {
                    "red" => round.red = count,
                    "green" => round.green = count,
                    "blue" => round.blue = count,
                    _ => panic!("unknown color"),
                }
            });
            game.rounds.push(round);
        });
        Ok(game)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let res = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let game = line.parse::<Game>().unwrap();
            let maxes = game.rounds.into_iter().fold((0, 0, 0), |(r, g, b), round| {
                (r.max(round.red), g.max(round.green), b.max(round.blue))
            });

            let (r, g, b) = maxes;
            match (r <= 12, g <= 13, b <= 14) {
                (true, true, true) => game.id,
                _ => 0,
            }
        })
        .sum::<usize>();

    println!("{res}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Game, Round};

    #[test]
    fn parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let res = input.parse::<Game>().unwrap();
        dbg!(&res);
        assert_eq!(
            res,
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Round {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            }
        )
    }
}
