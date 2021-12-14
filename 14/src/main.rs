// Day 14: Extended Polymerization
// https://adventofcode.com/2021/day/14

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::path::Path;
use std::process;
use std::str::FromStr;
use std::{env, fs};

use counter::Counter;
use itertools::Itertools;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let instructions = read_instructions(&path);
    println!(
        "Part 1: Quantity of most common minus least common, after 10 steps = {}",
        calculate_result(&instructions, 10)
    );
    println!(
        "Part 2: Quantity of most common minus least common, after 40 steps = {}",
        calculate_result(&instructions, 40)
    );
}

fn read_instructions(path: impl AsRef<Path>) -> Instructions {
    let file_contents = fs::read_to_string(path).expect("Error reading input file");
    file_contents
        .parse::<Instructions>()
        .expect("Error parsing input file")
}

fn calculate_result(instructions: &Instructions, steps: u8) -> usize {
    let mut template_pairs = instructions.template_pairs.clone();

    for _ in 1..=steps {
        template_pairs = perform_step(&template_pairs, &instructions.rules);
    }

    let element_counts =
        template_pairs
            .iter()
            .fold(HashMap::new(), |mut counts, ((element_a, _), count)| {
                *counts.entry(element_a).or_insert(0) += count;
                counts
            });

    let most_common_quantity = element_counts.values().max().unwrap_or(&0);
    let least_common_quantity = element_counts.values().min().unwrap_or(&0);

    most_common_quantity - least_common_quantity
}

fn perform_step(template_pairs: &TemplatePairs, rules: &Rules) -> TemplatePairs {
    let mut new_template_pairs = HashMap::new();

    for ((element_a, element_b), count) in template_pairs {
        if let Some(element_to_insert) = rules.get(&(*element_a, *element_b)) {
            *new_template_pairs
                .entry((*element_a, Some(*element_to_insert)))
                .or_insert(0) += *count;
            *new_template_pairs
                .entry((Some(*element_to_insert), *element_b))
                .or_insert(0) += *count;
        } else {
            *new_template_pairs
                .entry((*element_a, *element_b))
                .or_insert(0) += *count;
        }
    }

    new_template_pairs
}

type TemplatePairs = HashMap<(Option<char>, Option<char>), usize>;
type Rules = HashMap<(Option<char>, Option<char>), char>;

struct Instructions {
    template_pairs: TemplatePairs,
    rules: Rules,
}

impl FromStr for Instructions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split_terminator("\n\n");

        let template = sections
            .next()
            .ok_or_else(|| String::from("Missing template"))?;
        let mut template_pairs = template
            .chars()
            .tuple_windows::<(_, _)>()
            .map(|(element_a, element_b)| (Some(element_a), Some(element_b)))
            .collect::<Counter<_>>()
            .into_map();
        // We need to add the last character on to ensure the counts are correct later,
        // since tuple_windows() stops when there are no more complete pairs.
        let last_character = template.chars().last();
        template_pairs.insert((last_character, None), 1);

        let rules = sections
            .next()
            .ok_or_else(|| String::from("Missing rules"))?
            .lines()
            .map(|rule| {
                rule.split_once(" -> ")
                    .and_then(|(element_pair, b)| {
                        let (element_a, element_b) = element_pair.chars().collect_tuple()?;
                        let element_to_insert = b.chars().next()?;
                        Some(((Some(element_a), Some(element_b)), element_to_insert))
                    })
                    .ok_or_else(|| format!("Invalid rule: {}", rule))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            template_pairs,
            rules,
        })
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
        let instructions = read_instructions(&example_file());
        assert_eq!(calculate_result(&instructions, 10), 1588);
    }

    #[test]
    fn part_two_example() {
        let instructions = read_instructions(&example_file());
        assert_eq!(calculate_result(&instructions, 40), 2_188_189_693_529);
    }
}
