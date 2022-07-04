enum State {
    Water,
    Ship,
    Destroyed,
}


struct Cell {
    state :State,
}

struct Grid {
    cells :[[Cell; 9]; 9],
}


fn main() {
    println!("Hello, world!");
}
