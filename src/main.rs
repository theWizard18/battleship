mod game;
mod cell;
mod player;
mod grid;

pub use crate::game::Game;

fn main() {
    let mut game :Game = Game::new();
    game.turn();
}
