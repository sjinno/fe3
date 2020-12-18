#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let len1 = _first_list.len();
    let len2 = _second_list.len();

    if len1 == len2 {
        if _first_list == _second_list {
            return Comparison::Equal;
        }
        return Comparison::Unequal;
    }

    if len2 < len1 {
        if len2 == 0 {
            return Comparison::Superlist;
        }
        if is_sublist(_second_list, _first_list, len2, len1, 0) {
            return Comparison::Superlist;
        }
    }

    if len1 == 0 && len2 != 0 {
        return Comparison::Sublist;
    }

    if is_sublist(_first_list, _second_list, len1, len2, 0) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(
    first_list: &[T],
    second_list: &[T],
    len1: usize,
    len2: usize,
    mut p2: usize,
) -> bool {
    // Initialize a pointer for first_list.
    let mut p1 = 0;

    // Set pointer p2 to point the value that is equivalent to first_list[p1] if any.
    while first_list[p1] != second_list[p2] {
        p2 += 1;
        if p2 == len2 {
            return false;
        }
    }

    // Return true if first_list is a sublist of second_list.
    // Return false if they are unequal.
    while first_list[p1] == second_list[p2] {
        p1 += 1;
        p2 += 1;
        if p1 == len1 {
            return true;
        } else if p2 == len2 {
            return false;
        }
    }

    // To check recurring values, call is_sublist function recursively.
    if p2 != len2 {
        return is_sublist(first_list, second_list, len1, len2, p2 - 1);
    }

    false
}
