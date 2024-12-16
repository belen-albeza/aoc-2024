struct Equation {
    target: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn solvable1(&self) -> bool {
        solve_partial_part1(self.target, &self.numbers, None)
    }

    fn solvable2(&self) -> bool {
        solve_partial_part2(self.target, &self.numbers, None)
    }
}

fn solve_partial_part1(target: u64, numbers: &[u64], acc: Option<u64>) -> bool {
    if acc.unwrap_or(0) > target {
        return false;
    }

    if numbers.len() == 0 {
        return acc.unwrap_or(0) == target;
    }

    solve_partial_part1(target, &numbers[1..], Some(acc.unwrap_or(0) + numbers[0]))
        || solve_partial_part1(target, &numbers[1..], Some(acc.unwrap_or(1) * numbers[0]))
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}

fn solve_partial_part2(target: u64, numbers: &[u64], acc: Option<u64>) -> bool {
    if acc.unwrap_or(0) > target {
        return false;
    }

    if numbers.len() == 0 {
        return acc.unwrap_or(0) == target;
    }

    solve_partial_part2(target, &numbers[1..], Some(acc.unwrap_or(0) + numbers[0]))
        || solve_partial_part2(target, &numbers[1..], Some(acc.unwrap_or(1) * numbers[0]))
        || solve_partial_part2(
            target,
            &numbers[1..],
            Some(concat(acc.unwrap_or(0), numbers[0])),
        )
}

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let chunks = line.split(": ").collect::<Vec<&str>>();
            let target = chunks[0].parse().unwrap();
            let numbers = chunks[1].split(" ").map(|x| x.parse().unwrap()).collect();
            Equation { target, numbers }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> u64 {
    input
        .into_iter()
        .filter_map(|x| if x.solvable1() { Some(x.target) } else { None })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> u64 {
    input
        .into_iter()
        .filter_map(|x| if x.solvable2() { Some(x.target) } else { None })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 11387);
    }
}
