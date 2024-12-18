use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;
use itertools::Itertools;

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
    let grid: Grid<Cell> = Grid::from(input);
    let regions = build_all_regions(&grid);

    regions
        .into_iter()
        .map(|region| {
            let perimeter = region.values().map(|&n| (4 - n) as u64).sum::<u64>();
            let area = region.keys().count() as u64;

            area * perimeter
        })
        .sum()
}

fn build_all_regions(grid: &Grid<Cell>) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];
    let mut classified: HashSet<Point> = HashSet::new();

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

fn side_count(region: &Region) -> u64 {
    let raw_sides = find_sides(region);

    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    dirs.into_iter()
        .map(|d| count_sides_in_dir(&raw_sides, d))
        .sum()
}

fn find_sides(region: &Region) -> Vec<(Point, Point)> {
    let cells: Vec<Point> = region.keys().copied().collect();

    let offsets = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut sides: Vec<(Point, Point)> = vec![];

    for cell in cells.iter() {
        let cell_sides = offsets.iter().filter_map(|offset| {
            let candidate = (cell.0 + offset.0, cell.1 + offset.1);
            if !cells.contains(&candidate) {
                Some((*cell, *offset))
            } else {
                None
            }
        });

        sides.extend(cell_sides);
    }

    sides
}

fn count_sides_in_dir(all_sides: &[(Point, Point)], target_dir: Point) -> u64 {
    let grouped_sides: Vec<Vec<(Point, Point)>> = all_sides
        .iter()
        .filter(|(_, dir)| *dir == target_dir)
        .copied()
        .sorted_by_key(|((x, y), _)| if is_column(target_dir) { *x } else { *y })
        .chunk_by(|((x, y), _)| if is_column(target_dir) { *x } else { *y })
        .into_iter()
        .map(|(_, group)| group.collect())
        .collect();

    let mut gaps = 0;
    for row in grouped_sides {
        let sorted_row: Vec<(Point, Point)> = row
            .into_iter()
            .sorted_by_key(|((x, y), _)| if is_column(target_dir) { *y } else { *x })
            .collect();
        gaps += 1;
        for pair in sorted_row.windows(2) {
            if pair.len() == 1 {
                continue;
            }

            let (a, _) = pair[0];
            let (b, _) = pair[1];

            let diff = (a.0 - b.0, a.1 - b.1);
            if (diff.0).abs() > 1 || (diff.1).abs() > 1 {
                gaps += 1;
            }
        }
    }

    gaps
}

fn is_column((_, y): Point) -> bool {
    y == 0
}

#[aoc(day12, part2)]
fn part2(input: &str) -> u64 {
    let grid = Grid::from(input);
    let regions = build_all_regions(&grid);

    regions
        .into_iter()
        .map(|region| {
            let sides = side_count(&region);
            let area = region.keys().count() as u64;

            sides * area
        })
        .sum()
}

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

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1206);
    }

    #[test]
    fn side_count_single_cell() {
        let mut region: Region = HashMap::new();
        region.insert((0, 0), 0);

        assert_eq!(side_count(&region), 4);
    }
    #[test]
    fn side_count_rect() {
        let mut region: Region = HashMap::new();
        region.insert((0, 0), 0);
        region.insert((0, 1), 0);
        region.insert((0, 2), 0);

        assert_eq!(side_count(&region), 4);
    }

    #[test]
    fn count_tetris_t() {
        let mut region: Region = HashMap::new();
        region.insert((1, 0), 0);
        region.insert((0, 1), 0);
        region.insert((1, 1), 0);
        region.insert((2, 1), 0);

        assert_eq!(side_count(&region), 8);
    }
}
