enum Fbb {
    Foo,
    Bar,
    Baz,
}

fn main() {
    let foo: Fbb = Fbb::Foo;
    
    match foo {
        Fbb::Foo => println!("I'm Foo!"),
        Fbb::Bar => println!("I'm Bar!"),
        Fbb::Baz => println!("I'm Baz!"),
    }
}
