// Day 15: Chiton
// https://adventofcode.com/2021/day/15

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::cmp::Reverse;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

use priority_queue::PriorityQueue;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let risk_levels = read_risk_levels(&path);
    println!(
        "Part 1: Lowest total risk = {}",
        lowest_total_risk(&risk_levels)
    );
    let expanded_risk_levels = generate_expanded_map(&risk_levels, 5);
    println!(
        "Part 2: Lowest total risk with expanded map = {}",
        lowest_total_risk(&expanded_risk_levels)
    );
}

fn read_risk_levels(path: impl AsRef<Path>) -> HashMap<Point, u32> {
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

fn lowest_total_risk(risk_levels: &HashMap<Point, u32>) -> u32 {
    let mut risk_from_source: HashMap<Point, u32> = HashMap::new();
    // Using unpopulated (aside from start point) priority queue, since was quicker than the
    // populated variant (440ms vs 610ms for `cargo run --release input.txt`). See more about
    // Dijkstra's variants here:
    // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Using_a_priority_queue
    let mut queue = PriorityQueue::new();

    let start_point = (0, 0);
    let end_point = *risk_levels.keys().max().unwrap_or(&start_point);
    risk_from_source.insert(start_point, 0);
    queue.push(start_point, Reverse(0));

    while let Some((point, Reverse(total_risk))) = queue.pop() {
        if point == end_point {
            break;
        }

        for adjacent_point in get_adjacent_points(point) {
            if let Some(adjacent_risk) = risk_levels.get(&adjacent_point) {
                let new_adjacent_total_risk = total_risk + adjacent_risk;
                let existing_adjacent_total_risk =
                    risk_from_source.get(&adjacent_point).unwrap_or(&u32::MAX);

                if new_adjacent_total_risk < *existing_adjacent_total_risk {
                    risk_from_source.insert(adjacent_point, new_adjacent_total_risk);
                    queue.push_increase(adjacent_point, Reverse(new_adjacent_total_risk));
                }
            }
        }
    }

    *risk_from_source.get(&end_point).unwrap_or(&0)
}

fn get_adjacent_points((row, column): Point) -> impl Iterator<Item = Point> {
    [(-1, 0), (0, -1), (0, 1), (1, 0)]
        .iter()
        .map(move |(row_offset, column_offset)| (row + row_offset, column + column_offset))
}

fn generate_expanded_map(risk_levels: &HashMap<Point, u32>, multiplier: u8) -> HashMap<Point, u32> {
    let grid_width = *risk_levels.keys().map(|(x, _)| x).max().unwrap() + 1;

    risk_levels
        .iter()
        .flat_map(|((x, y), risk)| {
            (0..multiplier).flat_map(move |x_offset| {
                (0..multiplier).map(move |y_offset| {
                    let new_x = x + i64::from(x_offset) * grid_width;
                    let new_y = y + i64::from(y_offset) * grid_width;
                    let new_risk = (risk + u32::from(x_offset + y_offset) - 1) % 9 + 1;
                    ((new_x, new_y), new_risk)
                })
            })
        })
        .collect()
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
        let risk_levels = read_risk_levels(&example_file());
        assert_eq!(lowest_total_risk(&risk_levels), 40);
    }

    #[test]
    fn part_two_example() {
        let risk_levels = read_risk_levels(&example_file());
        let expanded_risk_levels = generate_expanded_map(&risk_levels, 5);
        assert_eq!(lowest_total_risk(&expanded_risk_levels), 315);
    }

    #[test]
    fn part_two_generate_expanded_map() {
        let expanded_example_file =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("example-expanded.txt");
        let expected_expanded_risk_levels = read_risk_levels(expanded_example_file);
        let original_risk_levels = read_risk_levels(example_file());
        assert_eq!(
            generate_expanded_map(&original_risk_levels, 5),
            expected_expanded_risk_levels
        );
    }
}
