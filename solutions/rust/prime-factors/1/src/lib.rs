pub fn factors(mut n: u64) -> Vec<u64> {
    if n < 2 {
        return Vec::new();
    }

    let twos = n.trailing_zeros() as usize;
    n >>= twos;

    let mut factors = vec![2; twos];

    let mut d = 3u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            factors.push(d);
            n /= d;
        } else {
            d += 2;
        }
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}
