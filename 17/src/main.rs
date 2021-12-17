// Day 17: Trick Shot
// https://adventofcode.com/2021/day/17

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]
#![allow(clippy::similar_names)]

use std::cmp;
use std::ops::RangeInclusive;

fn main() {
    println!(
        "Part 1: Highest Y position to reach target = {:?}",
        max_height_that_reaches_target(&Target {
            x: 209..=238,
            y: -86..=-59
        })
    );
    println!(
        "Part 2: Total distinct velocities that reach target = {}",
        total_velocities_that_reach_target(&Target {
            x: 209..=238,
            y: -86..=-59
        })
    );
}

fn max_height_that_reaches_target(target: &Target) -> Option<i64> {
    max_height_of_shots_that_reach_target(target).next()
}

fn total_velocities_that_reach_target(target: &Target) -> usize {
    max_height_of_shots_that_reach_target(target).count()
}

fn max_height_of_shots_that_reach_target(target: &Target) -> impl Iterator<Item = i64> + '_ {
    let min_y_velocity = *target.y.start();
    let max_y_velocity = -*target.y.start();
    let min_x_velocity = 0;
    let max_x_velocity = *target.x.end();

    (min_y_velocity..=max_y_velocity)
        // In descending order of Y velocity, so the highest height shots are first.
        .rev()
        .flat_map(move |y_velocity| {
            (min_x_velocity..=max_x_velocity)
                .filter_map(move |x_velocity| try_shot(target, x_velocity, y_velocity))
        })
}

fn try_shot(target: &Target, mut x_velocity: u64, mut y_velocity: i64) -> Option<i64> {
    let mut x_position = 0;
    let mut y_position = 0;
    let mut max_y_position = 0;

    loop {
        x_position += x_velocity;
        y_position += y_velocity;
        // X velocity decreases by 1 each time, until it reaches zero.
        x_velocity = x_velocity.saturating_sub(1);
        y_velocity -= 1;
        max_y_position = cmp::max(max_y_position, y_position);

        if target.x.contains(&x_position) && target.y.contains(&y_position) {
            // Inside the target.
            return Some(max_y_position);
        } else if x_position > *target.x.end() || y_position < *target.y.start() {
            // We've overshot the target, so no point performing more steps.
            return None;
        }
    }
}

struct Target {
    x: RangeInclusive<u64>,
    y: RangeInclusive<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_shot() {
        assert!(try_shot(
            &Target {
                x: 20..=30,
                y: -10..=-5
            },
            7,
            2
        )
        .is_some());
        assert!(try_shot(
            &Target {
                x: 20..=30,
                y: -10..=-5
            },
            6,
            3
        )
        .is_some());
        assert!(try_shot(
            &Target {
                x: 20..=30,
                y: -10..=-5
            },
            9,
            0
        )
        .is_some());
        assert!(try_shot(
            &Target {
                x: 20..=30,
                y: -10..=-5
            },
            17,
            4
        )
        .is_none());
        assert_eq!(
            try_shot(
                &Target {
                    x: 20..=30,
                    y: -10..=-5
                },
                6,
                9
            ),
            Some(45)
        );
    }

    #[test]
    fn test_max_height_that_reaches_target() {
        assert_eq!(
            max_height_that_reaches_target(&Target {
                x: 20..=30,
                y: -10..=-5
            }),
            Some(45)
        );
    }

    #[test]
    fn test_total_velocities_that_reach_target() {
        assert_eq!(
            total_velocities_that_reach_target(&Target {
                x: 20..=30,
                y: -10..=-5
            }),
            112
        );
    }
}
