use std::fmt::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

#[derive(Copy, Clone, Eq, PartialEq)]
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

enum Event {
    Tick,
    Input(Key),
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut field = vec![vec![Cell::Empty; 10]; 10];
    field[0][0] = Cell::Block;
    field[5][0] = Cell::Circle;
    draw(&field);

    let (timer_tx, event) = mpsc::channel();
    // let input_tx = timer_tx.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1000));
        timer_tx.send(Event::Tick).expect("tick failed");
    });
    // thread::spawn(move || {
    //     let stdin = std::io::stdin();
    //     for c in stdin.keys() {
    //         let key = c.unwrap_or(Key::Null);
    //         input_tx.send(Event::Input(key)).expect("input failed");
    //     }
    // });
    let mut c = 0;
    loop {
        match event.recv().unwrap() {
            Event::Tick => tick(&mut field, &mut c),
            // Event::Input(key) => match key {
            //     Key::Left => shift(Direction::Left),
            //     Key::Right => shift(Direction::Right),
            //     Key::Up => shift(Direction::Up),
            //     Key::Down => shift(Direction::Down),
            //     Key::Ctrl('c') => break,
            //     _ => (),
            // },
            _ => (),
        }
        draw(&field);
    }
}

fn draw(field: &Vec<Vec<Cell>>) {
    print!("\x1B[2J\x1B[1;1H");
    for row in field {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn tick(field: &mut Vec<Vec<Cell>>, c: &mut usize) {
    field[0][*c] = Cell::Empty;
    *c += 1;
    field[0][*c] = Cell::Block;
}
