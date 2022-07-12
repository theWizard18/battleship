use rand::Rng;
use std::io::{self, Write};
pub use crate::cell::Cell;

struct Player {
    name: String,
    grid: Grid,
    attack: Option<(usize, usize)>,
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
        self.gen_human_move();
        self.gen_machine_move();
    }

    fn gen_human_move(&mut self) {
        let x = self.input_coord( true,
            "Enter the x coordinate (letters): ".to_string());
        let y = self.input_coord( false,
            "Enter the y coordinate (numbers): ".to_string());
        self.human.attack = Some((x, y));
    }

    fn input_coord(&self, is_xcoord: bool, prompt_message: String) -> usize {
        let mut input = String::new();
        let mut result: char;
        loop {
            println!("{}", prompt_message);
//            io::stdout().flush().expect("prompt flush error");
            match io::stdin().read_line(&mut input) {
                Ok(_) => result = input.remove(0),
                Err(_) => continue,
            };

            if is_xcoord {
                match result.is_ascii_alphabetic() {
                    true => result as usize - 65,
                    false => continue,
                }
            } else {
                match result.is_numeric() {
                    true => result as usize,
                    false => continue,
                }
            };
        }
    }

    fn gen_machine_move(&mut self) {
        let mut rng = rand::thread_rng();
        let (mut x, mut y) :(usize, usize);
        loop {
            (x,y) = (rng.gen_range(0..10), rng.gen_range(0..10));
            if self.human.grid.cells[x][y] != Cell::Shotted {
                break;
            }
        };
        self.machine.attack = Some((x, y));
    }

    fn print(&self) {
        self.machine.grid.print();
        self.human.grid.print();
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

