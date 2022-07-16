use rand::Rng;
use std::io::{self, Write};
pub use crate::{player::Player, cell::Cell};

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
            print!("{}", prompt_message);
            io::stdout().flush().expect("prompt flush error");
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

