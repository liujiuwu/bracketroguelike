use ch02_helloecs_life::cell::Cell;

struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}