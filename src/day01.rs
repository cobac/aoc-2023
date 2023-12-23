use aoc_runner_derive::aoc;

#[aoc(day1, part1, coba)]
pub fn p1(input: &str) -> u32 {
    let v: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|char| char.is_numeric())
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    // println!("{v:?}");

    v.iter()
        .map(|x| x.first().unwrap() * 10 + x.last().unwrap())
        .sum()
}

#[aoc(day1, part1, internet)]
pub fn p1_(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut values = line.chars().filter_map(|char| char.to_digit(10));
            let first = values.next().expect("expecting at least one number");
            let last = values.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

#[aoc(day1, part2, coba)]
pub fn p2(input: &str) -> u32 {
    // This approach doesn't work coz e.g. "eightwothree" needs to be "8wo3" and "1oneight" needs to be "18"
    // let v: Vec<_> = input
    //     .replace("one", "1")
    //     .replace("two", "2")
    //     .replace("three", "3")
    //     .replace("four", "4")
    //     .replace("five", "5")
    //     .replace("six", "6")
    //     .replace("seven", "7")
    //     .replace("eight", "8")
    //     .replace("nine", "9")
    //     .lines()
    //     .map(|line| {
    //         line.chars()
    //             .filter(|char| char.is_numeric())
    //             .map(|x| x.to_digit(10).unwrap())
    //             .collect::<Vec<u32>>()
    //     })
    //     .collect();

    input.lines().map(parse_lines).sum()
}

fn parse_lines(line: &str) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .zip(1..);

    let first_num = 'out: {
        for i in 0..line.len() {
            // better to clone iterator twice than allocating full thing once?
            for (word, num) in numbers.clone() {
                if line[i..].starts_with(word) {
                    break 'out num as u32;
                } else {
                    let c = line.chars().nth(i).unwrap();
                    if c.is_ascii_digit() {
                        break 'out c.to_digit(10).unwrap();
                    }
                };
            }
        }
        0 // should never call this is horrible
    };

    let last_num = 'out: {
        for i in (0..line.len()).rev() {
            for (word, num) in numbers.clone() {
                if line[i..].starts_with(word) {
                    break 'out num as u32;
                } else {
                    let c = line.chars().nth(i).unwrap();
                    if c.is_ascii_digit() {
                        break 'out c.to_digit(10).unwrap();
                    }
                };
            }
        }
        first_num // should never call this is horrible
    };
    first_num * 10 + last_num
}

#[aoc(day1, part2, internet)]
pub fn p2_(input: &str) -> u32 {
    input.lines().map(parse_lines_).sum()
}

fn parse_lines_(line: &str) -> u32 {
    let mut values = (0..line.len())
        .map(|index| {
            let reduced_line = &line[index..];
            let out = if reduced_line.starts_with("one") {
                '1'
            } else if reduced_line.starts_with("two") {
                '2'
            } else if reduced_line.starts_with("three") {
                '3'
            } else if reduced_line.starts_with("four") {
                '4'
            } else if reduced_line.starts_with("five") {
                '5'
            } else if reduced_line.starts_with("six") {
                '6'
            } else if reduced_line.starts_with("seven") {
                '7'
            } else if reduced_line.starts_with("eight") {
                '8'
            } else if reduced_line.starts_with("nine") {
                '9'
            } else {
                reduced_line.chars().next().unwrap()
            };
            out
        })
        .filter_map(|char| char.to_digit(10));

    let first = values.next().expect("expecting at least one number");
    let last = values.last().unwrap_or(first);
    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(p1(input), 142);
    }

    #[test]
    fn p2_test() {
        let input = "two1nine
eightwo3three
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(p2(input), 29 + 83 + 13 + 24 + 42 + 14 + 76);
    }
}
