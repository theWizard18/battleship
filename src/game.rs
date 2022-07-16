use rand::Rng;
use std::io::{self, Write};
pub use crate::{player::Player, cell::Cell};

pub struct Game {
    pub human :Player,
    pub machine :Player,
}
impl Game {
    pub fn new() -> Game {
        Game {
            human: Player::new("player".to_string(), false),
            machine: Player::new("machine".to_string(), true),
        }
    }

    pub fn turn(&mut self) {
        self.print();
        self.gen_human_attack();
        self.gen_machine_attack();
    }

    fn gen_human_attack(&mut self) {
        let x = self.input_coord( true,
            "Enter the x coordinate (letters): ".to_string());
        let y = self.input_coord( false,
            "Enter the y coordinate (numbers): ".to_string());
        self.machine.grid.cells[y][x] = Cell::Shotted;
    }

    fn input_coord(&self, is_xcoord: bool, prompt_message: String) -> usize {
        let mut input = String::new();
        let mut result: char;
        loop {
            print!("{}", prompt_message);
            io::stdout().flush().expect("prompt flush error");
            match io::stdin().read_line(&mut input) {
                Ok(_) => result = input.remove(0),
                Err(_) => {
                    println!("error reading line");
                    continue;
                },
            };

            if is_xcoord {
                match result.is_ascii_uppercase() {
                    true => {
                        return result as usize - 65;
                    },
                    false => {
                        println!("error converting alphabetic input.\n
                            Are you sure you entered an uppercase A-J letter?");
                        continue;
                    },
                }
            } else {
                match result.is_numeric() {
                    true => return result as usize - 48,
                    false => {
                        println!("error converting numeric input.\n
                            Are you sure you entered a number?");
                        continue;
                    },
                }
            };
        }
    }

    fn gen_machine_attack(&mut self) {
        let mut rng = rand::thread_rng();
        let (mut x, mut y) :(usize, usize);
        loop {
            (x,y) = (rng.gen_range(0..10), rng.gen_range(0..10));
            if self.human.grid.cells[x][y] != Cell::Shotted {
                break;
            }
        };
        self.human.grid.cells[x][y] = Cell::Shotted;
    }

    fn print(&self) {
        self.machine.grid.print();
        self.human.grid.print();
    }
}

