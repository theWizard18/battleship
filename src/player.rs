pub use crate::grid::Grid;

pub struct Player {
    pub name: String,
    pub grid: Grid,
}
impl Player {
    pub fn new(name: String, hide: bool) -> Player {
        Player {
            name,
            grid: Grid::new(hide),
        }
    }
}

