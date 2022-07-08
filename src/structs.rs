use rand::Rng;
use std::io::{self, Write};

struct Player {
    name: String,
    grid: Grid,
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            grid: Grid::new(false),
        }
    }
}

pub struct Game {
    human :Player,
    machine :Player,
}
impl Game {
    pub fn new() -> Game {
        Game {
            human: Player::new("player".to_string()),
            machine: Player::new("machine".to_string()),
        }
    }
    
    pub fn turn(&mut self) {
        self.print();
        self.human_input();
    }

    fn human_input(&self) -> (char, char) {
        let x = self.input_processing(
            "enter the x coordinate (letters): ".to_string());
        let y = self.input_processing(
            "enter the y coordinate (numbers): ".to_string());
        (x, y)
    }

    fn input_processing(&self, prompt_message: String) -> char {
        let mut input = String::new();
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut result: char;
        loop {
            print!("{}", prompt_message);
            stdout.flush().expect("prompt flush error");
            match stdin.read_line(&mut input) {
                Ok(_) => result =  input.remove(0),
                Err(_) => continue,
            };
            match result.is_ascii_alphanumeric() {
                true => self.input_interpreter(result),
                false => continue,
            };
        }
    }

    fn input_interpreter(&self, coords :(char, char)) -> (usize, usize) {
        (coords.0 as usize -65, coords.1 as usize)
    }

    fn print(&self) {
        self.machine.grid.print();
        self.human.grid.print();
    }
    
    fn enemy_move(&mut self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let (mut x, mut y) :(usize, usize);
        loop {
            (x,y) = (rng.gen_range(0..10), rng.gen_range(0..10));
            if self.human.grid.cells[x][y] != Cell::Shotted {
                break;
            }
        };
        (x, y)
    }
}

struct Grid {
    cells :[[Cell; 10]; 10],
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
        for i in self.ships_sizes {
            loop {
                (x,y) = (rng.gen_range(0..9), rng.gen_range(0..9));
                is_horizontal = rng.gen_bool(0.5);
                if self.ship_distribution(is_horizontal, i, x, y) {
                    break;
                };
            };
        };
    }

    fn ship_distribution(&mut self, is_horizontal, i, x, y) -> bool {
        match is_horizontal {
            false => {
                if i-1 + y < 10 && !self.cells[x][y..y+i].iter()
                        .any(|&cell| cell == Cell::Ship) {
                    for j in 0..i {
                        self.cells[x][y+j] = Cell::Ship;
                    };
                    true
                };
            },
            true => {
                if i-1 + x < 10 && !self.cells[x..x+i].iter()
                        .any(|&row| row[y] == Cell::Ship) {
                    for j in 0..i {
                        self.cells[x+j][y] = Cell::Ship;
                    };
                    true
                };
            },
        };
        false
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

