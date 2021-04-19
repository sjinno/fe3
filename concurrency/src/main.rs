use std::fmt::{self, Write};
use std::sync::mpsc;

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Block,
    Circle,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => f.write_char('・'),
            Cell::Block => f.write_str("■ "),
            Cell::Circle => f.write_str("● "),
        }
    }
}
fn main() {
    let mut field: Field = vec![vec![Cell::Empty; 10]; 10];
    field[0][0] = Cell::Block;
    field[5][0] = Cell::Circle;
    draw(field);
}

type Field = Vec<Vec<Cell>>;

fn draw(field: Field) {
    for row in field {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}
