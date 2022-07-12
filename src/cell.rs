
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Cell {
    Water,
    Ship,
    Shotted,
}
impl Cell {
    pub fn chr_sprite(&self) -> char {
        match self {
            Cell::Water => '.',
            Cell::Ship => '@',
            Cell::Shotted => '*',
        }
    }
}

