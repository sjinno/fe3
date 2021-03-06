use std::fmt;
use std::io::{stdin, stdout, Write};

use rand::{thread_rng, Rng};
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Clone)]
enum Square {
    Empty,
    Taken,
    HiddenTreasure,
    Found,
}

impl fmt::Display for Square {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Empty | Square::HiddenTreasure => fmt.write_str("・"),
            // Square::HiddenTreasure => fmt.write_str("★ "),
            Square::Taken => fmt.write_str("■ "),
            Square::Found => fmt.write_str("● "),
        }
    }
}

fn main() {
    let mut rng = thread_rng();

    let mut v = vec![vec![Square::Empty; 10]; 10];
    let mut row = 0;
    let mut col = 0;
    v[row][col] = Square::Taken;

    let r = rng.gen_range(0..10);
    let c = rng.gen_range(0..10);
    v[r][c] = Square::HiddenTreasure;

    let state = get_current_state(&v);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

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

    for c in stdin.keys() {
        v[row][col] = Square::Empty;
        match c.unwrap() {
            Key::Up => {
                if row > 0 {
                    row -= 1;
                }
            }
            Key::Down => {
                if row < 9 {
                    row += 1;
                }
            }
            Key::Left => {
                if col > 0 {
                    col -= 1;
                }
            }
            Key::Right => {
                if col < 9 {
                    col += 1;
                }
            }
            Key::Char('q') => break,
            _ => (),
        }

        match v[row][col] {
            Square::HiddenTreasure => {
                v[row][col] = Square::Found;
                let state = get_current_state(&v);
                write!(
                    stdout,
                    "{}row = {}, col = {}\n\rHidden treasure found!",
                    state, row, col,
                )
                .unwrap();
                stdout.flush().unwrap();
                break;
            }
            _ => {
                v[row][col] = Square::Taken;
                let state = get_current_state(&v);
                write!(stdout, "{}row = {}, col = {}", state, row, col,).unwrap();
                stdout.flush().unwrap();
            }
        }
    }

    write!(stdout, "{}", cursor::Show).unwrap();
}

fn get_current_state(v: &Vec<Vec<Square>>) -> String {
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
