use rand::Rng;
use std::io::{self, Write};
pub use crate::player::Player;
use crate::cell::Cell;

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
        println!("\n\n");
        self.human.process_attacked_cell(&mut self.machine, y, x);
    }

    fn input_coord(&self, is_xcoord: bool, prompt_message: String) -> usize {
        loop {
            let mut input = String::new();
            print!("{}", prompt_message);
            io::stdout().flush().expect("prompt flush error");
            let result: char = match io::stdin().read_line(&mut input) {
                Ok(_) => input.remove(0),
                Err(_) => {
                    println!("error reading line");
                    continue;
                },
            };

            if is_xcoord {
                match (65..75).contains(&(result as u8)) {
                    true => {
                        return result as usize - 65;
                    },
                    false => {
                        println!("error converting alphabetic input.");
                        println!("Are you sure you entered an uppercase A-J letter?");
                        continue;
                    },
                }
            } else {
                match result.is_numeric() {
                    true => return result as usize - 48,
                    false => {
                        println!("error converting numeric input.");
                        println!("Are you sure you entered a number?");
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
            match self.human.grid.cells[y][x] {
                Cell::StrikedShip | Cell::StrikedWater => continue,
                _ => break,
            }
        };
        self.machine.process_attacked_cell(&mut self.human, y, x);
    }


    pub fn print(&self) {
        self.machine.grid.print();
        self.human.grid.print();
    }
}

