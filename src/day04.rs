use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day4, part1, coba)]
pub fn p1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (winning, you_have) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let winning: HashSet<_> = winning
                .split(' ')
                // Deal with extra whitespaces
                .filter_map(|n| n.parse::<u32>().ok())
                .collect();
            let you_have: HashSet<_> = you_have
                .split(' ')
                // Deal with extra whitespaces
                .filter_map(|n| n.parse::<u32>().ok())
                .collect();

            match winning.intersection(&you_have).count().try_into().unwrap() {
                0 => 0,
                x => 2_u32.pow(x - 1),
            }
        })
        .sum()
}
