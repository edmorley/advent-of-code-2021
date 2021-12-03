// https://adventofcode.com/2021/day/1

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;
use std::str::FromStr;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let commands = read_commands(path);

    let position = calculate_position(&commands);
    println!(
        "Part 1: Position is {:?}. Horizontal x Depth = {}",
        position,
        position.horizontal * position.depth
    );

    let position_with_aim = calculate_position_with_aim(&commands);
    println!(
        "Part 2: Position using aim is {:?}. Horizontal x Depth = {}",
        position_with_aim,
        position_with_aim.horizontal * position_with_aim.depth
    );
}

fn read_commands(path: impl AsRef<Path>) -> Vec<Command> {
    let f = File::open(path).expect("Error opening input file");
    BufReader::new(f)
        .lines()
        .map(|line| {
            line.expect("Error reading line")
                .parse::<Command>()
                .expect("Error parsing line")
        })
        .collect()
}

fn calculate_position(commands: &[Command]) -> Position {
    let mut position = Position::new();
    for command in commands {
        position.apply_command(command);
    }
    position
}

fn calculate_position_with_aim(commands: &[Command]) -> PositionWithAim {
    let mut position = PositionWithAim::new();
    for command in commands {
        position.apply_command(command);
    }
    position
}

#[derive(Debug, PartialEq)]
struct Position {
    horizontal: u64,
    depth: u64,
}

impl Position {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }

    fn apply_command(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => self.horizontal += command.amount,
            // Since this is depth, `Up` decreases the value, rather than the inverse.
            Direction::Up => self.depth -= command.amount,
            Direction::Down => self.depth += command.amount,
        };
    }
}

#[derive(Debug, PartialEq)]
struct PositionWithAim {
    horizontal: u64,
    depth: u64,
    // Higher aim means aiming deeper.
    aim: u64,
}

impl PositionWithAim {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn apply_command(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => {
                self.horizontal += command.amount;
                self.depth += self.aim * command.amount;
            }
            // Aim shallower.
            Direction::Up => self.aim -= command.amount,
            // Aim deeper.
            Direction::Down => self.aim += command.amount,
        };
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
    use std::path::{Path, PathBuf};

    fn example_file() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("example.txt")
    }

    #[test]
    fn part_one_example() {
        let commands = read_commands(example_file());
        let position = calculate_position(&commands);
        assert_eq!(position.horizontal, 15);
        assert_eq!(position.depth, 10);
        assert_eq!(position.horizontal * position.depth, 150);
    }

    #[test]
    fn part_two_example() {
        let commands = read_commands(example_file());
        let position = calculate_position_with_aim(&commands);
        assert_eq!(position.horizontal, 15);
        assert_eq!(position.depth, 60);
        assert_eq!(position.horizontal * position.depth, 900);
    }
}
