use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

type Point = (i32, i32);
type Size = (i32, i32);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn tick(&mut self, bounds: (i32, i32)) {
        self.position.0 = (self.position.0 + self.velocity.0).rem_euclid(bounds.0);
        self.position.1 = (self.position.1 + self.velocity.1).rem_euclid(bounds.1);
    }

    fn quadrant(&self, bounds: (i32, i32)) -> Option<Quadrant> {
        let half_width = bounds.0 / 2;
        let half_height = bounds.1 / 2;
        match self.position {
            (x, y) if x < half_width && y < half_height => Some(Quadrant::TopLeft),
            (x, y) if x > half_width && y < half_height => Some(Quadrant::TopRight),
            (x, y) if x < half_width && y > half_height => Some(Quadrant::BottomLeft),
            (x, y) if x > half_width && y > half_height => Some(Quadrant::BottomRight),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    robots: Vec<Robot>,
    bounds: Size,
}

impl Map {
    pub fn new(robots: &[Robot], bounds: Size) -> Self {
        Self {
            robots: robots.to_owned(),
            bounds,
        }
    }

    pub fn run(&mut self, ticks: usize) -> Vec<Robot> {
        for _ in 0..ticks {
            for robot in self.robots.iter_mut() {
                robot.tick(self.bounds);
            }
        }

        self.robots.clone()
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"-?\d+").unwrap();

    input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = re
                .find_iter(line)
                .map(|x| x.as_str().parse().unwrap())
                .collect();
            Robot {
                position: (nums[0], nums[1]),
                velocity: (nums[2], nums[3]),
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Robot]) -> u64 {
    let bounds = if cfg!(test) { (11, 7) } else { (101, 103) };

    let mut map = Map::new(input, bounds);
    let robots = map.run(100);
    robots
        .into_iter()
        .sorted_by_key(|r| r.quadrant(bounds))
        .chunk_by(|r| r.quadrant(bounds))
        .into_iter()
        .filter_map(|(quad, group)| quad.map(|_| group.count()))
        .fold(1, |acc, x| x * acc) as u64
}

#[aoc(day14, part2)]
fn part2(_: &[Robot]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn parse_input() {
        let input = r"p=0,4 v=3,-3";
        assert_eq!(
            parse(input),
            vec![Robot {
                position: (0, 4),
                velocity: (3, -3)
            }]
        );
    }

    #[test]
    fn robot_tick() {
        let mut robot = Robot {
            position: (0, 0),
            velocity: (-2, -1),
        };

        robot.tick((5, 5));

        assert_eq!(robot.position, (3, 4));
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 12);
    }
}
