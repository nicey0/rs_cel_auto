mod auto;

#[derive(Debug, Clone, PartialEq)]
enum CellType {
    Dead,
    Alive,
}

#[cfg(test)]
mod tests {
    use super::*;
    use auto::Auto;

    #[test]
    fn it_works() {
        let mut a: Auto<CellType> = Auto::new(CellType::Dead,
                                              vec![
                                                  vec![CellType::Dead; 3],
                                                  vec![CellType::Alive; 3],
                                                  vec![CellType::Dead; 3],
                                              ],
                                              None);
        println!("{:#?}", a);
        a.set_rules(|cell, _| {
            if cell == CellType::Alive {
                return CellType::Dead
            } else {
                return CellType::Alive
            }
        });
        a.step_panic();
        println!("{:#?}", a);
    }
}
