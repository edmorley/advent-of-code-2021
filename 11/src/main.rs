// Day 11: Dumbo Octopus
// https://adventofcode.com/2021/day/11

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let octopuses = read_octopuses(&path);
    println!(
        "Part 1: Total flashes after 100 steps = {}",
        total_flashes(octopuses.clone(), 100)
    );
    println!(
        "Part 2: First step number when all flashes synchronised = {}",
        first_step_when_synchronised(octopuses)
    );
}

// Returns a map of octopus locations to energy levels.
// Side note: It turns out "octopi" is not grammatically correct:
// https://en.wikipedia.org/wiki/Octopus#Etymology_and_pluralisation
fn read_octopuses(path: impl AsRef<Path>) -> HashMap<Point, u32> {
    let f = File::open(path).expect("Error opening input file");
    BufReader::new(f)
        .lines()
        .enumerate()
        .flat_map(|(row_num, line)| {
            line.expect("Error reading line")
                .chars()
                .enumerate()
                .map(|(col_num, char)| {
                    let digit = char.to_digit(10).expect("Error parsing line");
                    ((row_num as i64, col_num as i64), digit)
                })
                .collect::<HashMap<_, _>>()
        })
        .collect()
}

fn total_flashes(mut octopuses: HashMap<Point, u32>, steps: u64) -> u64 {
    let mut total_flashes = 0;
    for _ in 1..=steps {
        total_flashes += perform_step(&mut octopuses);
    }
    total_flashes
}

fn first_step_when_synchronised(mut octopuses: HashMap<Point, u32>) -> u64 {
    let total_octopuses = octopuses.len();
    let mut step = 0;

    loop {
        step += 1;
        let flashes = perform_step(&mut octopuses);
        if flashes == total_octopuses.try_into().unwrap() {
            return step;
        }
    }
}

fn perform_step(octopuses: &mut HashMap<Point, u32>) -> u64 {
    let mut flashes = 0;
    let mut pending_flash: Vec<Point> = Vec::new();

    for (point, energy_level) in octopuses.iter_mut() {
        *energy_level += 1;
        if *energy_level == 10 {
            pending_flash.push(*point);
        }
    }

    while let Some(point) = pending_flash.pop() {
        flashes += 1;

        for adjacent_point in get_adjacent_points(point) {
            if let Some(energy_level) = octopuses.get_mut(&adjacent_point) {
                *energy_level += 1;
                if *energy_level == 10 {
                    pending_flash.push(adjacent_point);
                }
            }
        }
    }

    // Reset all flashed octopuses back to zero.
    octopuses
        .values_mut()
        .filter(|energy_level| **energy_level > 9)
        .for_each(|energy_level| *energy_level = 0);

    flashes
}

fn get_adjacent_points((row, column): Point) -> impl Iterator<Item = Point> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .map(move |(row_offset, column_offset)| (row + row_offset, column + column_offset))
}

type Point = (i64, i64);

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn example_file() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("example.txt")
    }

    #[test]
    fn part_one_example() {
        let octopuses = read_octopuses(&example_file());
        assert_eq!(total_flashes(octopuses, 100), 1656);
    }

    #[test]
    fn part_two_example() {
        let octopuses = read_octopuses(&example_file());
        assert_eq!(first_step_when_synchronised(octopuses), 195);
    }
}
