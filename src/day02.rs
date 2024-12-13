type Report = Vec<u32>;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_valid(report: &Report) -> bool {
    if report.len() < 2 {
        return true;
    }

    let is_asc = report[0] < report[1];

    for pair in report.windows(2) {
        let diff = pair[0].abs_diff(pair[1]);
        if diff > 3 || diff < 1 {
            return false;
        }

        if (is_asc && pair[0] > pair[1]) || (!is_asc && pair[0] < pair[1]) {
            return false;
        }
    }

    true
}

fn is_report_valid_with_dampening(report: &Report) -> bool {
    let res = is_report_valid(report);

    if res {
        return true;
    }

    for x in 0..report.len() {
        let mut dampened = report.clone();
        dampened.remove(x);

        if is_report_valid(&dampened) {
            return true;
        }
    }

    false
}

#[aoc(day2, part1)]
fn part1(input: &[Report]) -> u32 {
    input
        .iter()
        .filter(|report| is_report_valid(*report))
        .count() as u32
}

#[aoc(day2, part2)]
fn part2(input: &[Report]) -> u32 {
    input
        .iter()
        .filter(|report| is_report_valid_with_dampening(*report))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 4);
    }
}
