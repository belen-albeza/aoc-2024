type Block = Option<usize>;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Block> {
    input
        .chars()
        .map(|x| x.to_string().parse::<usize>().unwrap())
        .enumerate()
        .map(|(i, x)| {
            let block = if i % 2 == 0 { Some(i / 2) } else { None };
            vec![block; x as usize]
        })
        .flatten()
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Block]) -> usize {
    let mut disk = input.to_owned();

    for i in (0..disk.len()).rev() {
        for j in 0..i {
            if disk[j].is_none() {
                disk[j] = disk[i];
                disk[i] = None;
                break;
            }
        }
    }

    checksum(&disk)
}

fn checksum(disk: &[Block]) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, x)| i * x.unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn parse_input() {
        let expected = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .map(|x| x.to_string().parse().ok())
            .collect::<Vec<Block>>();

        assert_eq!(parse(INPUT), expected)
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 1928);
    }
}
