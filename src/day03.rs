use aoc_runner_derive::aoc;

struct Schematic<'a>(&'a str);

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

#[aoc(day3, part1, coba)]
pub fn p1(input: &str) -> u32 {
    let (width, height) = (input.lines().next().unwrap().len(), input.lines().count());
    let schematic = Schematic(input);
    todo!()
}

// Discarded WIP, should be easier to iterate over numbers,
// and then check if they touch a symbol instead of iterating over symbols and checking numbers
pub fn p1_old(input: &str) -> u32 {
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

    bitmask
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
