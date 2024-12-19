use aoc_runner_derive::aoc;

use crate::utils::Grid;

type Point = (i32, i32);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,
}

impl From<char> for Action {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

impl Action {
    fn delta(&self) -> Point {
        match self {
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Up => (0, -1),
            Self::Down => (0, 1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Box,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Box,
            _ => Self::Empty,
        }
    }
}

impl Into<char> for Cell {
    fn into(self) -> char {
        match self {
            Self::Wall => '#',
            Self::Box => 'O',
            _ => ' ',
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
            let width = rows[0].len();
            Self::new(rows.into_iter().flatten().collect(), width)
        }
    }
}

pub struct Warehouse {
    grid: Grid<Cell>,
    robot: Point,
}

impl From<&str> for Warehouse {
    fn from(value: &str) -> Self {
        let grid = Grid::<Cell>::from(value);
        let robot_idx = value
            .chars()
            .filter(|c| !c.is_whitespace())
            .position(|c| c == '@')
            .unwrap();
        let robot = grid.xy_for(robot_idx).unwrap();

        Self { grid: grid, robot }
    }
}

impl std::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.grid.height() {
            let row: String = (0..self.grid.width())
                .into_iter()
                .map(|x| {
                    if (x, y as i32) == self.robot {
                        '@'
                    } else {
                        self.grid.get_xy((x, y)).unwrap().into()
                    }
                })
                .collect();
            writeln!(f, "{}", row)?;
        }

        Ok(())
    }
}

impl Warehouse {
    pub fn run(&mut self, actions: &[Action]) {
        for &action in actions {
            self.run_action(action)
        }
    }

    pub fn boxes(self) -> Vec<Point> {
        self.grid
            .cells
            .iter()
            .enumerate()
            .filter_map(|(idx, &cell)| {
                if cell == Cell::Box {
                    Some(self.grid.xy_for(idx).unwrap())
                } else {
                    None
                }
            })
            .collect()
    }

    fn run_action(&mut self, action: Action) {
        let dir = action.delta();
        let xy = (self.robot.0 + dir.0, self.robot.1 + dir.1);

        let new_cell = self.grid.get_xy(xy).unwrap();
        match new_cell {
            Cell::Wall => {}
            Cell::Empty => self.robot = xy,
            Cell::Box => {
                self.push_box(xy, dir);
            }
        }
    }

    fn push_box(&mut self, position: Point, dir: Point) {
        let mut behind = position;
        while let Some(Cell::Box) = self.grid.get_xy(behind) {
            behind = (behind.0 + dir.0, behind.1 + dir.1);
        }

        match self.grid.get_xy(behind).unwrap() {
            Cell::Empty => {
                self.robot = position;
                self.grid.set_xy(position, Cell::Empty).unwrap();
                self.grid.set_xy(behind, Cell::Box).unwrap();
            }
            _ => {} // do nothing
        };
    }
}

fn parse_input(input: &str) -> (Warehouse, Vec<Action>) {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let warehouse = Warehouse::from(sections[0]);
    let actions = sections[1]
        .chars()
        .filter(|x| !x.is_whitespace())
        .map(Action::from)
        .collect();

    (warehouse, actions)
}

#[aoc(day15, part1)]
fn part1(input: &str) -> u64 {
    let (mut warehouse, actions) = parse_input(input);
    warehouse.run(&actions);
    warehouse
        .boxes()
        .into_iter()
        .map(|(x, y)| 100 * y as u64 + x as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 10092);
    }

    #[test]
    fn robot_moves_to_empty() {
        let input = r"####
#@.#
####";
        let mut warehouse = Warehouse::from(input);
        warehouse.run_action(Action::Right);

        assert_eq!(warehouse.robot, (2, 1));
    }

    #[test]
    fn robot_moves_against_wall() {
        let input = r"####
#@.#
####";
        let mut warehouse = Warehouse::from(input);
        warehouse.run_action(Action::Left);

        assert_eq!(warehouse.robot, (1, 1));
    }

    #[test]
    fn robot_moves_against_box() {
        let input = r"#####
#@O.#
#####";
        let mut warehouse = Warehouse::from(input);
        warehouse.run_action(Action::Right);
        assert_eq!(warehouse.robot, (2, 1));
        assert_eq!(warehouse.grid.get_xy((2, 1)), Some(Cell::Empty));
        assert_eq!(warehouse.grid.get_xy((3, 1)), Some(Cell::Box));
    }

    #[test]
    fn robot_moves_against_many_boxes() {
        let input = r"######
#@OO.#
######";
        let mut warehouse = Warehouse::from(input);
        warehouse.run_action(Action::Right);
        assert_eq!(warehouse.robot, (2, 1));
        assert_eq!(warehouse.grid.get_xy((2, 1)), Some(Cell::Empty));
        assert_eq!(warehouse.grid.get_xy((3, 1)), Some(Cell::Box));
        assert_eq!(warehouse.grid.get_xy((4, 1)), Some(Cell::Box));
    }
}
