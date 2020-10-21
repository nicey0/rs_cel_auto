extern crate rand;

mod auto;
mod gui;
use gui::Colored;

fn gen_grid(w: usize, h: usize) -> Vec<Vec<CellType>> {
    let mut grid: Vec<Vec<CellType>> = Vec::new();
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

fn n_to_ct(v: Vec<Vec<usize>>) -> Vec<Vec<CellType>> {
    let mut grid: Vec<Vec<CellType>> = Vec::new();
    for y in 0..v.len() {
        grid.push(vec![]);
        for x in 0..v[0].len() {
            grid[y].push(if v[y][x] == 1 {
                CellType::Alive
            } else {
                CellType::Dead
            });
        }
    }
    grid
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Dead,
    Alive,
}

impl std::fmt::Display for CellType {
    fn fmt(&self,  f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Dead => 0,
            Self::Alive => 1,
        })
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use auto::Auto;

    #[test]
    fn game_of_life() {
        // Conway's Game of Life
        let mut a = Auto::new(
            gen_grid(100, 100),
            |&cell, n| {
                let living = n.iter().filter(|&&t| t == CellType::Alive).count();
                if (living == 2 && cell == CellType::Alive) || living == 3 {
                    return CellType::Alive
                } else {
                    return CellType::Dead
                }
            }
        );
        let start = Instant::now();
        for _ in 0..50 {
            a.step();
        }
        let took = start.elapsed().as_millis();
        println!("{:.3} FPS", 50f64 / (took as f64 / 1000f64));
    }

    //#[test]
    //fn wireworld() {
        //let mut a = match Auto::new(vec![
            //vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
            //vec![WW::EHead, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor],
            //vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
            //vec![WW::EHead, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor],
            //vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
            //vec![WW::EHead, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor],
            //vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
            //vec![WW::EHead, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor],
            //vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
            //vec![WW::EHead, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor, WW::Conductor],
            //vec![WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty,     WW::Empty, WW::Empty],
        //], None) { Ok(t) => t, Err(e) => panic!(e) };
        //a.set_rules(|&cell, &n| {
            //let heads = n.iter().filter(|&&t| t == WW::EHead).count();
            //return match cell {
                //WW::Empty => WW::Empty,
                //WW::EHead => WW::ETail,
                //WW::ETail => WW::Conductor,
                //WW::Conductor => {
                    //return if heads == 1 || heads == 2 {
                        //WW::EHead
                    //} else {
                        //WW::Conductor
                    //}
                //},
            //}
        //});
        //a.run("Wireworld", 4);
    //}
}
