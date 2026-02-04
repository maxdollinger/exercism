use std::iter::successors;

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let steps = successors(Some(n), |&n| {
        Some(if n & 1 == 0 { n / 2 } else { n * 3 + 1 })
    })
    .take_while(|&n| n > 1)
    .count();

    Some(steps as u64)
}
