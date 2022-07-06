use rand::Rng;

pub struct Grid {
    cells :[[Cell; 10]; 10],
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

    pub fn new() -> Grid {
        let mut grid = Grid {
            cells: [[Cell::Water; 10]; 10],
        };
        grid.distribute_ships();
        grid
    }

    fn distribute_ships(&mut self) {
        let mut rng = rand::thread_rng();
        let (mut x, mut y);
        let mut is_horizontal: bool;
        let ships_sizes: [usize; 15] =
            [5, 4, 4, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2,];
        for i in ships_sizes {
            (x,y) = (rng.gen_range(0..10), rng.gen_range(0..10));
            is_horizontal = rng.gen_bool(0.5);
            match is_horizontal {
                false => {
                    if i-1 + y < 10 {
                        for j in 0..i {
                            self.cells[x][y+j] = Cell::Ship;
                        };
                    };
                },
                true => {
                    if i-1 + x < 10 {
                        for j in 0..i {
                            self.cells[x+j][y] = Cell::Ship;
                        };
                    };
                },
            };
        };
    }
}


#[derive(Copy)]
#[derive(Clone)]
enum Cell {
    Water,
    Ship,
    Shotted,
}

impl Cell {
    fn chr_sprite(&self) -> char {
        match self {
            Cell::Water => '.',
            Cell::Ship => '@',
            Cell::Shotted => '*',
        }
    }
}

