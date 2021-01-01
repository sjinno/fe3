pub fn num_fmt() {
    let num = 1221321333456.213123;
    let s = num.to_string();
    let split: Vec<&str> = s.split(".").collect();
    println!("{:#?}", split);
    let integer = split[0];
    let l = integer.len();
    let mut fmt = String::new();

    for (i, c) in integer.chars().enumerate() {
        if i != 0 && (l - i).rem_euclid(3) == 0 {
            fmt.push_str("_");
        }
        fmt.push_str(&c.to_string());
    }
    fmt.push_str(".");
    fmt.push_str(split[1]);
    println!("{:#?}", fmt);
}
