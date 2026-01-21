use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&f| f > 0)
        .flat_map(|f| {
            (1..)
                .take_while(|x| (x * f) < limit)
                .map(|x| x * f)
                .collect::<Vec<u32>>()
        })
        .collect::<HashSet<u32>>()
        .iter()
        .sum()
}
