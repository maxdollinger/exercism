pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    (2..)
        .filter(|&x| {
            let limit = (x as f64).sqrt() as u32;
            let is_prime = primes
                .iter()
                .take_while(|&&p| p <= limit)
                .all(|&p| x % p != 0);
            if is_prime {
                primes.push(x);
            }
            is_prime
        })
        .take(n as usize + 1)
        .last()
        .unwrap()
}
