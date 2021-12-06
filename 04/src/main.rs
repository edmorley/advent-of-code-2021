// Day 4: Giant Squid
// https://adventofcode.com/2021/day/4

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::num::ParseIntError;
use std::path::Path;
use std::process;
use std::str::FromStr;
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let bingo_game = read_bingo_game(&path);
    let (winning_score, losing_score) = winning_losing_game_scores(bingo_game);
    println!("Part 1: Winning score = {:?}", winning_score);
    println!("Part 2: Losing score = {:?}", losing_score);
}

fn read_bingo_game(path: impl AsRef<Path>) -> BingoGame {
    let file_contents = fs::read_to_string(path).expect("Error reading input file");
    file_contents
        .parse::<BingoGame>()
        .expect("Error parsing input file")
}

fn winning_losing_game_scores(mut bingo_game: BingoGame) -> (Option<u64>, Option<u64>) {
    let mut scores = Vec::new();

    for number in bingo_game.numbers_to_be_drawn {
        for card in &mut bingo_game.cards {
            if card.still_playing && card.mark_number(number) {
                scores.push(number * card.sum_of_unmatched());
            }
        }
    }

    let winning_score = scores.first();
    let losing_score = scores.last();
    (winning_score.copied(), losing_score.copied())
}

struct BingoGame {
    numbers_to_be_drawn: Vec<u64>,
    cards: Vec<BingoCard>,
}

impl FromStr for BingoGame {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split_terminator("\n\n");

        let numbers_to_be_drawn: Vec<u64> = sections
            .next()
            .unwrap_or_default()
            .split(',')
            .map(str::parse::<u64>)
            .collect::<Result<_, _>>()?;

        let cards: Vec<BingoCard> = sections
            .map(str::parse::<BingoCard>)
            .collect::<Result<_, _>>()?;

        Ok(BingoGame {
            numbers_to_be_drawn,
            cards,
        })
    }
}

struct BingoCard {
    unmatched_number_locations: HashMap<u64, (usize, usize)>,
    row_match_counts: HashMap<usize, usize>,
    column_match_counts: HashMap<usize, usize>,
    still_playing: bool,
}

impl BingoCard {
    fn mark_number(&mut self, number: u64) -> bool {
        if let Some((row_number, column_number)) = self.unmatched_number_locations.remove(&number) {
            let row_match_count = self.row_match_counts.entry(row_number).or_insert(0);
            let column_match_count = self.column_match_counts.entry(column_number).or_insert(0);
            *row_match_count += 1;
            *column_match_count += 1;
            if *row_match_count == 5 || *column_match_count == 5 {
                self.still_playing = false;
                return true;
            }
        }
        false
    }

    fn sum_of_unmatched(&self) -> u64 {
        self.unmatched_number_locations.keys().sum()
    }
}

impl FromStr for BingoCard {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Rows are newline delimited, then columns by spaces. Eg:
        // "22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19"
        // This is converted to a map of bingo number to (row, column) coordinates.
        let number_locations: HashMap<u64, (usize, usize)> =
            s.lines()
                .enumerate()
                .flat_map(|(row_num, row_string)| {
                    row_string.split_whitespace().enumerate().map(
                        move |(column_num, number_string)| {
                            Ok((number_string.parse::<u64>()?, (row_num, column_num)))
                        },
                    )
                })
                .collect::<Result<_, Self::Err>>()?;
        Ok(Self {
            unmatched_number_locations: number_locations,
            row_match_counts: HashMap::new(),
            column_match_counts: HashMap::new(),
            still_playing: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn example_file() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("example.txt")
    }

    #[test]
    fn part_one_example() {
        let bingo_game = read_bingo_game(example_file());
        let (winning_score, _) = winning_losing_game_scores(bingo_game);
        assert_eq!(winning_score, Some(4512));
    }

    #[test]
    fn part_two_example() {
        let bingo_game = read_bingo_game(example_file());
        let (_, losing_score) = winning_losing_game_scores(bingo_game);
        assert_eq!(losing_score, Some(1924));
    }
}
