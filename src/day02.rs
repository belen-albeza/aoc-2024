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

#[aoc(day2, part1)]
fn part1(input: &[Report]) -> u32 {
    input
        .iter()
        .filter(|report| is_report_valid(*report))
        .count() as u32
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
}
