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
    !((c == '.') || c.is_digit(10))
}

#[rustfmt::skip]
fn get_neighbour_indexes(x: i32, y: i32, max_x: i32, max_y: i32) -> Vec<(usize, usize)> {
    vec![(x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
         (x - 1, y),                 (x + 1, y),
         (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    ]
    .into_iter()
    .filter(|(x, y)| *x > 0 && *x < max_x && *y > 0 && *y < max_y)
        .map(|(x, y)| (x.try_into().unwrap(), y.try_into().unwrap()))
        .collect()
}

#[aoc(day3, part1, coba)]
pub fn p1(input: &str) -> u32 {
    let (width, height) = (input.lines().next().unwrap().len(), input.lines().count());
    let schematic = Schematic(&input);
    // need a mask to take into account numbers that might touch multiple symbols
    let mut bitmask = vec![vec![false; width]; height];

    for (x, y) in (0..=width - 1).zip(0..=height - 1) {
        if is_symbol(schematic.owned_index(x, y)) {
            let indixes = get_neighbour_indexes(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                (width as i32) - 1,
                (height as i32) - 1,
            );
            for (xi, yi) in indixes {
                bitmask[yi][xi] = true;
            }
        }
    }

    3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symbol() {
        assert!(is_symbol('1') == false);
        assert!(is_symbol('.') == false);
        assert!(is_symbol('&') == true);
    }
}
