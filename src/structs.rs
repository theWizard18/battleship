use rand::Rng;

pub struct Grid {
    cells :[[Cell; 10]; 10],
    ships_sizes :[usize; 5],
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
            ships_sizes: [5, 4, 3, 3, 2,],
        };
        grid.distribute_ships();
        grid
    }

    fn distribute_ships(&mut self) {
        let mut rng = rand::thread_rng();
        let (mut x, mut y);
        let mut is_horizontal: bool;
        for i in self.ships_sizes {
            loop {
                (x,y) = (rng.gen_range(0..9), rng.gen_range(0..9));
                println!("x = {}, y = {}, i = {}", x, y, i);
                is_horizontal = rng.gen_bool(0.5);
                match is_horizontal {
                    false => {
                        println!("false debug");
                        if i-1 + y < 10 && !self.cells[x][y..y+i]
                                .contains(&Cell::Ship) {
                            for j in 0..i {
                                self.cells[x][y+j] = Cell::Ship;
                            };
                            break;
                        };
                    },
                    true => {
                        println!("true debug");
                        if i-1 + x < 10 && !self.cells[x..x+i]
                                .contains(&Cell::Ship) {
                            for j in 0..i {
                                self.cells[x+j][y] = Cell::Ship;
                            };
                            break;
                        };
                    },
                };
            };
        };
    }
}


#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
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

