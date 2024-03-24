use std::str::FromStr;

use aoc_runner_derive::aoc;

#[derive(Debug)]
enum Cubes {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl FromStr for Cubes {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, color) = s.split_once(' ').expect("color count splitting");
        match (count, color) {
            (count, "red") => Ok(Self::Red(count.parse().expect("should be a int"))),
            (count, "green") => Ok(Self::Green(count.parse().expect("should be a int"))),
            (count, "blue") => Ok(Self::Blue(count.parse().expect("should be a int"))),
            _ => Err(format!(
                "Can't parse ({count:?}, {color:?}) into cube counts"
            )),
        }
    }
}

#[aoc(day2, part1, coba)]
pub fn p1(input: &str) -> u32 {
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;
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
                                Cubes::Red(count) => count > max_red,
                                Cubes::Green(count) => count > max_green,
                                Cubes::Blue(count) => count > max_blue,
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
