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

#[aoc(day9, part2)]
fn part2(input: &[Block]) -> usize {
    let mut disk = input.to_owned();

    let count = disk
        .iter()
        .rev()
        .find_map(|x| if x.is_some() { Some(x.unwrap()) } else { None })
        .unwrap_or(0);

    for id in (0..=count).rev() {
        defrag(&mut disk, id);
    }

    checksum(&disk)
}

fn defrag(disk: &mut Vec<Block>, id: usize) {
    let chunks = disk_chunks(&disk).collect::<Vec<Chunk>>();
    let target_chunk = chunks.iter().rev().find(|x| x.block == Some(id)).unwrap();
    if let Some(free_chunk) = chunks
        .iter()
        .find(|x| x.block.is_none() && x.len >= target_chunk.len && x.index < target_chunk.index)
    {
        let free_range = free_chunk.index..free_chunk.index + target_chunk.len;
        let target_range = target_chunk.index..target_chunk.index + target_chunk.len;

        for (i, j) in free_range.zip(target_range) {
            disk.swap(i, j);
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Chunk {
    block: Block,
    len: usize,
    index: usize,
}

impl Chunk {
    fn new(block: Block, len: usize, index: usize) -> Self {
        Self { block, len, index }
    }
}

struct ChunkIterator<'a> {
    idx: usize,
    disk: &'a [Block],
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = Chunk;

    fn next(&mut self) -> Option<Chunk> {
        let block = self.disk.get(self.idx)?;
        let mut i = self.idx + 1;
        while let Some(other_block) = self.disk.get(i) {
            if other_block == block {
                i += 1;
            } else {
                break;
            }
        }

        let chunk = Chunk::new(*block, i - self.idx, self.idx);
        self.idx = i;

        Some(chunk)
    }
}

fn disk_chunks<'a>(disk: &'a [Block]) -> ChunkIterator<'a> {
    ChunkIterator { idx: 0, disk }
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
    fn iterate_chunks() {
        let disk = parse(INPUT);
        let chunks = disk_chunks(&disk).collect::<Vec<Chunk>>();
        // "00...111...2...333.44.5555.6666.777.888899"
        assert_eq!(
            chunks,
            vec![
                Chunk::new(Some(0), 2, 0),
                Chunk::new(None, 3, 2),
                Chunk::new(Some(1), 3, 5),
                Chunk::new(None, 3, 8),
                Chunk::new(Some(2), 1, 11),
                Chunk::new(None, 3, 12),
                Chunk::new(Some(3), 3, 15),
                Chunk::new(None, 1, 18),
                Chunk::new(Some(4), 2, 19),
                Chunk::new(None, 1, 21),
                Chunk::new(Some(5), 4, 22),
                Chunk::new(None, 1, 26),
                Chunk::new(Some(6), 4, 27),
                Chunk::new(None, 1, 31),
                Chunk::new(Some(7), 3, 32),
                Chunk::new(None, 1, 35),
                Chunk::new(Some(8), 4, 36),
                Chunk::new(Some(9), 2, 40),
            ]
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 2858);
    }
}
