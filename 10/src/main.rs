// Day 10: Syntax Scoring
// https://adventofcode.com/2021/day/10

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

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
    let lines = read_lines(&path);
    println!(
        "Part 1: Total score for corrupted lines = {}",
        corrupted_lines_total_score(&lines)
    );
    println!(
        "Part 2: Median score for incomplete lines = {}",
        incomplete_lines_median_score(&lines)
    );
}

fn read_lines(path: impl AsRef<Path>) -> Vec<Vec<char>> {
    let f = File::open(path).expect("Error opening input file");
    BufReader::new(f)
        .lines()
        .map(|line| line.expect("Error reading line").chars().collect())
        .collect()
}

fn corrupted_lines_total_score(lines: &[Vec<char>]) -> u64 {
    lines
        .iter()
        .filter_map(|line| {
            if let LineType::Corrupted(score) = analyse_line(line) {
                Some(score)
            } else {
                None
            }
        })
        .sum()
}

fn incomplete_lines_median_score(lines: &[Vec<char>]) -> u64 {
    let mut scores: Vec<u64> = lines
        .iter()
        .filter_map(|line| {
            if let LineType::Incomplete(score) = analyse_line(line) {
                Some(score)
            } else {
                None
            }
        })
        .collect();

    scores.sort_unstable();
    let midpoint = scores.len() / 2;
    scores[midpoint]
}

fn analyse_line(line: &[char]) -> LineType {
    let mut open_chunks: Vec<char> = Vec::new();

    for &symbol in line {
        match symbol {
            '(' | '[' | '{' | '<' => open_chunks.push(symbol),
            ')' | ']' | '}' | '>' => {
                let expected_closing_symbol = open_chunks.pop().map(expected_closing_symbol);
                if expected_closing_symbol != Some(symbol) {
                    return LineType::Corrupted(corrupted_symbol_score(symbol));
                }
            }
            _ => panic!("Unknown symbol: {}", symbol),
        };
    }

    if open_chunks.is_empty() {
        LineType::Valid
    } else {
        LineType::Incomplete(incomplete_chunks_score(&open_chunks))
    }
}

fn expected_closing_symbol(opening_symbol: char) -> char {
    match opening_symbol {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unknown symbol: {}", opening_symbol),
    }
}

fn corrupted_symbol_score(symbol: char) -> u64 {
    match symbol {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown symbol: {}", symbol),
    }
}

fn incomplete_chunks_score(open_chunks: &[char]) -> u64 {
    open_chunks.iter().rev().fold(0, |total, &opening_symbol| {
        let missing_symbol = expected_closing_symbol(opening_symbol);
        total * 5 + missing_symbol_score(missing_symbol)
    })
}

fn missing_symbol_score(symbol: char) -> u64 {
    match symbol {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Unknown symbol: {}", symbol),
    }
}

enum LineType {
    Valid,
    Corrupted(u64),
    Incomplete(u64),
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
        let lines = read_lines(&example_file());
        assert_eq!(corrupted_lines_total_score(&lines), 26397);
    }

    #[test]
    fn part_two_example() {
        let lines = read_lines(&example_file());
        assert_eq!(incomplete_lines_median_score(&lines), 288_957);
    }
}
