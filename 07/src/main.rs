// Day 7: The Treachery of Whales
// https://adventofcode.com/2021/day/7

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::path::Path;
use std::process;
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let positions = read_positions(&path);
    let (position, fuel_cost) = optimal_position_part_one(&positions);
    println!("Part 1: Position = {}, fuel cost = {}", position, fuel_cost);
    let (position, fuel_cost) = optimal_position_part_two(&positions);
    println!("Part 2: Position = {}, fuel cost = {}", position, fuel_cost);
}

fn read_positions(path: impl AsRef<Path>) -> Vec<i64> {
    let file_contents = fs::read_to_string(path).expect("Error reading input file");
    file_contents
        .trim()
        .split(',')
        .map(|line| line.parse::<i64>().expect("Error parsing line"))
        .collect()
}

fn optimal_position_part_one(positions: &[i64]) -> (i64, u64) {
    // We can skip comparing the fuel cost from every possible position since the
    // median will always be the optimal position. If there are an even number of
    // elements both will have the same fuel cost, so it's fine to pick either.
    let mut positions = positions.to_vec();
    positions.sort_unstable();
    let midpoint = positions.len() / 2;
    let median_position = positions[midpoint];
    let total_fuel = positions.iter().fold(0, |total_fuel, position| {
        let moves_required = (position - median_position).abs() as u64;
        total_fuel + moves_required
    });
    (median_position, total_fuel)
}

fn optimal_position_part_two(positions: &[i64]) -> (i64, u64) {
    // Since fuel cost is now equivalent to the triangular number of moves performed
    // by each crab, the optimal position is no longer equivalent to the median.
    let min_position = *positions.iter().min().unwrap_or(&0);
    let max_position = *positions.iter().max().unwrap_or(&0);
    let (optimal_position, total_fuel) = (min_position..=max_position)
        .map(|proposed_position| {
            let total_fuel = positions.iter().fold(0, |total_fuel, position| {
                let moves_required = (position - proposed_position).abs() as u64;
                total_fuel + triangular_number(moves_required)
            });
            (proposed_position, total_fuel)
        })
        .min_by_key(|(_, total_fuel)| *total_fuel)
        .unwrap_or_default();
    (optimal_position, total_fuel)
}

// https://en.wikipedia.org/wiki/Triangular_number
fn triangular_number(n: u64) -> u64 {
    n * (n + 1) / 2
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
        let positions = read_positions(example_file());
        let (position, fuel_cost) = optimal_position_part_one(&positions);
        assert_eq!(position, 2);
        assert_eq!(fuel_cost, 37);
    }

    #[test]
    fn part_two_example() {
        let positions = read_positions(example_file());
        let (position, fuel_cost) = optimal_position_part_two(&positions);
        assert_eq!(position, 5);
        assert_eq!(fuel_cost, 168);
    }
}
