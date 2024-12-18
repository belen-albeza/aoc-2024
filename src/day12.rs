use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

use crate::utils::Grid;

type Cell = char;

impl From<&str> for Grid<Cell> {
    fn from(value: &str) -> Self {
        let rows: Vec<Vec<Cell>> = value.lines().map(|line| line.chars().collect()).collect();

        if rows.len() == 0 {
            Self::default()
        } else {
            let width = rows.len();
            Self::new(rows.into_iter().flatten().collect(), width)
        }
    }
}

type Point = (i32, i32);
type Region = HashMap<Point, usize>;

#[aoc(day12, part1)]
fn part1(input: &str) -> u64 {
    let mut regions: Vec<Region> = vec![];
    let mut classified: HashSet<Point> = HashSet::new();
    let grid: Grid<Cell> = Grid::from(input);

    for y in 0..grid.height() as i32 {
        for x in 0..grid.width() as i32 {
            if classified.contains(&(x, y)) {
                continue;
            }

            let region = build_region((x, y), &grid, &mut HashMap::new());
            for key in region.keys() {
                classified.insert(*key);
            }

            regions.push(region);
        }
    }

    regions
        .into_iter()
        .map(|region| {
            let perimeter = region.values().map(|&n| (4 - n) as u64).sum::<u64>();
            let area = region.keys().count() as u64;

            area * perimeter
        })
        .sum()
}

fn build_region(xy: Point, grid: &Grid<Cell>, region: &mut Region) -> Region {
    let id = grid.get_xy(xy).unwrap();

    let neighbors: Vec<(Point, Cell)> = grid
        .neighbors4_xy(xy)
        .into_iter()
        .filter(|&(_, other_id)| other_id == id)
        .collect();

    region.insert(xy, neighbors.len());

    for (other_xy, _) in neighbors {
        if region.contains_key(&other_xy) {
            continue;
        }

        let partial = build_region(other_xy, &grid, region);
        region.extend(partial);
    }

    region.clone()
}

// #[aoc(day12, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 1930);
    }
}
