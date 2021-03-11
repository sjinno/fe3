#[macro_export]
macro_rules! printf {
    ($v:expr) => {
        println!("{:#?}", $v);
    };
}

fn main() {}
