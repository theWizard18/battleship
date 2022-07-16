use rand::Rng;
use crate::cell::Cell;

pub struct Grid {
    pub cells :[[Cell; 10]; 10],
    ships_sizes :[usize; 5],
    hide: bool,
}
impl Grid {
    pub fn print(&self) {
        let mut row_counter = 0;
        for row in &self.cells {
            for cell in row {
                print!(" {}", cell.chr_sprite());
            }
            println!(" {}", row_counter);
            row_counter += 1;
        }
        println!(" A B C D E F G H I J");
    }

    pub fn new(hide: bool) -> Grid {
        let mut grid = Grid {
            cells: [[Cell::Water; 10]; 10],
            ships_sizes: [5, 4, 3, 3, 2,],
            hide,
        };
        grid.distribute_ships();
        grid
    }

    fn distribute_ships(&mut self) {
        let mut rng = rand::thread_rng();
        let (mut x, mut y);
        let mut is_horizontal: bool;
        for size in self.ships_sizes {
            loop {
                (x,y) = (rng.gen_range(0..9), rng.gen_range(0..9));
                is_horizontal = rng.gen_bool(0.5);
                if self.ship_distribution(&is_horizontal, size.clone(), x.clone(), y.clone()) {
                    break;
                };
            };
        };
    }

    fn ship_distribution(&mut self, is_horizontal: &bool, i: usize, x: usize, y: usize) -> bool {
        match is_horizontal {
            false => {
                if i-1 + y < 10 && !self.cells[x][y..y+i].iter()
                        .any(|&cell| cell == Cell::Ship) {
                    for j in 0..i {
                        self.cells[x][y+j] = Cell::Ship;
                    };
                    return true
                };
            },
            true => {
                if i-1 + x < 10 && !self.cells[x..x+i].iter()
                        .any(|&row| row[y] == Cell::Ship) {
                    for j in 0..i {
                        self.cells[x+j][y] = Cell::Ship;
                    };
                    return true
                };
            },
        }
        false
    }
}

