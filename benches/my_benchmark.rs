use criterion::{black_box, criterion_group, criterion_main, Criterion};

// pub fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("String::new()", |b| b.iter(|| black_box(String::new())));
//     c.bench_function("String::from(\"\")", |b| b.iter(|| black_box(String::from(""))));
//     c.bench_function("\"\".to_string()", |b| b.iter(|| black_box("".to_string())));
//     c.bench_function("format!(\"\")", |b| b.iter(|| black_box(format!(""))));
//     c.bench_function("String::default()", |b| b.iter(|| black_box(String::default())));
// }

// pub fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("String::from(\"Hello, world!\")", |b| b.iter(|| black_box(String::from("Hello, world!"))));
//     c.bench_function("\"Hello, world!\".to_string()", |b| b.iter(|| black_box("Hello, world!".to_string())));
//     c.bench_function("format!(\"Hello, world!\")", |b| b.iter(|| black_box(format!("Hello, world!"))));
//     c.bench_function("format!(\"{}\", \"Hello, world!\")", |b| b.iter(|| black_box(format!("{}", "Hello, world!"))));
// }

// pub fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("&str to String", |b| b.iter(|| {
//         let s = black_box("Hello, world!");
//         let s1 = black_box(s.to_string());
//         black_box(s1);
//     }));
//     c.bench_function("String to &str", |b| b.iter(|| {
//         let s = black_box(String::from("Hello, world!"));
//         let s1 = black_box(s.as_str());
//         black_box(s1);
//     }));
// }

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Vec::new()", |b| b.iter(|| black_box(Vec::<usize>::new())));
    c.bench_function("Vec::defualt()", |b| b.iter(|| black_box(Vec::<usize>::default())));
    // c.bench_function("vec![]", |b| b.iter(|| black_box(vec![])));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
