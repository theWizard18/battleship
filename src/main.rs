mod structs;
pub use crate::structs::Grid;

fn main() {
    let grid :Grid = Grid::new();
    grid.print();
}
