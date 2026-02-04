pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ => {
            let next = if n.is_multiple_of(2) {
                n / 2
            } else {
                n * 3 + 1
            };
            collatz(next).map(|steps| steps + 1)
        }
    }
}
