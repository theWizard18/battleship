mod game;
mod cell;
mod player;
mod grid;

pub use crate::game::Game;

fn main() {
    let mut game :Game = Game::new();
    while game.human.grid.has_ships() && game.machine.grid.has_ships() {
        game.turn();
    };
    game.print();
    match (game.human.grid.has_ships(), game.machine.grid.has_ships()) {
        (true, false) => println!("CONGLATULATIONS !!! YOU WON !!!!"),
        (false, true) => println!("Too bad, you lost"),
        (false, false) => println!("It's a draw."),
        (_,_)=> println!("Uh-oh, something's wrong."),
    };
}
