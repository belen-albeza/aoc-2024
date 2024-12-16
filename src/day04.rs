use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let soup = Soup::from(input);

    let dirs = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let target = "xmas";
    let mut res = 0;

    for y in 0..soup.height() as i32 {
        for x in 0..soup.width() as i32 {
            let words = dirs.iter().filter_map(|&step| {
                let word: String = soup.word_iter((x, y), step, target.len()).collect();
                if word == target {
                    Some(word)
                } else {
                    None
                }
            });

            res += words.count() as u32
        }
    }

    res
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    let target = "mas";
    let offsets = [(-1, -1), (1, 1), (-1, 1), (1, -1)]
        .into_iter()
        .map(|(a, b)| {
            let half = (target.len() as i32 - 1) / 2;
            (a * half, b * half)
        })
        .collect::<Vec<(i32, i32)>>();

    let soup = Soup::from(input);

    let mut res = 0;

    for y in 0..soup.height() as i32 {
        for x in 0..soup.width() as i32 {
            let words = offsets.iter().filter_map(|&offset| {
                let step = (offset.0 * -1, offset.1 * -1);
                let start = (x + offset.0, y + offset.1);
                let word: String = soup.word_iter(start, step, target.len()).collect();

                if word == target {
                    Some(word)
                } else {
                    None
                }
            });

            if words.count() >= 2 {
                res += 1;
            }
        }
    }

    res
}

#[derive(Debug, Clone, PartialEq)]
struct Soup {
    cells: Vec<char>,
    width: usize,
}

impl Soup {
    fn get(&self, (x, y): (i32, i32)) -> Option<char> {
        if x < 0 || x >= self.width() as i32 || y < 0 || y >= self.height() as i32 {
            return None;
        }

        let idx = y * self.width() as i32 + x;
        self.cells.get(idx as usize).copied()
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    fn word_iter(
        &self,
        start: (i32, i32),
        step: (i32, i32),
        target_len: usize,
    ) -> impl Iterator<Item = char> + use<'_> {
        SoupIterator::new(&self, start, step, target_len)
    }
}

impl From<&str> for Soup {
    fn from(value: &str) -> Self {
        let cells = value
            .to_lowercase()
            .chars()
            .filter(|x| !x.is_whitespace())
            .collect();
        let width = value.len() / value.lines().count();

        Self { cells, width }
    }
}

struct SoupIterator<'a> {
    soup: &'a Soup,
    step: (i32, i32),
    current: (i32, i32),
    target_len: usize,
    count: usize,
}

impl<'a> SoupIterator<'a> {
    fn new(soup: &'a Soup, start: (i32, i32), step: (i32, i32), target_len: usize) -> Self {
        Self {
            soup,
            current: start,
            step,
            target_len,
            count: 0,
        }
    }
}

impl<'a> Iterator for SoupIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.target_len {
            return None;
        }

        if let Some(x) = self.soup.get(self.current) {
            self.current = (self.current.0 + self.step.0, self.current.1 + self.step.1);
            self.count += 1;

            Some(x)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn soup_from_input() {
        let input = "ABC\nDEF";
        assert_eq!(
            Soup::from(input),
            Soup {
                cells: vec!['a', 'b', 'c', 'd', 'e', 'f'],
                width: 3,
            }
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 9);
    }
}
