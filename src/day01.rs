use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|x| {
            let nums = x
                .split_whitespace()
                .map(|raw_id| raw_id.parse().unwrap())
                .collect::<Vec<u32>>();
            (nums[0], nums[1])
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[(u32, u32)]) -> u32 {
    let mut list_a = input.into_iter().map(|(x, _)| *x).collect::<Vec<u32>>();
    let mut list_b = input.into_iter().map(|(_, y)| *y).collect::<Vec<u32>>();

    list_a.sort();
    list_b.sort();

    list_a.iter().zip(list_b).map(|(a, b)| a.abs_diff(b)).sum()
}

#[aoc(day1, part2)]
fn part2(input: &[(u32, u32)]) -> u32 {
    let list_a = input.into_iter().map(|(x, _)| *x).collect::<Vec<u32>>();
    let list_b = input.into_iter().map(|(_, y)| *y).collect::<Vec<u32>>();

    list_a
        .into_iter()
        .map(|x| {
            let count = list_b.iter().filter(|&&y| x == y).count();
            x * count as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 31);
    }
}
