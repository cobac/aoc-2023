use aoc_runner_derive::aoc;

struct Schematic<'a>(&'a str);

impl Schematic<'_> {
    fn janky_index(&self, x: usize, y: usize) -> String {
        self.0
            .lines()
            .nth(y)
            .expect("out of bounds: y")
            .chars()
            .nth(x)
            .expect("out of bounds: x")
            .to_string()
    }
}

#[aoc(day3, part1, coba)]
pub fn p1(input: &str) -> u32 {
    let (width, height) = (input.lines().next().unwrap().len(), input.lines().count());
    dbg!(width, height);
    let schematic = Schematic(&input);

    3
}
