pub fn egg_count(display_value: u32) -> usize {
    (0..u32::BITS)
        .filter(|pos| (display_value >> pos) & 1 == 1)
        .count()
}
