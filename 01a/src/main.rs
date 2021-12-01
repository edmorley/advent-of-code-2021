#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

use itertools::Itertools;

// https://adventofcode.com/2021/day/1
fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let total_increases = calculate_depth_increases(path);
    println!("Total depth increases: {}", total_increases);
}

fn calculate_depth_increases(path: impl AsRef<Path>) -> usize {
    let f = File::open(path).expect("Error opening input file");
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|line| {
            line.expect("Error reading line")
                .parse::<u64>()
                .expect("Error parsing line")
        })
        .tuple_windows::<(_, _)>()
        .filter(|(current, next)| next > current)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn example() {
        let example_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("example.txt");
        assert_eq!(calculate_depth_increases(example_path), 7);
    }
}
