use aoc_runner_derive::aoc;

use crate::utils::Grid;
use itertools::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, PartialEq, Clone)]
enum Cell {
    Empty,
    Antenna(char),
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            x if x.is_whitespace() => Cell::Empty,
            '.' => Cell::Empty,
            _ => Cell::Antenna(value),
        }
    }
}

impl From<&str> for Grid<Cell> {
    fn from(value: &str) -> Self {
        let rows: Vec<Vec<Cell>> = value
            .lines()
            .map(|line| line.chars().map(Cell::from).collect())
            .collect();

        if rows.len() == 0 {
            Self::default()
        } else {
            let width = rows.len();
            Self::new(rows.into_iter().flatten().collect(), width)
        }
    }
}

trait Installation {
    fn antennas(&self) -> HashMap<char, Vec<(i32, i32)>>;
}

impl Installation for Grid<Cell> {
    fn antennas(&self) -> HashMap<char, Vec<(i32, i32)>> {
        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                if let Some(Cell::Antenna(freq)) = self.get_xy((x, y)) {
                    let mut positions = antennas.get(&freq).cloned().unwrap_or(vec![]);
                    positions.push((x, y));
                    antennas.insert(freq, positions);
                }
            }
        }

        antennas
    }
}

#[aoc(day8, part1)]
fn part1(input: &str) -> u32 {
    let grid = Grid::from(input);
    let antennas = grid.antennas();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, positions) in antennas {
        let pairs = positions.into_iter().combinations(2);
        for pair in pairs {
            let a = pair[0];
            let b = pair[1];
            let offsets: Vec<(i32, i32)> = vec![(a.0 - b.0, a.1 - b.1), (b.0 - a.0, b.1 - a.1)];
            for (point, offset) in pair.into_iter().zip(offsets) {
                let antinode = (point.0 + offset.0, point.1 + offset.1);
                if grid.get_xy(antinode).is_some() {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len() as u32
}

#[aoc(day8, part2)]
fn part2(input: &str) -> u32 {
    let grid = Grid::from(input);
    let antennas = grid.antennas();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, positions) in antennas {
        let pairs = positions.into_iter().combinations(2);
        for pair in pairs {
            let a = pair[0];
            let b = pair[1];
            let (start_offset, end_offset) = ((a.0 - b.0, a.1 - b.1), (b.0 - a.0, b.1 - a.1));

            antinodes.insert(a);

            let start_slope = Slope::new(a, start_offset, &grid);
            for antinode in start_slope {
                antinodes.insert(antinode);
            }
            let end_slope = Slope::new(a, end_offset, &grid);
            for antinode in end_slope {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len() as u32
}

struct Slope<'a, T: Copy> {
    current: (i32, i32),
    step: (i32, i32),
    grid: &'a Grid<T>,
}

impl<'a, T: Copy> Slope<'a, T> {
    fn new(start: (i32, i32), step: (i32, i32), grid: &'a Grid<T>) -> Self {
        Self {
            current: start,
            step,
            grid,
        }
    }
}

impl<'a, T: Copy> Iterator for Slope<'a, T> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        self.current = (self.current.0 + self.step.0, self.current.1 + self.step.1);
        self.grid.get_xy(self.current).map(|_| self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 34);
    }
}
