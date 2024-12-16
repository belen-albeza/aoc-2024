#[derive(Debug, PartialEq, Clone)]
pub struct Grid<T: Copy> {
    cells: Vec<T>,
    width: usize,
}

impl<T: Copy> Default for Grid<T> {
    fn default() -> Self {
        Self {
            cells: vec![],
            width: 0,
        }
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(cells: Vec<T>, width: usize) -> Self {
        Self { cells, width }
    }

    pub fn get_xy(&self, position: (i32, i32)) -> Option<T> {
        self.index_for(position).map(|idx| self.cells[idx])
    }

    pub fn width(&self) -> i32 {
        self.width as i32
    }

    pub fn height(&self) -> i32 {
        (self.cells.len() / self.width) as i32
    }

    fn index_for(&self, (x, y): (i32, i32)) -> Option<usize> {
        if x < 0 || x >= self.width() || y < 0 || y >= self.height() {
            return None;
        }

        Some((y * self.width() + x) as usize)
    }

    // fn xy_for(&self, index: usize) -> Option<(i32, i32)> {
    //     let y = index / self.width;
    //     let x = index % self.width;

    //     if index < self.cells.len() {
    //         Some((x as i32, y as i32))
    //     } else {
    //         None
    //     }
    // }
}