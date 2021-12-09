// Day 9: Smoke Basin
// https://adventofcode.com/2021/day/9

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::collections::HashMap;
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
    let heights = read_heights(&path);
    println!(
        "Part 1: Sum of risk levels for low points = {}",
        low_points_total_risk(&heights)
    );
    println!(
        "Part 2: Largest three basin sizes multiplied = {}",
        largest_three_basin_sizes_multiplied(&heights)
    );
}

fn read_heights(path: impl AsRef<Path>) -> Vec<Vec<u32>> {
    let f = File::open(path).expect("Error opening input file");
    BufReader::new(f)
        .lines()
        .map(|line| {
            line.expect("Error reading line")
                .chars()
                .map(|char| char.to_digit(10).expect("Error parsing line"))
                .collect()
        })
        .collect()
}

fn low_points_total_risk(heights: &[Vec<u32>]) -> u32 {
    const MAX_HEIGHT: u32 = 9;
    let mut total_risk = 0;
    let mut row_above: Option<&Vec<u32>> = None;

    for (row_num, row) in heights.iter().enumerate() {
        let mut previous_height: Option<&u32> = None;
        for (col_num, height) in row.iter().enumerate() {
            let height_left = previous_height.unwrap_or(&MAX_HEIGHT);
            let height_right = row.get(col_num + 1).unwrap_or(&MAX_HEIGHT);
            let height_above = row_above
                .and_then(|row| row.get(col_num))
                .unwrap_or(&MAX_HEIGHT);
            let height_below = heights
                .get(row_num + 1)
                .and_then(|row| row.get(col_num))
                .unwrap_or(&MAX_HEIGHT);
            if height < height_left
                && height < height_right
                && height < height_above
                && height < height_below
            {
                total_risk += height + 1;
            }
            previous_height = Some(height);
        }
        row_above = Some(row);
    }
    total_risk
}

fn largest_three_basin_sizes_multiplied(heights: &[Vec<u32>]) -> usize {
    let mut latest_basin_id: usize = 0;
    let mut points_to_basin_id: HashMap<(usize, usize), usize> = HashMap::new();
    let mut basin_sizes: HashMap<usize, usize> = HashMap::new();

    for (row_num, row) in heights.iter().enumerate() {
        for (col_num, height) in row.iter().enumerate() {
            // Heights of 9 aren't counted in the basin.
            if *height == 9 {
                continue;
            }

            let basin_id_left = if col_num > 0 {
                points_to_basin_id.get(&(row_num, col_num - 1))
            } else {
                None
            };
            let basin_id_above = if row_num > 0 {
                points_to_basin_id.get(&(row_num - 1, col_num))
            } else {
                None
            };

            let current_basin_id = match (basin_id_left, basin_id_above) {
                // This current location joins up two previously separate basins, so we need to
                // merge the two basin IDs. Here we chose to merge into the ID of the basin above.
                (Some(&id_left), Some(&id_above)) => {
                    // Update all points mapped to the left basin to instead map to the basin above.
                    points_to_basin_id
                        .values_mut()
                        .filter(|id| **id == id_left)
                        .for_each(|id| *id = id_above);
                    // And update the basin size mappings accordingly.
                    let left_basin_size = basin_sizes.remove(&id_left).unwrap();
                    let above_basin_size = basin_sizes.entry(id_above).or_insert(0);
                    *above_basin_size += left_basin_size;
                    id_above
                }
                // There is a single existing basin, so continue using its ID.
                (Some(id), None) | (None, Some(id)) => *id,
                // This is a new basin, so generate a new ID.
                (None, None) => {
                    latest_basin_id += 1;
                    latest_basin_id
                }
            };
            points_to_basin_id.insert((row_num, col_num), current_basin_id);
            let size = basin_sizes.entry(current_basin_id).or_insert(0);
            *size += 1;
        }
    }

    basin_sizes.values().sorted().rev().take(3).product()
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
        let heights = read_heights(&example_file());
        assert_eq!(low_points_total_risk(&heights), 15);
    }

    #[test]
    fn part_two_example() {
        let heights = read_heights(&example_file());
        assert_eq!(largest_three_basin_sizes_multiplied(&heights), 1134);
    }
}
