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

#[aoc(day1, part2, coba)]
pub fn p2(input: &str) -> u32 {
    // This approach doesn't work coz e.g. "eightwothree" needs to be "8wo3"
    let v: Vec<_> = input
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
        .lines()
        .map(|line| {
            line.chars()
                .filter(|char| char.is_numeric())
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    // println!("{v:?}");
    v.into_iter()
        .map(|x| x.first().unwrap() * 10 + x.last().unwrap())
        .sum()
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
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(p2(input), 281);
    }
}
