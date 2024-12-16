use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    North = 0,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Wall([bool; 4]),
    Guard(Dir),
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall([false; 4]),
            '^' => Self::Guard(Dir::North),
            '>' => Self::Guard(Dir::East),
            'v' => Self::Guard(Dir::South),
            '<' => Self::Guard(Dir::West),
            _ => Self::Empty,
        }
    }
}

impl Cell {
    fn add_hit(&self, dir: Dir) -> Option<Cell> {
        match self {
            Self::Wall(hits) => {
                let idx = dir as usize;
                if hits[idx] {
                    return None;
                }

                let mut new_hits = hits.clone();
                new_hits[idx] = true;
                Some(Self::Wall(new_hits))
            }
            _ => Some(*self),
        }
    }

    fn is_guard(&self) -> bool {
        match self {
            Self::Guard(_) => true,
            _ => false,
        }
    }

    fn is_obstacle(&self) -> bool {
        match self {
            Self::Wall(_) => true,
            _ => false,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    fn dir(&self) -> Option<Dir> {
        match self {
            Self::Guard(dir) => Some(*dir),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Guard {
    position: (i32, i32),
    dir: Dir,
}

impl Guard {
    fn advance(&self) -> Self {
        let position = match self.dir {
            Dir::North => (self.position.0, self.position.1 - 1),
            Dir::East => (self.position.0 + 1, self.position.1),
            Dir::South => (self.position.0, self.position.1 + 1),
            Dir::West => (self.position.0 - 1, self.position.1),
        };

        Self {
            position,
            dir: self.dir,
        }
    }

    fn turn(&self) -> Self {
        let dir = match self.dir {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        };

        Self {
            position: self.position,
            dir,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    cells: Vec<Cell>,
    width: usize,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        if let Some(row) = value.lines().peekable().peek() {
            let width = row.len();
            let cells = value
                .lines()
                .map(|l| l.chars().map(Cell::from).collect::<Vec<Cell>>())
                .flatten()
                .collect();
            Self { width, cells }
        } else {
            Self {
                width: 0,
                cells: vec![],
            }
        }
    }
}

impl Grid {
    fn guard(&self) -> Guard {
        let (i, cell) = self
            .cells
            .iter()
            .enumerate()
            .find(|(_, x)| x.is_guard())
            .unwrap();

        let position = self.xy_for(i);
        let dir = cell.dir().unwrap();

        Guard { position, dir }
    }

    fn xy(&self, position: (i32, i32)) -> Option<Cell> {
        self.index_for(position).map(|i| self.cells[i])
    }

    fn set_xy(&mut self, position: (i32, i32), cell: Cell) -> Option<Cell> {
        let i = self.index_for(position)?;
        self.cells[i] = cell;

        Some(cell)
    }

    fn index_for(&self, (x, y): (i32, i32)) -> Option<usize> {
        if x >= 0 && x < self.width() && y >= 0 && y < self.height() {
            Some((y * self.width() + x) as usize)
        } else {
            None
        }
    }

    fn xy_for(&self, i: usize) -> (i32, i32) {
        let y = i / self.width;
        let x = i % self.width;

        (x as i32, y as i32)
    }

    fn width(&self) -> i32 {
        self.width as i32
    }

    fn height(&self) -> i32 {
        if self.cells.len() > 0 {
            (self.cells.len() / self.width) as i32
        } else {
            0
        }
    }
}

pub struct Patrol {
    grid: Grid,
    guard: Guard,
    has_loop: Option<bool>,
}

impl Patrol {
    pub fn new(grid: Grid) -> Self {
        let guard = grid.guard();
        Self {
            grid,
            guard,
            has_loop: None,
        }
    }

    pub fn run(&mut self) -> bool {
        while self.tick().is_some() {}

        self.has_loop.unwrap_or(false)
    }

    fn tick(&mut self) -> Option<bool> {
        let forward_guard = self.guard.advance();
        if let Some(cell) = self.grid.xy(forward_guard.position) {
            if cell == Cell::Guard(forward_guard.dir) {
                self.has_loop = Some(true);
                return None;
            }

            self.guard = if cell.is_obstacle() {
                if let Some(new_cell) = cell.add_hit(self.guard.dir) {
                    self.grid.set_xy(forward_guard.position, new_cell);
                    self.guard.turn()
                } else {
                    self.has_loop = Some(true);
                    return None;
                }
            } else {
                forward_guard
            };

            self.grid
                .set_xy(self.guard.position, Cell::Guard(self.guard.dir))?;
            self.has_loop = Some(false);
            Some(false)
        } else {
            None
        }
    }

    fn route_len(&self) -> usize {
        self.grid.cells.iter().filter(|x| x.is_guard()).count()
    }
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u32 {
    let grid = Grid::from(input);
    let mut patrol = Patrol::new(grid);

    patrol.run();
    patrol.route_len() as u32
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u32 {
    let grid = Grid::from(input);

    let empty_indices =
        grid.cells
            .iter()
            .enumerate()
            .filter_map(|(idx, cell)| if cell.is_empty() { Some(idx) } else { None });

    let mut patrols = empty_indices
        .map(|i| {
            let mut patrol_grid = grid.clone();
            patrol_grid.cells[i] = Cell::Wall([false; 4]);

            Patrol::new(patrol_grid)
        })
        .collect::<Vec<Patrol>>();

    let mut res = 0;
    for patrol in patrols.iter_mut() {
        let has_loop = patrol.run();
        if has_loop {
            res += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 6);
    }
}
