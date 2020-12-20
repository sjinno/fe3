// pub fn build_proverb(list: &[&str]) -> String {
//     let num_of_inputs = list.len();

//     if num_of_inputs == 0 {
//         return String::new();
//     }

//     let last_proverb = format!("And all for the want of a {}.", list[0]);

//     if num_of_inputs == 1 {
//         return last_proverb;
//     }

//     let mut rhyme = String::new();
//     for i in 0..num_of_inputs - 1 {
//         rhyme.push_str(&format!(
//             "For want of a {} the {} was lost.\n",
//             list[i],
//             list[i + 1]
//         ));
//     }
//     rhyme.push_str(&last_proverb);

//     rhyme
// }

// Cool solution...
pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|win| format!("For want of a {0} the {1} was lost.", win[0], win[1]))
        .chain(std::iter::once(format!(
            "And all for the want of a {}.",
            list[0]
        )))
        .collect::<Vec<_>>()
        .join("\n")
}
