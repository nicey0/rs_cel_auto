const NGH: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub struct Grid<CellType: Copy + PartialEq> {
    c: Vec<CellType>,
    width: usize,
}

impl<CellType: Copy + PartialEq> Grid<CellType> {
    pub fn new_1d(c: Vec<CellType>, width: usize) -> Self {
        Grid {
            c,
            width,
        }
    }

    pub fn new_2d(c: Vec<Vec<CellType>>, width: usize) -> Self {
        Grid {
            c: Grid::engooden_grid(c),
            width,
        }
    }

    fn engooden_grid(c: Vec<Vec<CellType>>) -> Vec<CellType> {
        let mut engoodened = Vec::with_capacity(c.len()*c[0].len());
        for row in c.iter() {
            for &c in row.iter() {
                engoodened.push(c);
            }
        }
        engoodened
    }

    pub fn get(&self, x: usize, y: usize) -> CellType {
        self.c[y * self.width + x]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.c.len() / self.width
    }

    pub fn len(&self) -> usize {
        self.c.len()
    }

    pub fn push(&mut self, e: CellType) {
        self.c.push(e);
    }
}

pub struct Auto<CellType: Copy + PartialEq> {
    grid: Grid<CellType>,
    rules: fn(&CellType, [Option<CellType>; 8]) -> CellType,
    neighbours: [Option<CellType>; 8],
}

impl<CellType: Copy + PartialEq> Auto<CellType> {
    pub fn new(grid: Grid<CellType>, rules: fn(&CellType, [Option<CellType>; 8]) -> CellType) -> Self {
        Auto {
            grid,
            rules,
            neighbours: [None; 8],
        }
    }

    pub fn get_grid(&self) -> &Grid<CellType> {
        &self.grid
    }

    pub fn get_neighbours(&mut self, x: usize, y: usize) -> [Option<CellType>; 8] {
        for (i, coor) in NGH.iter().enumerate() {
            let x = usize::overflowing_add(x, coor.0 as usize).0;
            let y = usize::overflowing_add(y, coor.1 as usize).0;
            self.neighbours[i] = if x <= (self.grid.width as i32-1) as usize && y <= (self.grid.height()-1) as usize {
                Some(self.grid.get(x, y).clone())
            } else {
                None
            }
        }
        self.neighbours
    }

    pub fn step(&mut self) {
        let rules = self.rules;
        let mut new = Grid::new_1d(Vec::with_capacity(self.grid.len()), self.grid.width);
        // take step
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width {
                new.push(rules(&self.grid.get(y, x), self.get_neighbours(y, x)));
            }
        }
        self.grid = new;
    }
}

//impl<CellType: Copy + PartialEq + std::fmt::Display> Auto<CellType> {
    //pub fn print(&self) {
    //}
//}
