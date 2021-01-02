pub fn num_fmt(num: &str) {
    // let s = num.to_string();
    let split: Vec<&str> = num.split(".").collect();
    let ls = split.len();
    println!("{:#?}", split);
    let integer = split[0];
    let l = integer.len();
    let mut fmt = String::new();
    integer.chars().into_iter().enumerate().for_each(|(i, x)| {
        if i != 0 && (l - i).rem_euclid(3) == 0 {
            fmt.push_str("_");
        }
        fmt.push_str(&x.to_string());
    });
    if ls == 2 {
        fmt.push_str(".");
        fmt.push_str(split[1]);
    }
    println!("{:#?}", fmt);

    // for (i, c) in integer.chars().enumerate() {
    //     if i != 0 && (l - i).rem_euclid(3) == 0 {
    //         fmt.push_str("_");
    //     }
    //     fmt.push_str(&c.to_string());
    // }
    // fmt.push_str(".");
    // fmt.push_str(split[1]);
    // println!("{:#?}", fmt);
}
