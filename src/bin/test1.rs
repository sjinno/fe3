use bencher::black_box;
use perf_event::{Builder, Counter};
use std::io::Result;

fn main() -> std::io::Result<()> {
    let mut counter = Builder::new().build()?;

    // Initialize an empty string:
    println!("Comaprison of number of instructions of initializing an empty string:\n");
    for i in 0..7 {
        counter.enable()?;
        let s = match i {
            0 | 1 | 2 => String::new(),
            3 => String::from(""),
            4 => "".to_string(),
            5 => format!(""),
            6 => String::default(),
            _ => break,
        };
        counter.disable()?;

        println!("{}: {}", i, s);
        println!("{} instructions retired\n", counter.read()?);
        counter.reset()?;
    }

    // Initialize "hello":
    println!("=========================");
    println!("Comaprison of number of instructions of initializing a non-empty string:\n");
    for i in 0..6 {
        counter.enable()?;
        let hello = match i {
            0 | 1 | 2 => String::from("hello"),
            3 => "hello".to_string(),
            4 => format!("hello"),
            5 => format!("{}", "hello"),
            _ => break,
        };
        counter.disable()?;

        println!("{}: {}", i, hello);
        println!("{} instructions retired\n", counter.read()?);
        counter.reset()?;
    }

    // Convert String to &str:
    println!("=========================");
    println!("String to &str:");
    let insns = count(&mut counter, &format!("hello"), |s| s.as_str())?;
    println!("{} instructions retired", insns);

    // Convert &str to String:
    println!("=========================");
    println!("&str to String:");
    let insns = count(&mut counter, &"hello", |s| format!("{}", s))?;
    println!("{} instructions retired", insns);

    // Initialize an empty vector:
    println!("=========================");
    println!("Comaprison of number of instructions of initializing an empty vector:\n");
    for i in 0..6 {
        counter.enable()?;
        counter.reset()?;
        let s = match i {
            0 | 1 | 2 => Vec::<usize>::new(),
            3 => vec![],
            4 => Vec::default(),
            _ => break,
        };
        counter.disable()?;

        println!("{}: {:?}", i, s);
        println!("{} instructions retired\n", counter.read()?);
    }

    println!("=========================");
    counter.enable()?;
    counter.reset()?;
    |v: usize| v;
    counter.disable()?;
    println!("{} instructions retired\n", counter.read()?);
    counter.reset()?;

    println!("=========================");
    counter.enable()?;
    empty();
    counter.disable()?;
    println!("{} instructions retired\n", counter.read()?);
    counter.reset()?;

    println!("=========================");
    counter.enable()?;
    almost_empty(0);
    counter.disable()?;
    println!("{} instructions retired\n", counter.read()?);
    counter.reset()?;

    Ok(())
}

fn count<F, In, Out>(counter: &mut Counter, v: In, body: F) -> Result<u64>
where
    F: FnOnce(In) -> Out,
{
    counter.enable()?;
    counter.reset()?;
    black_box(body(black_box(v)));
    counter.disable()?;
    counter.read()
}

fn empty() {}

fn almost_empty(v: usize) {}

// fn main() {
//     ...
//     let insns = count(&mut counter, &format!("hello"), |s| {
//         s.as_str()
//     })?;
//     println!("{} instructions retired", insns);
// }
