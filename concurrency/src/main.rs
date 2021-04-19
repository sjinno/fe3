use std::fmt;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Block,
    Circle,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => f.write_str("・"),
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
    let mut c = 0;
    let mut row = 5;
    let mut col = 0;

    field[c][c] = Cell::Block;
    field[row][col] = Cell::Circle;

    let mut stdout = stdout().into_raw_mode().unwrap();
    let state = get_current_state(&field);

    write!(
        stdout,
        "{}{}row = {}, col = {}",
        cursor::Hide,
        state,
        row,
        col,
    )
    .unwrap();
    stdout.flush().unwrap();

    let (timer_tx, event) = mpsc::channel();
    let input_tx = timer_tx.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1000));
        timer_tx.send(Event::Tick).expect("tick failed");
    });
    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            let key = c.unwrap_or(Key::Null);
            input_tx.send(Event::Input(key)).expect("input failed");
        }
    });

    loop {
        match event.recv().unwrap() {
            Event::Tick => tick(&mut field, &mut c),
            Event::Input(key) => match key {
                Key::Left => shift(&mut field, Direction::Left, &mut row, &mut col),
                Key::Right => shift(&mut field, Direction::Right, &mut row, &mut col),
                Key::Up => shift(&mut field, Direction::Up, &mut row, &mut col),
                Key::Down => shift(&mut field, Direction::Down, &mut row, &mut col),
                Key::Char('q') => break,
                _ => (),
            },
        }
        let state = get_current_state(&field);
        write!(stdout, "{}row = {}, col = {}", state, row, col,).unwrap();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", cursor::Show).unwrap();
}

fn get_current_state(v: &Vec<Vec<Cell>>) -> String {
    let mut state = "\x1B[2J\x1B[1;1H".to_string();
    for row in v {
        for col in row {
            let f = format!("{}", col);
            state.push_str(&f);
        }
        state.push_str("\n\r");
    }
    state
}

fn tick(field: &mut Vec<Vec<Cell>>, c: &mut usize) {
    field[0][*c] = Cell::Empty;
    *c += 1;
    field[0][*c] = Cell::Block;
}

fn shift(field: &mut Vec<Vec<Cell>>, d: Direction, r: &mut usize, c: &mut usize) {
    field[*r][*c] = Cell::Empty;
    match d {
        Direction::Up => *r -= 1,
        Direction::Down => *r += 1,
        Direction::Left => *c -= 1,
        Direction::Right => *c += 1,
    }
    field[*r][*c] = Cell::Circle;
}
