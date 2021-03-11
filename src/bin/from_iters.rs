use std::iter::FromIterator;

// Playing with FromIterator.
pub fn play_with_from_iter() {
    // let s: &str = "*.*";
    // let col_len = s.len();
    // let zeros = std::iter::repeat(0).take(col_len);
    // let mut v = Vec::from_iter(zeros);
    // println!("{:?}", v);
    // s.chars().into_iter()
    //         .enumerate()
    //         .for_each(|(i, x)| {
    //           if x == '*' { v[i] = 10; }
    //         });
    // println!("{:?}", v);
    let s: &[&str] = &["*.*", "*.*", "*.*"];
    let row_len = s[0].len();
    let col_len = s.len();
    // println!("{}, {}", row_len, col_len);
    let z = std::iter::repeat(0).take(row_len);
    let vr = Vec::from_iter(z);
    let zeros = std::iter::repeat(vr).take(col_len);
    let mut v = Vec::from_iter(zeros);
    println!("{:?}", v);
    s.iter().enumerate().for_each(|(row, x)| {
        x.chars().into_iter().enumerate().for_each(|(col, y)| {
            if y == '*' {
                v[row][col] = 10;
            }
        })
    });
    change_values_in_v(&mut v);
    println!("{:?}", v);
}

// Change values in v.
fn change_values_in_v(v: &mut Vec<Vec<usize>>) {
    v.into_iter()
        .for_each(|row| row.into_iter().for_each(|col| *col += 1));
}

fn main() {}
