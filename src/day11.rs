use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    input.split(" ").map(|x| x.parse().unwrap()).collect()
}

#[aoc(day11, part1)]
fn part1(input: &[u64]) -> u64 {
    let mut stones = input.to_owned();

    for _ in 0..25 {
        stones = blink(stones);
    }

    stones.len() as u64
}

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut output = stones.clone();

    let mut i = 0;
    for stone in stones {
        match stone {
            0 => {
                output[i] = 1;
            }
            x if digits(x).len() % 2 == 0 => {
                let (a, b) = split_number(x);
                output.insert(i, a);
                i += 1;
                output[i] = b;
            }
            _ => {
                output[i] = output[i] * 2024;
            }
        }

        i += 1;
    }

    output
}

fn digits(x: u64) -> Vec<u64> {
    x.to_string()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect()
}

fn split_number(x: u64) -> (u64, u64) {
    let digits = x.to_string();
    let (a, b) = (&digits[..digits.len() / 2], &digits[digits.len() / 2..]);

    (a.parse().unwrap(), b.parse().unwrap())
}

#[aoc(day11, part2)]
fn part2(input: &[u64]) -> u64 {
    input.into_iter().map(|&x| memoized_blink(x, 75)).sum()
}

#[memoize]
fn memoized_blink(stone: u64, count: usize) -> u64 {
    // NOTE: we need to return the *amount* of stones, so it's a matter of
    // counting how many times we split before count is zero.
    if count == 0 {
        return 1;
    }

    match stone {
        0 => memoized_blink(1, count - 1),
        x if digits(x).len() % 2 == 0 => {
            let (a, b) = split_number(x);
            memoized_blink(a, count - 1) + memoized_blink(b, count - 1)
        }
        _ => memoized_blink(stone * 2024, count - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 55312);
    }
}
