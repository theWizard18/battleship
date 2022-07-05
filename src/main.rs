
#[derive(Clone)]

enum Cell {
    Water,
    Ship,
    Destroyed,
}

impl Cell {
    fn chr_sprite(&self) -> char {
        match self {
            Cell::Water => '.',
            Cell::Ship => '@',
            Cell::Destroyed => '*',
        }
    }
}


struct Grid {
    cells :Vec<Vec<Cell>>,
}

impl Grid {
    fn print(&self) {
        let mut row_counter = 0;
        for row in &self.cells {
            for cell in row {
                print!(" {}", cell.chr_sprite());
            }
            println!(" {}", row_counter);
            row_counter += 1;
        }
        println!(" A B C D E F G H I");
    }

    fn new_of_size(size :u8) -> Grid {
        let cell :Cell = Cell::Water;
        let mut row :Vec<Cell> = Vec::new();
        let mut column :Vec<Vec<Cell>> = Vec::new();

        for _i in 0..size {
            row.clear();
            for _j in 0..size {
                row.push(cell.clone());
            }
            column.push(row.clone());
        }
        Grid {
            cells: column,
        }
    }
}


fn main() {
    let grid :Grid = Grid::new_of_size(12);
    grid.print();
}

