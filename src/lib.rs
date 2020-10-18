extern crate rand;

mod auto;
mod gui;
use gui::Colored;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Dead,
    Alive,
}

impl Colored for CellType {
    fn get_color(&self) -> [f32; 4] {
        match self {
            Self::Alive => [1.0, 1.0, 1.0, 1.0],
            Self::Dead => [0.0, 0.0, 0.0, 1.0],
        }
        
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum WW {
    Empty,
    EHead,
    ETail,
    Conductor,
}

impl Colored for WW {
    fn get_color(&self) -> [f32; 4] {
        match self {
            Self::Empty => [0.0, 0.0, 0.0, 1.0],
            Self::EHead => [0.0, 0.0, 1.0, 1.0],
            Self::ETail => [1.0, 0.0, 0.0, 1.0],
            Self::Conductor => [1.0, 1.0, 0.0, 1.0],
        }
        
    }
}

fn gen_grid(side: usize) -> Vec<Vec<CellType>> {
    let mut grid: Vec<Vec<CellType>> = Vec::new();
    let (w, h) = (side, side);
    for y in 0..h {
        grid.push(vec![]);
        for _ in 0..w {
            if rand::random() {
                grid[y].push(CellType::Alive);
            } else {
                grid[y].push(CellType::Dead);
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use auto::Auto;

    #[test]
    fn stuff() {
        let mut a = match Auto::new(vec![
                                    vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
                                    vec![WW::EHead, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor],
                                    vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
                                    vec![WW::EHead, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor],
                                    vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
                                    vec![WW::EHead, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor],
                                    vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
                                    vec![WW::EHead, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor],
                                    vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
                                    vec![WW::EHead, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor],
                                    vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
        ], None) {
            Ok(t) => t,
            Err(e) => panic!(e)
        };
        // Wireworld
        a.set_rules(|cell, n| {
            let heads = n.iter().filter(|&&t| t == WW::EHead).count();
            return match cell {
                WW::Empty => WW::Empty,
                WW::EHead => WW::ETail,
                WW::ETail => WW::Conductor,
                WW::Conductor => {
                    return if heads == 1 || heads == 2 {
                        WW::EHead
                    } else {
                        WW::Conductor
                    }
                },
            }
        });
        // Conway's Game of Life
        //a.set_rules(|cell, n| {
            //let living = n.iter().filter(|&&t| t == CellType::Alive).count();
            //if cell == CellType::Alive {
                //if living == 2 || living == 3 {
                    //return CellType::Alive
                //} else {
                    //return CellType::Dead
                //}
            //} else {
                //if living == 3 {
                    //return CellType::Alive
                //} else {
                    //return CellType::Dead
                //}
            //}
        //});
        a.run();
    }
}
