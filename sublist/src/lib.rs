// #[derive(Debug, PartialEq)]
// pub enum Comparison {
//     Equal,
//     Sublist,
//     Superlist,
//     Unequal,
// }

// pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
//     let len1 = first_list.len();
//     let len2 = second_list.len();

//     if len1 == len2 {
//         if first_list == second_list {
//             return Comparison::Equal;
//         }
//         return Comparison::Unequal;
//     }

//     if len2 < len1 {
//         if len2 == 0 {
//             return Comparison::Superlist;
//         }
//         if is_sublist(second_list, first_list, len2, len1, 0) {
//             return Comparison::Superlist;
//         }
//     }

//     if len1 == 0 && len2 != 0 {
//         return Comparison::Sublist;
//     }

//     if is_sublist(first_list, second_list, len1, len2, 0) {
//         return Comparison::Sublist;
//     }

//     Comparison::Unequal
// }

// fn is_sublist<T: PartialEq>(
//     first_list: &[T],
//     second_list: &[T],
//     len1: usize,
//     len2: usize,
//     mut p2: usize,
// ) -> bool {
//     // Initialize a pointer for first_list.
//     let mut p1 = 0;

//     // Set pointer p2 to point the value that is equivalent to first_list[p1] if any.
//     while first_list[p1] != second_list[p2] {
//         p2 += 1;
//         if p2 == len2 {
//             return false;
//         }
//     }

//     // Return true if first_list is a sublist of second_list.
//     // Return false if they are unequal.
//     while first_list[p1] == second_list[p2] {
//         p1 += 1;
//         p2 += 1;
//         if p1 == len1 {
//             return true;
//         } else if p2 == len2 {
//             return false;
//         }
//     }

//     // To check recurring values, call is_sublist function recursively.
//     if p2 != len2 {
//         return is_sublist(first_list, second_list, len1, len2, p2 - 1);
//     }

//     false
// }

// cargo test -- --ignored  0.24s user 0.06s system 146% cpu 0.205 total
// cargo test -- --ignored  0.68s user 0.23s system 167% cpu 0.540 total
// cargo test -- --ignored  0.26s user 0.05s system 167% cpu 0.190 total

// cargo test -- --ignored  0.29s user 0.05s system 158% cpu 0.214 total
// cargo test -- --ignored  0.31s user 0.05s system 164% cpu 0.216 total
// cargo test -- --ignored  0.24s user 0.05s system 143% cpu 0.200 total
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T: Eq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => {
            if a.windows(n).any(|v| v == b) {
                Superlist
            } else {
                Unequal
            }
        }
        (m, n) if m < n => {
            if b.windows(m).any(|v| v == a) {
                Sublist
            } else {
                Unequal
            }
        }
        (_, _) => {
            if a == b {
                Equal
            } else {
                Unequal
            }
        }
    }
}
