use std::fmt;

mod auto;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Dead,
    Alive,
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Dead => "0",
            Self::Alive => "1",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use auto::Auto;

    #[test]
    fn it_works() {
        let grid = vec![
            vec![CellType::Dead; 3],
            vec![CellType::Alive; 3],
            vec![CellType::Dead; 3],
        ];
        let mut a = match Auto::new(grid, None) {
            Ok(t) => t,
            Err(e) => panic!(e),
        };
        a.set_rules(|cell, _| {
            if cell == CellType::Alive {
                return CellType::Dead
            } else {
                return CellType::Alive
            }
        });
        a.step_panic();
        a.print();
        println!("--------------");
        a.step_panic();
        a.print();
        println!("--------------");
        a.step_panic();
        a.print();
        println!("--------------");
        a.step_panic();
        a.print();
        println!("--------------");
        a.step_panic();
        a.print();
        println!("--------------");
        a.step_panic();
        a.print();
    }
}
