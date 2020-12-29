fn main() {
    // Fibonacci sequence using scan.
    let i = (0..10).scan((0, 1), |s, _| {
        let f = s.0;
        *s = (s.1, s.0 + s.1);
        Some(f)
    });
    println!("{:?}", i.collect::<Vec<_>>());

    // Fibonacci sequence using from_fn.
    let mut f = (0, 1);
    let fib = std::iter::from_fn(move || {
        let n = f.0;
        f = (f.1, f.0 + f.1);
        if n < 42 {
            Some(n)
        } else {
            None
        }
    });
    println!("{:?}", fib.collect::<Vec<_>>());
}
