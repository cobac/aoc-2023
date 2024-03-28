use aoc_runner_derive::aoc;

struct Schematic<'a>(&'a str);

impl Schematic<'_> {
    // Shouldn't it be possible to return an annotated reference somehow?
    fn owned_index(&self, x: usize, y: usize) -> Option<char> {
        self.0.lines().nth(y)?.chars().nth(x)
    }
}

fn is_symbol(c: char) -> bool {
    !((c == '.') | c.is_digit(10))
}

fn get_neighbour_indexes(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}


#[aoc(day3, part1, coba)]
pub fn p1(input: &str) -> u32 {
    let (width, height) = (input.lines().next().unwrap().len(), input.lines().count());
    let schematic = Schematic(&input);
    // need a mask to take into accoutn numbers that might touch multiple symbols
    // let bitmask: [bool, (width, height)] = todo!();

    for (x, y) in (0..=width - 1).zip(0..=height - 1) {
        if is_symbol(schematic.owned_index(x, y)) {}
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
