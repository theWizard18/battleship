mod game;
mod cell;
mod player;
mod grid;

pub use crate::{game::Game, cell::Cell};

fn main() {
    let mut game :Game = Game::new();
    while game.human.grid.cells
        .iter().any( |row| row.iter().any(|y| y.clone() == Cell::Ship)) && 
            game.machine.grid.cells
                .iter().any( |row| row.iter().any(|y| y.clone() == Cell::Ship)) 
        {   // good gracious this is a mess, gonna try to fix it tomorrow
        game.turn();
    };
}
