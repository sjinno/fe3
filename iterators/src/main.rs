use std::iter::{from_fn, successors};

fn main() {
    // Fibonacci sequence using scan.
    let i = (0..).scan((0, 1), |s, _| {
        let f = s.0;
        *s = (s.1, s.0 + s.1);
        Some(f)
    });
    println!("{:?}", i.take(6).collect::<Vec<_>>());

    // Fibonacci sequence using from_fn.
    let mut f = (0, 1);
    let fib = from_fn(move || {
        let n = f.0;
        f = (f.1, f.0 + f.1);
        Some(n)
    });
    println!("{:?}", fib.take(20).collect::<Vec<_>>());

    // Fibonacci sequence using successors.
    let fibbb = successors(Some((0, 1)), |&(prev, current)| {
        Some((current, current + prev))
    })
    .map(|(prev, _)| prev);
    println!("{:?}", fibbb.take(20).collect::<Vec<_>>());
}
