pub use crate::grid::Grid;

pub struct Player {
    pub name: String,
    pub grid: Grid,
    pub attack: Option<(usize, usize)>,
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            grid: Grid::new(false),
            attack: None
        }
    }
}

