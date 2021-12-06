// Day 6: Lanternfish
// https://adventofcode.com/2021/day/6

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::path::Path;
use std::process;
use std::{env, fs};

use counter::Counter;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let timer_value_counts = read_timer_value_counts(&path);
    println!(
        "Part 1: Total fish after 80 days = {}",
        total_fish_after_days(&timer_value_counts, 80)
    );
    println!(
        "Part 2: Total fish after 256 days = {}",
        total_fish_after_days(&timer_value_counts, 256)
    );
}

// Reads a file containing a single line of comma delimited fish timer values (eg "3,4,3,1,2"),
// and returns a map of fish timer values to the count of fish with that value.
fn read_timer_value_counts(path: impl AsRef<Path>) -> HashMap<u8, usize> {
    let file_contents = fs::read_to_string(path).expect("Error reading input file");
    file_contents
        .trim()
        .split(',')
        .map(|line| line.parse::<u8>().expect("Error parsing line"))
        .collect::<Counter<_>>()
        .into_map()
}

fn total_fish_after_days(initial_timer_value_counts: &HashMap<u8, usize>, days: u16) -> usize {
    let mut timer_value_counts = initial_timer_value_counts.clone();

    for _ in 1..=days {
        let mut new_timer_value_counts: HashMap<u8, usize> = HashMap::new();
        for (&timer_value, &count) in &timer_value_counts {
            match timer_value {
                0 => {
                    // Reset the zero-timer fish back to 6.
                    let new_count = new_timer_value_counts.entry(6).or_insert(0);
                    *new_count += count;
                    // Add an identical number of new fish.
                    // Not using the entry API since there will never be an existing entry for 8.
                    new_timer_value_counts.insert(8, count);
                }
                1..=8 => {
                    // Decrement the timer for all other fish.
                    let new_count = new_timer_value_counts.entry(timer_value - 1).or_insert(0);
                    *new_count += count;
                }
                _ => panic!("Invalid timer value: {}", timer_value),
            }
        }
        timer_value_counts = new_timer_value_counts;
    }

    // Total fish.
    timer_value_counts.values().sum()
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
        let timer_value_counts = read_timer_value_counts(&example_file());
        assert_eq!(total_fish_after_days(&timer_value_counts, 80), 5934);
    }

    #[test]
    fn part_two_example() {
        let timer_value_counts = read_timer_value_counts(&example_file());
        assert_eq!(
            total_fish_after_days(&timer_value_counts, 256),
            26_984_457_539
        );
    }
}
