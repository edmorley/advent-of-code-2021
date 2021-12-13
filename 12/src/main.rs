// Day 12: Passage Pathing
// https://adventofcode.com/2021/day/12

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::collections::{HashMap, HashSet};
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
    let graph = read_graph(&path);
    println!(
        "Part 1: Total paths (visiting small caves only once) = {}",
        total_paths(&graph, true)
    );
    println!(
        "Part 2: Total paths (allowing one small cave to be revisited again) = {}",
        total_paths(&graph, false)
    );
}

fn read_graph(path: impl AsRef<Path>) -> HashMap<String, Vec<String>> {
    let f = File::open(path).expect("Error opening input file");
    let edges = BufReader::new(f).lines().map(|line| {
        let line = line.expect("Error reading line");
        let (start_node, end_node) = line.split_once('-').expect("Error parsing line");
        (start_node.to_string(), end_node.to_string())
    });
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for (start_node, end_node) in edges {
        let start_node_neighbours = graph.entry(start_node.clone()).or_default();
        start_node_neighbours.push(end_node.clone());
        let end_node_neighbours = graph.entry(end_node).or_default();
        end_node_neighbours.push(start_node);
    }
    graph
}

fn total_paths(graph: &HashMap<String, Vec<String>>, only_visit_small_caves_once: bool) -> u64 {
    let mut unexplored_routes = vec![Route {
        current_node: "start",
        visited: HashSet::new(),
        small_cave_visited_twice: false,
    }];
    let mut completed_routes = 0;

    while let Some(mut route) = unexplored_routes.pop() {
        if route.current_node == "end" {
            completed_routes += 1;
            continue;
        }

        route.visited.insert(route.current_node);

        if let Some(adjacent_nodes) = graph.get(route.current_node) {
            for adjacent_node in adjacent_nodes {
                let mut small_cave_visited_twice = route.small_cave_visited_twice;
                if is_small_cave(adjacent_node) && route.visited.contains(adjacent_node.as_str()) {
                    if adjacent_node == "start"
                        || only_visit_small_caves_once
                        || small_cave_visited_twice
                    {
                        continue;
                    }
                    small_cave_visited_twice = true;
                }
                unexplored_routes.push(Route {
                    current_node: adjacent_node,
                    visited: route.visited.clone(),
                    small_cave_visited_twice,
                });
            }
        }
    }

    completed_routes
}

fn is_small_cave(name: &str) -> bool {
    name == name.to_lowercase()
}

struct Route<'a> {
    current_node: &'a str,
    visited: HashSet<&'a str>,
    small_cave_visited_twice: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn example_file(example_number: u8) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("example{}.txt", example_number))
    }

    #[test]
    fn part_one_example1() {
        let graph = read_graph(&example_file(1));
        assert_eq!(total_paths(&graph, true), 10);
    }

    #[test]
    fn part_one_example2() {
        let graph = read_graph(&example_file(2));
        assert_eq!(total_paths(&graph, true), 19);
    }

    #[test]
    fn part_one_example3() {
        let graph = read_graph(&example_file(3));
        assert_eq!(total_paths(&graph, true), 226);
    }

    #[test]
    fn part_two_example1() {
        let graph = read_graph(&example_file(1));
        assert_eq!(total_paths(&graph, false), 36);
    }

    #[test]
    fn part_two_example2() {
        let graph = read_graph(&example_file(2));
        assert_eq!(total_paths(&graph, false), 103);
    }

    #[test]
    fn part_two_example3() {
        let graph = read_graph(&example_file(3));
        assert_eq!(total_paths(&graph, false), 3509);
    }
}
