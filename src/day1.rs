use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
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
        assert_eq!(
            p1("1
2
3
4
"),
            11 + 22 + 33 + 44
        )
    }
}
