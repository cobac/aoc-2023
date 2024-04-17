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

#[aoc(day4, part2, coba)]
pub fn p2(input: &str) -> u32 {
    let no_cards = input.lines().count();
    let mut card_counts: Vec<u32> = vec![1; no_cards];

    input.lines().enumerate().for_each(|(current_card, line)| {
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

        match winning.intersection(&you_have).count() {
            0 => (),
            next_cards => {
                let current_copies = card_counts[current_card];
                card_counts[current_card + 1..current_card + next_cards + 1]
                    .iter_mut()
                    .for_each(|el| {
                        *el += current_copies;
                    });
            }
        };
    });
    card_counts.into_iter().sum::<u32>()
}
