const NGH: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub struct Auto<CellType: Clone + PartialEq> {
    grid: Box<Vec<Vec<CellType>>>,
    rules: fn(&CellType, &Vec<CellType>) -> CellType,
}

impl<CellType: Clone + PartialEq> Auto<CellType> {
    pub fn new(grid: Vec<Vec<CellType>>, rules: fn(&CellType, &Vec<CellType>) -> CellType) -> Self {
        if Self::validate_grid(&grid) {
            return Auto {
                grid: Box::new(grid),
                rules,
            }
        } else {
            panic!("Invalid grid!");
        }
    }

    fn validate_grid(grid: &Vec<Vec<CellType>>) -> bool {
        let width = grid[0].len();
        for row in grid.iter() {
            if row.len() != width {
                return false
            }
        }
        true
    }

    pub fn get_grid(&self) -> &Vec<Vec<CellType>> {
        &self.grid
    }

    pub fn get_neighbours(grid: &Vec<Vec<CellType>>, coor: (usize, usize)) -> Vec<CellType> {
        NGH.iter()
            .filter_map(|c| {
                let x = usize::overflowing_add(coor.0,c.0 as usize).0;
                let y = usize::overflowing_add(coor.1,c.1 as usize).0;
                return if x <= (grid.len() as i32-1) as usize && y <= (grid[0].len()-1) as usize {
                    Some(grid[x][y].clone())
                } else {
                    None
                }
            }).collect()
    }

    pub fn step(&mut self) {
        let rules = self.rules;
        let mut new: Box<Vec<Vec<CellType>>> = Box::new(Vec::with_capacity(self.grid.len()));
        // take step
        for y in 0..self.grid.len() {
            new.push(Vec::with_capacity(self.grid[0].len()));
            for x in 0..self.grid[0].len() {
                new[y].push(rules(&self.grid[y][x], &Self::get_neighbours(&self.grid, (y, x))));
            }
        }
        self.grid = new;
    }
}

impl<CellType: Copy + PartialEq + std::fmt::Display> Auto<CellType> {
    pub fn print(&self) {
        self.grid.iter().for_each(|row| {
            row.iter().for_each(|cell| print!("{} ", cell));
            println!("");
        });
    }
}
