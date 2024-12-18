type Point = (i64, i64);
use std::u64;

use itertools::Itertools;
use memoize::memoize;
use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

impl Machine {
    pub fn tokens(&self) -> Option<u64> {
        move_claw(*self, (0, 0), 0, (100, 100))
    }

    pub fn tokens_part2(&self) -> Option<u64> {
        let ax = self.a.0;
        let ay = self.a.1;
        let bx = self.b.0;
        let by = self.b.1;
        let px = self.prize.0;
        let py = self.prize.1;

        // a*Ax = Px - b*Bx;
        // a*Ay = Py - b*By;
        if (py * ax - ay * px) % (by * ax - ay * bx) == 0 {
            let tokens_b = (py * ax - ay * px) / (by * ax - ay * bx);
            if (px - tokens_b * bx) % ax == 0 {
                let tokens_a = (px - tokens_b * bx) / ax;
                return Some((tokens_a * 3 + tokens_b) as u64);
            }
        }

        None
    }
}

#[memoize]
fn move_claw(machine: Machine, position: Point, tokens: u64, remaining: (u64, u64)) -> Option<u64> {
    if position == machine.prize {
        return Some(tokens);
    }
    if position.0 > machine.prize.0 || position.1 > machine.prize.1 {
        return None;
    }

    let res_a = if remaining.0 > 0 {
        move_claw(
            machine,
            (position.0 + machine.a.0, position.1 + machine.a.1),
            tokens + 3,
            (remaining.0 - 1, remaining.1),
        )
    } else {
        None
    };

    let res_b = if remaining.1 > 0 {
        move_claw(
            machine,
            (position.0 + machine.b.0, position.1 + machine.b.1),
            tokens + 1,
            (remaining.0, remaining.1 - 1),
        )
    } else {
        None
    };

    if let Some(tokens_a) = res_a {
        Some(tokens_a.min(res_b.unwrap_or(u64::MAX)))
    } else {
        res_b
    }
}

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Machine> {
    let re = Regex::new(r"\d+").unwrap();

    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let rows: Vec<Point> = chunk
                .map(|line| {
                    let numbers = re
                        .find_iter(line)
                        .map(|matched| matched.as_str().parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();

                    (numbers[0], numbers[1])
                })
                .collect();
            Machine {
                a: rows[0],
                b: rows[1],
                prize: rows[2],
            }
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[Machine]) -> u64 {
    input.into_iter().filter_map(|m| m.tokens()).sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Machine]) -> u64 {
    let inc = 10_000_000_000_000;
    input
        .into_iter()
        .map(|m| Machine {
            a: m.a,
            b: m.b,
            prize: (m.prize.0 + inc, m.prize.1 + inc),
        })
        .filter_map(|m| m.tokens_part2())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn parse_input() {
        let expected = vec![
            Machine {
                a: (94, 34),
                b: (22, 67),
                prize: (8400, 5400),
            },
            Machine {
                a: (26, 66),
                b: (67, 21),
                prize: (12748, 12176),
            },
            Machine {
                a: (17, 86),
                b: (84, 37),
                prize: (7870, 6450),
            },
            Machine {
                a: (69, 23),
                b: (27, 71),
                prize: (18641, 10279),
            },
        ];

        assert_eq!(parse(INPUT), expected);
    }

    #[test]
    fn tokens_one_movement() {
        let machine = Machine {
            a: (1, 1),
            b: (2, 1),
            prize: (2, 1),
        };
        assert_eq!(machine.tokens(), Some(1));
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 480);
    }

    #[test]
    fn part2() {
        let machine = Machine {
            a: (94, 34),
            b: (22, 67),
            prize: (8400, 5400),
        };

        assert_eq!(machine.tokens_part2(), Some(280));
    }
}
