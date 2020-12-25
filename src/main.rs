// mod from_iters;
mod sierpinski;

fn main() {
    // sierpinski::sierpinski(8, 16);

    let mut c = 0;
    while c < 6 {
        print!("\x1B[2J\x1B[1;1H");
        sierpinski::sierpinski(64, 12);
        c += 1;
    }
    // from_iters::play_with_from_iter();
}
