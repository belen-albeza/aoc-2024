use aoc_runner_derive::aoc;

use crate::utils::Grid;

impl From<&str> for Grid<usize> {
    fn from(value: &str) -> Self {
        let rows: Vec<Vec<usize>> = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| x.to_string().parse().unwrap())
                    .collect()
            })
            .collect();

        if rows.len() == 0 {
            Self::default()
        } else {
            let width = rows.len();
            Self::new(rows.into_iter().flatten().collect(), width)
        }
    }
}

trait Map {
    fn start_positions(&self) -> impl Iterator<Item = (i32, i32)>;
    fn targets(&self) -> impl Iterator<Item = (i32, i32)>;
}

impl Map for Grid<usize> {
    fn start_positions(&self) -> impl Iterator<Item = (i32, i32)> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x == 0 { self.xy_for(i) } else { None })
    }

    fn targets(&self) -> impl Iterator<Item = (i32, i32)> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x == 9 { self.xy_for(i) } else { None })
    }
}

type Route = Vec<(i32, i32)>;
pub struct Trailhead<'a> {
    position: (i32, i32),
    grid: &'a Grid<usize>,
}

impl<'a> Trailhead<'a> {
    pub fn new(position: (i32, i32), grid: &'a Grid<usize>) -> Self {
        Self { position, grid }
    }

    pub fn score_part1(&mut self) -> u32 {
        self.grid
            .targets()
            .filter_map(|target| self.find_path(self.position, target, &mut Vec::new()))
            .count() as u32
    }

    pub fn score_part2(&mut self) -> u32 {
        self.grid
            .targets()
            .map(|target| {
                self.find_all_paths(self.position, target, &mut vec![vec![]])
                    .len()
            })
            .sum::<usize>() as u32
    }

    fn find_path(
        &mut self,
        start: (i32, i32),
        target: (i32, i32),
        visited: &mut Vec<(i32, i32)>,
    ) -> Option<Route> {
        visited.push(start);

        if start == target {
            return Some(visited.to_owned());
        }

        let height = self.grid.get_xy(start).unwrap();

        let neighbors = self.neighbors_for(start).into_iter().filter_map(|p| {
            let cell = self.grid.get_xy(p)?;
            if cell == height + 1 && !visited.contains(&p) {
                Some(p)
            } else {
                None
            }
        });

        for n in neighbors {
            let path = self.find_path(n, target, &mut visited.clone());
            if path.is_some() {
                return path;
            }
        }

        None
    }

    fn find_all_paths(
        &mut self,
        start: (i32, i32),
        target: (i32, i32),
        visited: &mut Vec<Route>,
    ) -> Vec<Route> {
        for route in visited.iter_mut() {
            route.push(start);
        }

        if start == target {
            return visited.to_owned();
        }

        let height = self.grid.get_xy(start).unwrap();

        let neighbors = self.neighbors_for(start).into_iter().filter_map(|p| {
            let cell = self.grid.get_xy(p)?;
            if cell == height + 1 {
                Some(p)
            } else {
                None
            }
        });

        let mut paths = vec![];
        for n in neighbors {
            let found = self.find_all_paths(n, target, &mut visited.clone());
            paths.push(found);
        }

        if paths.len() > 0 {
            paths.into_iter().flatten().collect()
        } else {
            vec![]
        }
    }

    fn neighbors_for(&self, xy: (i32, i32)) -> Vec<(i32, i32)> {
        vec![
            (xy.0, xy.1 - 1),
            (xy.0 + 1, xy.1),
            (xy.0, xy.1 + 1),
            (xy.0 - 1, xy.1),
        ]
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> u32 {
    let grid: Grid<usize> = Grid::from(input);

    grid.start_positions()
        .map(|pos| Trailhead::new(pos, &grid))
        .map(|mut x| x.score_part1())
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u32 {
    let grid: Grid<usize> = Grid::from(input);

    grid.start_positions()
        .map(|pos| Trailhead::new(pos, &grid))
        .map(|mut x| x.score_part2())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 36);
    }

    #[test]
    fn map_trailheads() {
        let map: Grid<usize> = Grid::from(INPUT);
        let trailheads = map.start_positions().collect::<Vec<(i32, i32)>>();

        assert_eq!(
            trailheads,
            vec![
                (2, 0),
                (4, 0),
                (4, 2),
                (6, 4),
                (2, 5),
                (5, 5),
                (0, 6),
                (6, 6),
                (1, 7)
            ]
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 81);
    }
}
