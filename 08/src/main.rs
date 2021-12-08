// Day 8: Seven Segment Search
// https://adventofcode.com/2021/day/8

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;
use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let entries = read_entries(&path);
    println!(
        "Part 1: Total easy digits = {}",
        total_easy_digits(&entries)
    );
    println!(
        "Part 2: Sum of all output values = {}",
        sum_of_output_values(&entries)
    );
}

fn read_entries(path: impl AsRef<Path>) -> Vec<Entry> {
    let f = File::open(path).expect("Error opening input file");
    BufReader::new(f)
        .lines()
        .map(|line| {
            line.expect("Error reading line")
                .parse::<Entry>()
                .expect("Error parsing line")
        })
        .collect()
}

fn total_easy_digits(entries: &[Entry]) -> usize {
    entries
        .iter()
        .flat_map(|entry| {
            entry
                .output
                .iter()
                .filter(|pattern| matches!(pattern.len(), 2..=4 | 7))
        })
        .count()
}

fn sum_of_output_values(entries: &[Entry]) -> u64 {
    entries.iter().map(decode_entry).sum()
}

fn decode_entry(entry: &Entry) -> u64 {
    let mut patterns_to_numbers: HashMap<String, u8> = HashMap::new();
    let mut numbers_to_patterns: HashMap<u8, String> = HashMap::new();

    while patterns_to_numbers.len() < 10 {
        for pattern in &entry.patterns {
            if patterns_to_numbers.contains_key(pattern) {
                continue;
            }
            if let Some(number) = try_decode_pattern(pattern, &numbers_to_patterns) {
                patterns_to_numbers.insert(pattern.to_string(), number);
                numbers_to_patterns.insert(number, pattern.to_string());
            }
        }
    }

    entry
        .output
        .iter()
        .map(|pattern| {
            patterns_to_numbers
                .get(pattern)
                .expect("Unknown pattern found in output!")
        })
        .join("")
        .parse()
        .unwrap_or_default()
}

fn try_decode_pattern(pattern: &str, numbers_to_patterns: &HashMap<u8, String>) -> Option<u8> {
    match pattern.len() {
        // Some segment counts are unique, so can be mapped immediately to a digit.
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        length => {
            // For the others, we need have to wait until we have the patterns for 1 and 4,
            // in order that we can deduce the pattern's value.
            let pattern_for_one = numbers_to_patterns.get(&1)?;
            let pattern_for_four = numbers_to_patterns.get(&4)?;
            match length {
                5 => {
                    if pattern_contains_another(pattern, pattern_for_one) {
                        Some(3)
                    } else if characters_overlap_between_patterns(pattern, pattern_for_four) == 2 {
                        Some(2)
                    } else {
                        Some(5)
                    }
                }
                6 => {
                    if pattern_contains_another(pattern, pattern_for_four) {
                        Some(9)
                    } else if pattern_contains_another(pattern, pattern_for_one) {
                        Some(0)
                    } else {
                        Some(6)
                    }
                }
                _ => panic!("Invalid pattern length!"),
            }
        }
    }
}

fn pattern_contains_another(pattern: &str, sub_pattern: &str) -> bool {
    sub_pattern.chars().all(|c| pattern.contains(c))
}

fn characters_overlap_between_patterns(pattern1: &str, pattern2: &str) -> usize {
    pattern1.chars().filter(|&c| pattern2.contains(c)).count()
}

struct Entry {
    patterns: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // eg: "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
        s.split('|')
            .map(|section| {
                section
                    .split_whitespace()
                    // Sort the letters in each word alphabetically, to aid matching later.
                    .map(|s| s.chars().sorted().collect())
                    .collect::<Vec<String>>()
            })
            .collect_tuple()
            .map(|(patterns, output)| Self { patterns, output })
            .ok_or_else(|| String::from("Invalid entry"))
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
        let entries = read_entries(&example_file());
        assert_eq!(total_easy_digits(&entries), 26);
    }

    #[test]
    fn part_two_example() {
        let entries = read_entries(&example_file());
        assert_eq!(sum_of_output_values(&entries), 61229);
    }

    #[test]
    fn decode_entry_testcase_one() {
        let entry = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
        cdfeb fcadb cdfeb cdbaf"
            .parse::<Entry>()
            .unwrap();
        assert_eq!(decode_entry(&entry), 5353);
    }

    #[test]
    fn decode_entry_testcase_two() {
        let entry = "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
        ed bcgafe cdgba cbgef"
            .parse::<Entry>()
            .unwrap();
        assert_eq!(decode_entry(&entry), 1625);
    }
}
