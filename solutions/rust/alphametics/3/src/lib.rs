use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (addends, sum_word) = parse_input(input)?;

    let mut unique_letters: Vec<char> = vec![];
    for c in input.to_uppercase().chars() {
        if !c.is_alphabetic() || unique_letters.contains(&c) {
            continue;
        }

        unique_letters.push(c);
    }

    // not needed but makes the testrun much faster
    unique_letters.sort_unstable();

    let mut solution: HashMap<char, u8> = HashMap::new();
    for digits in DigitsPermIter::new() {
        for i in 0..unique_letters.len() {
            solution.insert(unique_letters[i], digits[i]);
        }

        if test_solution(&addends, &sum_word, &solution) {
            return Some(solution);
        }
    }

    None
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
            Some(n) => {
                total += n;
                if total > target {
                    return false;
                }
            }
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

/// Iterator over all permutations of digits [0,1,2,...,9]
/// Uses Heap's Algorithm for efficient permutation generation
struct DigitsPermIter {
    digits: [u8; 10],
    c: [usize; 10],
    i: usize,
    first: bool,
}

impl DigitsPermIter {
    fn new() -> Self {
        DigitsPermIter {
            digits: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            c: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            i: 0,
            first: true,
        }
    }
}

impl Iterator for DigitsPermIter {
    type Item = [u8; 10];

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.digits);
        }

        while self.i < self.digits.len() {
            if self.c[self.i] < self.i {
                if self.i.is_multiple_of(2) {
                    self.digits.swap(0, self.i);
                } else {
                    self.digits.swap(self.c[self.i], self.i);
                }

                self.c[self.i] += 1;
                self.i = 0;
                return Some(self.digits);
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}
