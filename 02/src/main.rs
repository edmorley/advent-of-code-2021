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

    let position = calculate_position(&commands_file);
    println!(
        "Part 1: Position is {:?}. Horizontal x Depth = {}",
        position,
        position.horizontal * position.depth
    );

    let position_with_aim = calculate_position_with_aim(&commands_file);
    println!(
        "Part 2: Position using aim is {:?}. Horizontal x Depth = {}",
        position_with_aim,
        position_with_aim.horizontal * position_with_aim.depth
    );
}

fn calculate_position(commands_file: impl AsRef<Path>) -> Position {
    let commands = read_commands(commands_file);
    let starting_position = Position::new();
    commands.fold(starting_position, |position, command| {
        position.get_position_after_command(&command)
    })
}

fn calculate_position_with_aim(commands_file: impl AsRef<Path>) -> PositionWithAim {
    let commands = read_commands(commands_file);
    let starting_position = PositionWithAim::new();
    commands.fold(starting_position, |position, command| {
        position.get_position_after_command(&command)
    })
}

fn read_commands(commands_file: impl AsRef<Path>) -> impl Iterator<Item = Command> {
    read_lines(commands_file).map(|line| line.parse::<Command>().expect("Error parsing line"))
}

fn read_lines(path: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let f = File::open(path).expect("Error opening input file");
    let reader = BufReader::new(f);

    reader.lines().map(|line| line.expect("Error reading line"))
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

    fn get_position_after_command(&self, command: &Command) -> Self {
        match command.direction {
            Direction::Forward => Self {
                horizontal: self.horizontal + command.amount,
                depth: self.depth + self.aim * command.amount,
                aim: self.aim,
            },
            Direction::Up => Self {
                horizontal: self.horizontal,
                depth: self.depth,
                // Aim shallower.
                aim: self.aim - command.amount,
            },
            Direction::Down => Self {
                horizontal: self.horizontal,
                depth: self.depth,
                // Aim deeper.
                aim: self.aim + command.amount,
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
    use std::path::{Path, PathBuf};

    fn example_file() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("example.txt")
    }

    #[test]
    fn part_one_example() {
        let position = calculate_position(example_file());
        assert_eq!(position.horizontal, 15);
        assert_eq!(position.depth, 10);
        assert_eq!(position.horizontal * position.depth, 150);
    }

    #[test]
    fn part_two_example() {
        let position = calculate_position_with_aim(example_file());
        assert_eq!(position.horizontal, 15);
        assert_eq!(position.depth, 60);
        assert_eq!(position.horizontal * position.depth, 900);
    }
}
