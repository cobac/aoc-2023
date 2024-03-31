use std::{collections::BTreeMap, ops::Deref};

use aoc_runner_derive::aoc;

#[derive(Copy, Clone)]
struct Schematic<'a>(&'a str);

impl<'a> Deref for Schematic<'a> {
    type Target = &'a str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Schematic<'_> {
    // Shouldn't it be possible to return an annotated reference somehow?
    fn owned_index(&self, x: usize, y: usize) -> char {
        self.0
            .lines()
            .nth(y)
            .expect("out of bounds y")
            .chars()
            .nth(x)
            .expect("out of bounds x")
    }
}

fn is_symbol(c: char) -> bool {
    !((c == '.') || c.is_ascii_digit())
}

#[rustfmt::skip]
fn get_neighbour_indexes(x: i32, y: i32, max_x: i32, max_y: i32) -> Vec<(usize, usize)> {
    vec![(x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
         (x - 1, y),                 (x + 1, y),
         (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    ]
    .into_iter()
        .filter(|(x, y)| *x >= 0 && *x < max_x && *y >= 0 && *y < max_y)
        .map(|(x, y)| (x.try_into().unwrap(), y.try_into().unwrap()))
        .collect()
}

fn find_numbers(line: &str) -> Vec<Vec<usize>> {
    line.chars()
        .enumerate()
        // Get x_indixes of all numbers
        // also, where are my monads
        .filter_map(|(x, character)| match (x, character.is_ascii_digit()) {
            (_, false) => None,
            (x, true) => Some(x),
        })
        // Group x_indixes into sub-groups of contiguous values (e.g. [1, 2, 4, 5] => [[1, 2], [4, 5]])
        .fold(vec![], |mut grouped_indixes: Vec<Vec<usize>>, x| {
            // If there is already a group
            if let Some(last_group) = grouped_indixes.last_mut() {
                // If the current index -1 is not == previous index
                if x - 1 != *last_group.last().unwrap_or(&0) {
                    // Add new group
                    grouped_indixes.push(vec![]);
                }
                // Add current value to the last group
                grouped_indixes.last_mut().unwrap().push(x);
            } else {
                // If no group:
                // Add new group with current value
                grouped_indixes.push(vec![x]);
            }
            grouped_indixes
        })
}

#[aoc(day3, part1, coba)]
pub fn p1(input: &str) -> u32 {
    let (width, height) = (input.lines().next().unwrap().len(), input.lines().count());
    let schematic = Schematic(input);

    schematic
        .lines()
        .enumerate()
        .map(|(y, line)| (y, find_numbers(line)))
        // Line of grouped xs into sum of valid numbers per line
        .map(|(y, grouped_xs)| {
            grouped_xs
                .into_iter()
                // Indexes groups of numbers with symbols adjacent
                .filter(move |xs| {
                    xs.iter()
                        // Position has any symbol neighbours
                        .map(|x| {
                            get_neighbour_indexes(*x as i32, y as i32, width as i32, height as i32)
                                .iter()
                                // Specific neighbour is symbol
                                .map(|(neigh_x, neigh_y)| {
                                    is_symbol(schematic.owned_index(*neigh_x, *neigh_y))
                                })
                                .any(|b| b)
                        })
                        .any(|b| b)
                })
                // Parse index into numbers
                .map(move |xs| {
                    xs.iter().fold(0, |acc, x| {
                        acc * 10
                            + schematic
                                .owned_index(*x, y)
                                .to_digit(10)
                                .expect("Wrong indexing, assuming it's number")
                    })
                })
                // Sum all numbers per line, idk why type cannot be inferred
                .sum::<u32>()
        })
        // Sum all lines
        .sum()
}

// Discarded WIP. This attempt was iterating over symbols, trying to collect adjacent numbers
// Should be easier to iterate over numbers and check if there are symbols adjacent instead
pub fn p1_discarded_attempt(input: &str) -> u32 {
    let (width, height) = (input.lines().next().unwrap().len(), input.lines().count());
    let schematic = Schematic(input);
    // need a mask to take into account numbers that might touch multiple symbols
    let mut bitmask = vec![vec![false; width]; height];

    for x in 0..=(width - 1) {
        for y in 0..=(height - 1) {
            if is_symbol(schematic.owned_index(x, y)) {
                let indixes = get_neighbour_indexes(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                    (width as i32) - 1,
                    (height as i32) - 1,
                );
                for (xi, yi) in indixes {
                    bitmask[yi][xi] = true;
                    let mut i = 1;
                    loop {
                        if xi + i < height - 1 && schematic.owned_index(xi + i, yi).is_ascii_digit()
                        {
                            // TODO: Incorrect, need to check to the left too
                            bitmask[yi][xi + i] = true;
                            i += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    let _ = bitmask
        .into_iter()
        .map(|line| {
            line.into_iter()
                .enumerate()
                .filter(|(_, b)| *b)
                .map(|(i, _)| i)
                .fold(vec![], |mut grouped_indixes: Vec<Vec<usize>>, x| {
                    // If there is already a group
                    if let Some(last_group) = grouped_indixes.last_mut() {
                        // If the current index -1 is not == previous index
                        if x - 1 != *last_group.last().unwrap_or(&0) {
                            // Add new group
                            grouped_indixes.push(vec![]);
                        }
                        // Add current value to the last group
                        grouped_indixes.last_mut().unwrap().push(x);
                        // If group present
                    } else {
                        // Add new group with current value
                        grouped_indixes.push(vec![x]);
                    }
                    grouped_indixes
                })
        })
        .inspect(|x| {
            dbg!(x);
        });

    // TODO: Iterate over bitmask to select items of trues, grouping contiguous ones
    // use that to index into the schematic
    // sum them boiis

    todo!();
}

#[aoc(day3, part2, coba)]
pub fn p2(input: &str) -> u32 {
    let (width, height) = (input.lines().next().unwrap().len(), input.lines().count());
    let schematic = Schematic(input);
    // *-position : [nums]
    let mut symbol_numbers_map: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();

    schematic
        .lines()
        .enumerate()
        .map(|(y, line)| (y, find_numbers(line)))
        // Update symbol_numbers_map
        .for_each(|(y, grouped_xs)| {
            grouped_xs
                .into_iter()
                // Indexes groups of numbers with * adjacent
                .for_each(|xs| {
                    // Parse index into numbers
                    let value = xs.iter().fold(0, |acc, x| {
                        acc * 10
                            + schematic
                                .owned_index(*x, y)
                                .to_digit(10)
                                .expect("Wrong indexing, assuming it's number")
                    });
                    xs.iter()
                        // Position has any * neighbours
                        .for_each(|x| {
                            get_neighbour_indexes(*x as i32, y as i32, width as i32, height as i32)
                                .iter()
                                // Specific neighbour is *
                                .for_each(|(neigh_x, neigh_y)| {
                                    // Each value might be added multiple times if it's adjacent to multiple characters of the number
                                    if schematic.owned_index(*neigh_x, *neigh_y) == '*' {
                                        symbol_numbers_map
                                            .entry((*neigh_x, *neigh_y))
                                            .and_modify(|vec| vec.push(value))
                                            .or_insert(vec![value]);
                                    }
                                })
                        })
                })
        });

    symbol_numbers_map
        .iter_mut()
        .map(|(_, vals)| {
            // This assumes no * is adjacent to two identical numbers
            vals.dedup();
            vals
        })
        .filter(|vals| vals.len() == 2)
        .map(|vals| vals.iter().product::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symbol() {
        assert!(!is_symbol('1'));
        assert!(!is_symbol('.'));
        assert!(is_symbol('&'));
        assert!(is_symbol('*'));
        assert!(is_symbol('+'));
        assert!(is_symbol('#'));
        assert!(is_symbol('@'));
        assert!(is_symbol('/'));
    }

    #[ignore]
    #[test]
    fn test_get_neighbour_indexes() {
        let idx = get_neighbour_indexes(2, 2, 10, 10);
        dbg!(&idx);
        assert!(idx == vec![(1, 3), (4, 5)]);
    }
}
