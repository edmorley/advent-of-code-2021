// https://adventofcode.com/2021/day/1

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

use itertools::Itertools;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let depths = read_depths(&path);
    println!("Total depth increases...");
    println!("Part 1: {}", calculate_increases_simple(&depths));
    println!("Part 2: {}", calculate_increases_sliding_window(&depths));
}

fn read_depths(path: impl AsRef<Path>) -> Vec<u64> {
    let f = File::open(path).expect("Error opening input file");
    BufReader::new(f)
        .lines()
        .map(|line| {
            line.expect("Error reading line")
                .parse::<u64>()
                .expect("Error parsing line")
        })
        .collect()
}

fn calculate_increases_simple(depths: &[u64]) -> usize {
    depths
        .iter()
        .tuple_windows::<(_, _)>()
        .filter(|(current, next)| next > current)
        .count()
}

fn calculate_increases_sliding_window(depths: &[u64]) -> usize {
    // We need to compare the sum of measurements "ABC" with "BCD". Since "BC" overlaps
    // between the two, we can instead take a window of size four (rather than two windows
    // of size three and have to flatten) and compare only the first and last values.
    depths
        .iter()
        .tuple_windows::<(_, _, _, _)>()
        .filter(|(first, _, _, forth)| forth > first)
        .count()
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
        let depths = read_depths(&example_file());
        assert_eq!(calculate_increases_simple(&depths), 7);
    }

    #[test]
    fn part_two_example() {
        let depths = read_depths(&example_file());
        assert_eq!(calculate_increases_sliding_window(&depths), 5);
    }
}
