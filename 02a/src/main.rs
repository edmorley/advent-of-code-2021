#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;
use std::str::FromStr;

// https://adventofcode.com/2021/day/2
fn main() {
    let commands_file = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let position = calculate_position(commands_file);
    println!(
        "Position: {:?}. Horizontal x Depth = {}",
        position,
        position.horizontal * position.depth
    );
}

fn calculate_position(commands_file: impl AsRef<Path>) -> Position {
    let f = File::open(commands_file).expect("Error opening input file");
    let reader = BufReader::new(f);

    let commands = reader.lines().map(|line| {
        line.expect("Error reading line")
            .parse::<Command>()
            .expect("Error parsing line")
    });
    let starting_position = Position {
        horizontal: 0,
        depth: 0,
    };

    commands.fold(starting_position, |position, command| {
        position.get_position_after_command(&command)
    })
}

#[derive(Debug, PartialEq)]
struct Position {
    horizontal: u64,
    depth: u64,
}

impl Position {
    fn get_position_after_command(&self, command: &Command) -> Self {
        match command.direction {
            Direction::Forward => Self {
                horizontal: self.horizontal + command.amount,
                depth: self.depth,
            },
            // Since this is depth, `Up` decreases the value, rather than the inverse.
            Direction::Up => Self {
                horizontal: self.horizontal,
                depth: self.depth - command.amount,
            },
            Direction::Down => Self {
                horizontal: self.horizontal,
                depth: self.depth + command.amount,
            },
        }
    }
}

struct Command {
    direction: Direction,
    amount: u64,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some((direction, amount)) = value.split_once(' ') {
            Ok(Command {
                direction: direction.parse()?,
                amount: amount.parse().map_err(|_| ParseError::Amount)?,
            })
        } else {
            Err(ParseError::Command)
        }
    }
}

enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(ParseError::Direction),
        }
    }
}

#[derive(Debug)]
enum ParseError {
    Command,
    Direction,
    Amount,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn example() {
        let example_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("example.txt");
        let position = calculate_position(example_path);
        assert_eq!(position.horizontal, 15);
        assert_eq!(position.depth, 10);
        assert_eq!(position.horizontal * position.depth, 150);
    }
}
