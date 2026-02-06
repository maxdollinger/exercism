use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Option<(Vec<Vec<u8>>, Vec<u8>)> {
    let parts = match input.to_uppercase().split_once("==") {
        Some((fst, snd)) => (
            fst.split('+')
                .map(|a| {
                    a.trim()
                        .chars()
                        .filter(|c| c.is_ascii_alphabetic())
                        .map(|c| c as u8)
                        .collect::<Vec<u8>>()
                })
                .filter(|a| !a.is_empty())
                .collect::<Vec<Vec<u8>>>(),
            snd.trim()
                .chars()
                .filter(|c| c.is_ascii_alphabetic())
                .map(|c| c as u8)
                .collect::<Vec<u8>>(),
        ),
        _ => return None,
    };

    if parts.0.len() < 2 || parts.1.is_empty() {
        return None;
    }

    Some(parts)
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (addends, sum_word) = parse_input(input)?;

    let mut letters: Vec<u8> = addends
        .clone()
        .into_iter()
        .flatten()
        .chain(sum_word.clone())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    letters.sort();

    const DIGITS: &[u8; 10] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    brutforce_combination(&letters, DIGITS, &addends, &sum_word)
        .map(|v| v.iter().map(|e| (*e.0 as char, *e.1)).collect())
}

/// Validates if a given solution (mapping of letters to digits) satisfies the equation
fn validate_solution(addends: &[Vec<u8>], sum: &[u8], solution: &HashMap<u8, u8>) -> bool {
    let target = match word_to_number(sum, solution) {
        None => return false,
        Some(n) => n,
    };

    let mut total: u64 = 0;
    for addend in addends {
        match word_to_number(addend, solution) {
            None => return false,
            Some(n) => total += n,
        }
    }

    total == target
}

fn word_to_number(word: &[u8], mapping: &HashMap<u8, u8>) -> Option<u64> {
    word.iter()
        .enumerate()
        .try_fold(0u64, |number, (i, letter)| {
            mapping.get(letter).and_then(|digit| {
                if i == 0 && *digit == 0 {
                    None
                } else {
                    Some(number * 10 + (*digit as u64))
                }
            })
        })
}

fn next_permutation(a: &mut [usize]) -> bool {
    if a.len() < 2 {
        return false;
    }

    let mut i = a.len() - 2;
    while i > 0 && a[i] >= a[i + 1] {
        i -= 1;
    }
    if i == 0 && a[0] >= a[1] {
        return false;
    }
    let mut j = a.len() - 1;
    while a[j] <= a[i] {
        j -= 1;
    }
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}

fn next_combination(c: &mut [usize], n: usize) -> bool {
    let k = c.len();
    if k == 0 {
        return false;
    }
    for i in (0..k).rev() {
        let max_val = n - (k - i);
        if c[i] < max_val {
            c[i] += 1;
            for j in i + 1..k {
                c[j] = c[j - 1] + 1;
            }
            return true;
        }
    }
    false
}

fn brutforce_combination(
    a: &[u8],
    b: &[u8; 10],
    addends: &[Vec<u8>],
    sum_word: &[u8],
) -> Option<HashMap<u8, u8>> {
    let k = a.len();
    let n = b.len();
    if k > n {
        return None;
    }

    let mut comb: Vec<usize> = (0..k).collect(); // chosen indices into b
    let mut pairs: HashMap<u8, u8> = HashMap::new(); // reused output buffer

    loop {
        let mut perm = comb.clone(); // permute the chosen indices

        loop {
            for i in 0..k {
                pairs.insert(a[i], b[perm[i]]);
            }

            if validate_solution(addends, sum_word, &pairs) {
                return Some(pairs);
            }

            if !next_permutation(&mut perm) {
                break;
            }
        }

        if !next_combination(&mut comb, n) {
            break;
        }
    }

    None
}
