
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Cell {
    Water,
    Ship,
    StrikedShip,
    StrikedWater,
}
impl Cell {
    pub fn chr_sprite(&self) -> char {
        match self {
            Cell::Water => '.',
            Cell::Ship => '@',
            Cell::StrikedShip => '*',
            Cell::StrikedWater => '~',
        }
    }
}

