pub fn egg_count(display_value: u32) -> usize {
    (0..32).map(|bit| (display_value >> bit) & 1).sum::<u32>() as usize
}
