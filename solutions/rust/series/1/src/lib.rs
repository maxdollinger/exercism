pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|win| win.iter().collect())
        .collect()
}
