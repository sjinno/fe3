use std::{thread, time};

pub fn sierpinski(n: i32, rate: u64) {
    // let ten_millis = time::Duration::from_millis(10);
    // let now = time::Instant::now();
    // thread::sleep(ten_millis);
    // assert!(now.elapsed() >= ten_millis);

    let mut y = n;
    let mut i = 0;
    let mut x = 0;
    while y >= 0 {
        while i < y {
            print!(" ");
            i += 1;
        }
        while x + y < n {
            thread::sleep(time::Duration::from_millis(y as u64 / rate));
            if (x & y) != 0 {
                print!("  ");
            } else {
                print!("* ");
            }
            x += 1;
        }
        println!();
        if y == 0 {
            thread::sleep(time::Duration::from_millis(800));
        }
        y -= 1;
        i = 0;
        x = 0;
    }
    // print!("\x1B[2J");
    // print!("{}[2J", 27 as char);
}
