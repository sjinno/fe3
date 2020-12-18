pub fn raindrops(n: usize) -> String {
    let is_factor = |factor| n % factor == 0;
    let mut s = String::new();

    if is_factor(3) {
        s.push_str("Pling")
    };
    if is_factor(5) {
        s.push_str("Plang")
    };
    if is_factor(7) {
        s.push_str("Plong")
    };

    if s.is_empty() {
        return n.to_string();
    }

    s.to_string()

    // let mut s = String::new();
    // let pling_plang_plong = [("Pling", 3), ("Plang", 5), ("Plong", 7)];

    // for i in &pling_plang_plong {
    //     if n % i.1 == 0 {
    //         s.push_str(i.0);
    //     }
    // }

    // if s.is_empty() {
    //     return n.to_string();
    // }

    // s.to_string()
}
