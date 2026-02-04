#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(a: &[i32], b: &[i32]) -> Comparison {
    if a == b {
        return Comparison::Equal;
    }

    if a.is_empty() {
        return Comparison::Sublist;
    }

    if b.is_empty() {
        return Comparison::Superlist;
    }

    if a.len() > b.len() && a.windows(b.len()).any(|subs| subs == b) {
        return Comparison::Superlist;
    }

    if a.len() < b.len() && b.windows(a.len()).any(|subs| subs == a) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}
