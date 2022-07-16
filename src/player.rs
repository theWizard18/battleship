pub use crate::grid::Grid;
use crate::cell::Cell;

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

    pub fn print(&self) {
        println!("{}", self.name);
        self.grid.print();
    }

    pub fn process_attacked_cell(&self, adversary: &mut Player,
        y: usize, x: usize) {
        match adversary.grid.cells[y][x] {
            Cell::Ship => {
                println!("{} striked a(n) {} ship", self.name, adversary.name);
                adversary.grid.cells[y][x] = Cell::StrikedShip;
            },
            Cell::Water => {
                println!("{} striked the water", self.name);
                adversary.grid.cells[y][x] = Cell::StrikedWater;
            },
            _ => println!("{} already striked this cell", self.name),
        }
    }
}

