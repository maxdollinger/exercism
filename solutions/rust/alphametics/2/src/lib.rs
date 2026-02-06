use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (addends, sum_word) = parse_input(input)?;

    let mut unique_letters: Vec<char> = input
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    unique_letters.sort();

    if unique_letters.len() > 10 {
        return None;
    }

    let mut solution: HashMap<char, u8> = HashMap::new();
    let valiator = |digits: &[u8]| -> bool {
        for i in 0..unique_letters.len() {
            solution.insert(unique_letters[i], digits[i]);
        }

        test_solution(&addends, &sum_word, &solution)
    };

    if find_digit_permutation(valiator) {
        Some(solution)
    } else {
        None
    }
}

fn parse_input(input: &str) -> Option<(Vec<Vec<char>>, Vec<char>)> {
    let parts = match input.to_uppercase().split_once("==") {
        Some((fst, snd)) => (
            fst.split('+')
                .map(|a| a.trim().chars().collect::<Vec<char>>())
                .filter(|a| !a.is_empty())
                .collect::<Vec<Vec<char>>>(),
            snd.trim().chars().collect::<Vec<char>>(),
        ),
        _ => return None,
    };

    if parts.0.len() < 2 || parts.1.is_empty() {
        return None;
    }

    Some(parts)
}

fn test_solution(addends: &[Vec<char>], sum_word: &[char], solution: &HashMap<char, u8>) -> bool {
    let target = match word_to_number(sum_word, solution) {
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

fn word_to_number(word: &[char], mapping: &HashMap<char, u8>) -> Option<u64> {
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

fn find_digit_permutation(mut validator: impl FnMut(&[u8]) -> bool) -> bool {
    let mut digits: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    if validator(&digits) {
        return true;
    }

    let mut i = 0;
    let mut c = vec![0; digits.len()];
    while i < digits.len() {
        if c[i] < i {
            if i % 2 == 0 {
                digits.swap(0, i);
            } else {
                digits.swap(c[i], i);
            }

            if validator(&digits) {
                return true;
            }

            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    false
}
