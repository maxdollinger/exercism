pub fn square(s: u32) -> u64 {
    assert!((1..=64).contains(&s));

    1_u64 << (s - 1)
}

pub fn total() -> u64 {
    ((1_u128 << 65) - 1) as u64
}
