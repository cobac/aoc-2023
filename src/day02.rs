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
        let (count, color) = s.split_once(" ").expect("color count splitting");
        match (count, color) {
            (count, "green") => Ok(Self::Green(count.parse().expect("should be a int"))),
            (count, "red") => Ok(Self::Red(count.parse().expect("should be a int"))),
            (count, "blue") => Ok(Self::Blue(count.parse().expect("should be a int"))),
            _ => Err("Can't parse thing into cube counts".into()),
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
            line.split(": ")
                .last()
                .expect("No Game: str found")
                .split("; ")
                .map(|set| {
                    {
                        set.split(", ").map(|cube| {
                            match cube.parse::<Cubes>().expect("cube no parseable") {
                                Cubes::Green(count) => count > MAX_GREEN,
                                Cubes::Red(count) => count > MAX_RED,
                                Cubes::Blue(count) => count > MAX_BLUE,
                            }
                        })
                    }
                    .any(|x| x)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        p1(input);
    }
}
