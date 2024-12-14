use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
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

    fn parse(&self) -> Vec<Op> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        }

        RE.captures_iter(&self.src)
            .map(|c| c.extract())
            .map(|(_, [a, b])| Op::Mul(a.parse().unwrap(), b.parse().unwrap()))
            .collect()
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Op> {
    Parser::new(input.to_owned()).parse()
}

#[aoc(day3, part1)]
fn part1(input: &[Op]) -> u32 {
    input
        .into_iter()
        .map(|op| match op {
            Op::Mul(a, b) => a * b,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_parse_input() {
        let res = Parser::new(INPUT.to_owned()).parse();
        assert_eq!(
            res,
            vec![Op::Mul(2, 4), Op::Mul(5, 5), Op::Mul(11, 8), Op::Mul(8, 5)]
        )
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 161);
    }
}
