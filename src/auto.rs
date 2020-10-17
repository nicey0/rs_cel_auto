#[derive(Debug)]
pub struct Auto<CellType: Clone + PartialEq> {
    dead: CellType,
    grid: Vec<CellType>,
    width: usize,
    rules: Option<fn(CellType, Vec<CellType>) -> CellType>,
}

fn twod_to_oned<CellType: Clone + PartialEq>(twod: Vec<Vec<CellType>>) -> Vec<CellType> {
    let mut oned: Vec<CellType> = Vec::new();
    twod.iter().for_each(|row| row.iter().for_each(|e| oned.push(e.clone())));
    oned
}

impl<CellType: Clone + PartialEq> Auto<CellType> {
    pub fn new(dead: CellType, grid_2d: Vec<Vec<CellType>>,
               rules: Option<fn(CellType, Vec<CellType>) -> CellType>) -> Self {
        Auto {
            dead,
            width: grid_2d[0].len(),
            grid: twod_to_oned::<CellType>(grid_2d),
            rules,
        }
    }

    fn coor(&self, coor: (i32, i32)) -> CellType {
        self.grid[(coor.0 as usize) + (coor.1 as usize)*self.width].clone()
    }

    fn get_neighbours(&self, coor: (usize, usize)) -> Vec<CellType> {
        let (x, y) = (coor.0 as i32, coor.1 as i32);
        let (maxx, maxy) = ((self.width-1) as i32, (self.grid.len()/self.width-1) as i32);
        let mut valid_coors: Vec<(i32, i32)> = vec![
             (x-1, y-1), (x, y-1), (x+1, y-1),
             (x-1, y),             (x+1, y),
             (x-1, y+1), (x, y+1), (x+1, y+1),
        ];
        valid_coors.retain(|c| c.0 >= 0 && c.0 <= maxx && c.1 >= 0 && c.1 <= maxy);
        valid_coors.iter().map(|&e| self.coor((e.0, e.1))).collect()
    }

    pub fn get_grid(&self) -> &Vec<CellType> {
        &self.grid
    }

    pub fn set_rules(&mut self, rules: fn(CellType, Vec<CellType>) -> CellType) {
        self.rules = Some(rules);
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        if let Some(rules) = self.rules {
            // take step
            self.grid = self.grid.iter().enumerate()
                .map(|(i, cell)| {
                    rules(cell.clone(), self.get_neighbours(
                            (i % self.width, i / self.width)))
                })
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
