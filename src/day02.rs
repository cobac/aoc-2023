use std::str::FromStr;

use aoc_runner_derive::aoc;

const MAX_RED: u32 = 12;
const MAX_BLUE: u32 = 14;
const MAX_GREEN: u32 = 13;

#[derive(Debug)]
enum Cubes {
    Green(u32),
    Red(u32),
    Blue(u32),
}

impl FromStr for Cubes {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, color) = s.split_once(' ').expect("color count splitting");
        match (count, color) {
            (count, "green") => Ok(Self::Green(count.parse().expect("should be a int"))),
            (count, "red") => Ok(Self::Red(count.parse().expect("should be a int"))),
            (count, "blue") => Ok(Self::Blue(count.parse().expect("should be a int"))),
            _ => Err(format!(
                "Can't parse ({count:?}, {color:?}) into cube counts"
            )),
        }
    }
}

#[aoc(day2, part1, coba)]
pub fn p1(input: &str) -> u32 {
    // yeah whatever split on : split on ; split on ,
    // parse do the thing i really like programming but I don't think i can enjoy this
    // TODO, keep track of the game number so that you can count it
    input
        .lines()
        .map(|line| {
            let (game, sets) = line.split_once(": ").expect("no ':' found");
            let invalid = sets
                .split("; ")
                .map(|set| {
                    set.split(", ")
                        .map(
                            |cube| match cube.parse::<Cubes>().expect("cube no parseable") {
                                Cubes::Green(count) => count > MAX_GREEN,
                                Cubes::Red(count) => count > MAX_RED,
                                Cubes::Blue(count) => count > MAX_BLUE,
                            },
                        )
                        .any(|x| x)
                })
                .any(|x| x);
            (game, invalid)
        })
        .filter(|(_, invalid)| !invalid)
        .map(|(game, _)| {
            game.split_once(' ')
                .unwrap()
                .1
                .parse::<u32>()
                .expect("no int found")
        })
        .sum()
}

}
