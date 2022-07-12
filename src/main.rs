mod structs;
mod cell;
pub use crate::structs::Game;

fn main() {
    let mut game :Game = Game::new();
    game.turn();
}
