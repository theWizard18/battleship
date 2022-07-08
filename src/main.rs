mod structs;
pub use crate::structs::Game;

fn main() {
    let game :Game = Game::new();
    game.turn();
}
