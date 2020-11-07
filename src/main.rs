extern crate rand;

mod auto;
mod gui;

use auto::Grid;
use auto::Auto;
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

fn main() {
    let mut a = Auto::new(
        Grid::new_2d(gen_grid(150, 150), 150),
        |&cell, n| {
            let living = n.iter().filter(|&&t| t == Some(CellType::Alive)).count();
            if (living == 2 && cell == CellType::Alive) || living == 3 {
                return CellType::Alive
            } else {
                return CellType::Dead
            }
        },
    );
    /*
     * [ 1 2 3 4 5 6 7 ]
     * filter_map(
     *  n < 5 : Some(n+1)
        else: None
     * [ 2 3 4 5 ];
    */
    a.run("Conway's Game of Life", 100);
}
