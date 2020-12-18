// pub fn verse(n: u32) -> String {
//     match n {
//         0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
//         1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
//         2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
//         _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1).to_string(),
//     }
// }

// pub fn sing(start: u32, end: u32) -> String {
//     let mut lyrics = String::new();
//     for v in (end + 1..=start).rev() {
//         let mut verse = verse(v);
//         verse.push_str(&"\n");
//         lyrics.push_str(&verse);
//     }
//     lyrics.push_str(&verse(end));
//     lyrics
// }

// One of the community solutions that I find useful.
const VERSES: [&'static str; 3] = [
    "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
    "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
    "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"
];

pub fn verse(n: u16) -> String {
    match n {
        0 => VERSES[0].to_string(),
        1 => VERSES[1].to_string(),
        2 => VERSES[2].to_string(),
        _ => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\n\
                   Take one down and pass it around, {1} bottles of beer \
                   on the wall.\n",
            n,
            n - 1
        ),
    }
}

pub fn sing(from: u16, to: u16) -> String {
    (to..from + 1)
        .rev()
        .map(|x| verse(x))
        .collect::<Vec<_>>()
        .join("\n")
}
