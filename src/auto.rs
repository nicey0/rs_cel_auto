#[derive(Debug)]
pub struct Auto<CellType: Copy + PartialEq> {
    width: usize,
    grid: Vec<Vec<CellType>>,
    rules: Option<fn(CellType, Vec<CellType>) -> CellType>,
}

impl<CellType: Copy + PartialEq> Auto<CellType> {
    pub fn new(grid: Vec<Vec<CellType>>, rules: Option<fn(CellType, Vec<CellType>) -> CellType>) -> Result<Self,
    &'static str> {
        if Self::validate_grid(&grid) {
            return Ok(Auto {
                width: grid[0].len(),
                grid,
                rules,
            })
        }
        Err("Invalid grid")
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

    fn get_neighbours(&self, coor: (usize, usize)) -> Vec<CellType> {
        let (x, y) = (coor.0 as i32, coor.1 as i32);
        let (maxx, maxy) = ((self.width-1) as i32, (self.grid.len()/self.width-1) as i32);
        let mut valid_coors: Vec<(i32, i32)> = vec![
             (x-1, y-1), (x, y-1), (x+1, y-1),
             (x-1, y),             (x+1, y),
             (x-1, y+1), (x, y+1), (x+1, y+1),
        ];
        // filter out any invalid coordinates
        valid_coors.retain(|c| c.0 >= 0 && c.0 <= maxx && c.1 >= 0 && c.1 <= maxy);
        // collect type for each remaining valid coordinate into Vec<CellType> and return it
        valid_coors.iter().map(|&e| self.grid[e.0 as usize][e.1 as usize]).collect()
    }

    pub fn get_grid(&self) -> &Vec<Vec<CellType>> {
        &self.grid
    }

    pub fn set_rules(&mut self, rules: fn(CellType, Vec<CellType>) -> CellType) {
        self.rules = Some(rules);
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        if let Some(rules) = self.rules {
            // take step
            self.grid = self.grid.iter().enumerate().map(|(y, row)|
                row.iter().enumerate().map(|(x, &cell)|
                    // apply rules for each cell
                    rules(cell, self.get_neighbours((x, y))))
                .collect())
            .collect();
            return Ok(())
        } else {
            return Err("No rules");
        }
    }

    pub fn step_panic(&mut self) {
        if let Err(e) = self.step() {
            panic!(e);
        }
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
