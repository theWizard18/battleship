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
        let mut ship_counter = 30;
        let (mut x, mut y);
        while ship_counter != 0{
            (x,y) = (rand::thread_rng().gen_range(0..10),
                rand::thread_rng().gen_range(0..10));
            match self.cells[x][y] {
                Cell::Ship => continue,
                _ => {
                    self.cells[x][y] = Cell::Ship;
                    ship_counter -= 1;
                },
            };
        }
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

