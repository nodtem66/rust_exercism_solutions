use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_sublist(smaller: &[i32], larger: &[i32]) -> bool {
    smaller.is_empty()
        || larger
            .windows(smaller.len())
            .any(|window| window == smaller)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match &first_list.len().cmp(&second_list.len()) {
        Ordering::Equal => (first_list == second_list)
            .then(|| Comparison::Equal)
            .unwrap_or(Comparison::Unequal),
        Ordering::Less => is_sublist(first_list, second_list)
            .then(|| Comparison::Sublist)
            .unwrap_or(Comparison::Unequal),
        Ordering::Greater => is_sublist(second_list, first_list)
            .then(|| Comparison::Superlist)
            .unwrap_or(Comparison::Unequal),
    }
}
