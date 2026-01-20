pub fn square(s: u32) -> u64 {
    assert!(s >= 1);
    assert!(s <= 64);

    1_u64 << (s - 1)
}

pub fn total() -> u64 {
    ((1_u128 << 65) - 1) as u64
}
