// Day 13: Transparent Origami
// https://adventofcode.com/2021/day/13

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]
// Required due to use of Itertools::intersperse
#![allow(unstable_name_collisions)]

use std::cmp::Ordering;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::path::Path;
use std::process;
use std::str::FromStr;
use std::{env, fs};

use itertools::Itertools;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let paper = read_paper(&path);
    println!(
        "Part 1: Total dots after first fold = {}",
        total_dots_after_first_fold(&paper)
    );
    println!(
        "Part 2: Decoded message after all folds...\n{}",
        decoded_message(&paper)
    );
}

fn read_paper(path: impl AsRef<Path>) -> Paper {
    let file_contents = fs::read_to_string(path).expect("Error reading input file");
    file_contents
        .parse::<Paper>()
        .expect("Error parsing input file")
}

fn total_dots_after_first_fold(paper: &Paper) -> usize {
    if let Some(fold) = paper.folds.first() {
        get_dots_after_fold(&paper.dots, fold).len()
    } else {
        0
    }
}

fn decoded_message(paper: &Paper) -> String {
    let final_dots = paper.folds.iter().fold(paper.dots.clone(), |dots, fold| {
        get_dots_after_fold(&dots, fold)
    });

    dots_to_message(&final_dots)
}

fn get_dots_after_fold(dots: &HashSet<(u64, u64)>, folds: &Fold) -> HashSet<(u64, u64)> {
    dots.iter()
        .filter_map(|(x, y)| {
            // If dots are above/to the left of the fold line, then they are preserved.
            // If they are on the fold line, they are dropped.
            // Or if the are below/to the right of the fold line, then they must be translated.
            match folds {
                Fold::X(fold_at) => match x.cmp(fold_at) {
                    Ordering::Less => Some((*x, *y)),
                    Ordering::Equal => None,
                    Ordering::Greater => Some((2 * fold_at - x, *y)),
                },
                Fold::Y(fold_at) => match y.cmp(fold_at) {
                    Ordering::Less => Some((*x, *y)),
                    Ordering::Equal => None,
                    Ordering::Greater => Some((*x, 2 * fold_at - y)),
                },
            }
        })
        .collect()
}

fn dots_to_message(dots: &HashSet<(u64, u64)>) -> String {
    let max_x = dots.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let max_y = dots.iter().map(|(_, y)| *y).max().unwrap_or(0);

    (0..=max_y)
        .map(|y| {
            (0..=max_x)
                .map(|x| if dots.contains(&(x, y)) { '#' } else { ' ' })
                .collect::<String>()
        })
        .intersperse(String::from("\n"))
        .collect()
}

struct Paper {
    dots: HashSet<(u64, u64)>,
    folds: Vec<Fold>,
}

impl FromStr for Paper {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split_terminator("\n\n");

        let dots = sections
            .next()
            .unwrap_or_default()
            .lines()
            .map(|point| {
                let (x, y) = point.split_once(',').unwrap_or_default();
                Ok((x.parse::<u64>()?, y.parse::<u64>()?))
            })
            .collect::<Result<_, ParseIntError>>()
            .map_err(|_| String::from("Invalid points"))?;

        let folds = sections
            .next()
            .unwrap_or_default()
            .lines()
            .map(str::parse::<Fold>)
            .collect::<Result<_, _>>()?;

        Ok(Self { dots, folds })
    }
}

enum Fold {
    X(u64),
    Y(u64),
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // eg: "fold along y=7"
        s.split_once('=')
            .and_then(|(axis, value)| {
                let value = value.parse::<u64>().ok()?;
                match axis {
                    "fold along x" => Some(Fold::X(value)),
                    "fold along y" => Some(Fold::Y(value)),
                    _ => None,
                }
            })
            .ok_or_else(|| String::from("Invalid fold instruction"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::path::{Path, PathBuf};

    fn example_file() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("example.txt")
    }

    #[test]
    fn part_one_example() {
        let paper = read_paper(&example_file());
        assert_eq!(total_dots_after_first_fold(&paper), 17);
    }

    #[test]
    fn part_two_example() {
        let paper = read_paper(&example_file());
        let expected_message = indoc! {"
            #####
            #   #
            #   #
            #   #
            #####"};
        assert_eq!(decoded_message(&paper), expected_message);
    }
}
