use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Enable,
    Disable,
    Mul(u32, u32),
}

#[derive(Debug)]
struct Parser {
    src: String,
}

impl Parser {
    fn new(input: String) -> Self {
        Self { src: input }
    }

    fn parse_part1(&self) -> Vec<Op> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        }

        RE.captures_iter(&self.src)
            .map(|c| c.extract())
            .map(|(_, [a, b])| Op::Mul(a.parse().unwrap(), b.parse().unwrap()))
            .collect()
    }

    fn parse_part2(&self) -> Vec<Op> {
        lazy_static! {
            static ref RE_MUL: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
            static ref RE_DO: Regex = Regex::new(r"do\(\)").unwrap();
            static ref RE_DONT: Regex = Regex::new(r"don\'t\(\)").unwrap();
        }

        let muls: Vec<(usize, Op)> = RE_MUL
            .captures_iter(&self.src)
            .map(|c| (c.get(0).unwrap().start(), c.extract()))
            .map(|(index, (_, [a, b]))| {
                let op = Op::Mul(a.parse().unwrap(), b.parse().unwrap());
                (index, op)
            })
            .collect();
        let dos: Vec<(usize, Op)> = RE_DO
            .captures_iter(&self.src)
            .map(|c| {
                let index = c.get(0).unwrap().start();
                (index, Op::Enable)
            })
            .collect();

        let donts: Vec<(usize, Op)> = RE_DONT
            .captures_iter(&self.src)
            .map(|c| {
                let index = c.get(0).unwrap().start();
                (index, Op::Disable)
            })
            .collect();

        let mut all = vec![muls, dos, donts].concat();
        all.sort_by_key(|(index, _)| *index);

        all.into_iter().map(|(_, op)| op).collect()
    }
}

#[aoc_generator(day3, part1)]
fn parse_part1(input: &str) -> Vec<Op> {
    Parser::new(input.to_owned()).parse_part1()
}

#[aoc_generator(day3, part2)]
fn parse_part2(input: &str) -> Vec<Op> {
    Parser::new(input.to_owned()).parse_part2()
}

#[aoc(day3, part1)]
fn part1(input: &[Op]) -> u32 {
    input
        .into_iter()
        .map(|op| match op {
            Op::Mul(a, b) => a * b,
            _ => unreachable!("not available on part 1"),
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Op]) -> u32 {
    let mut res = 0;
    let mut enabled = true;

    for &op in input {
        match op {
            Op::Enable => {
                enabled = true;
            }
            Op::Mul(a, b) if enabled => res += a * b,
            Op::Disable => {
                enabled = false;
            }
            _ => {}
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_parse_input() {
        let res = Parser::new(INPUT1.to_owned()).parse_part1();
        assert_eq!(
            res,
            vec![Op::Mul(2, 4), Op::Mul(5, 5), Op::Mul(11, 8), Op::Mul(8, 5)]
        )
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(INPUT1)), 161);
    }

    #[test]
    fn part2_parse_input() {
        let res = Parser::new(INPUT2.to_owned()).parse_part2();
        assert_eq!(
            res,
            vec![
                Op::Mul(2, 4),
                Op::Disable,
                Op::Mul(5, 5),
                Op::Mul(11, 8),
                Op::Enable,
                Op::Mul(8, 5)
            ]
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(INPUT2)), 48);
    }
}
