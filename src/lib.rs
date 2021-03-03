mod enums;
mod macros;
// mod from_iters;
mod num_fmt;
mod sierpinski;
mod threads;

use enums::*;
use num_fmt::*;
use threads::*;

#[allow(unused)]
fn main() {
    // sierpinski::sierpinski(8, 16);
    // let mut c = 0;
    // while c < 10 {
    //     print!("\x1B[2J\x1B[1;1H");
    //     sierpinski::sierpinski(64, 12);
    //     c += 1;
    // }

    // from_iters::play_with_from_iter();

    // let foo = FBB::Foo;
    // printf!(foo);

    // num_fmt("12213123.435");

    thread1();
}


#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}