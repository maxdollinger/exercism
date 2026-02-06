use std::collections::HashMap;
use std::collections::HashSet;

/// Solves alphametics puzzles by converting to polynomial coefficients
///
/// Example: "I + BB == ILL" becomes:
/// - I has coefficient -99 (beause -100I + 1I)
/// - B has coefficient 11 (because 1B + 10B = 11B)
/// - L has coefficient -11 (because -1L + -10L = -11L)
///
/// So se equation to solve becomes: -99×I_digit + 11×B_digit + (-11)×L_digit = 0
/// The solution to this is then bruteforced by creating all possible permutations of 0..9 and
/// matching them with the letters
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let letter_coefficients = parse_equation_to_coefficients(input);

    for digits in DigitsPermutations::new() {
        if verify_solution(&digits, &letter_coefficients) {
            return Some(
                letter_coefficients
                    .iter()
                    .map(|trp| trp.0)
                    .zip(digits)
                    .collect(),
            );
        }
    }

    None
}

/// Parses the equation string and computes polynomial coefficients
/// Returns: Vec of (letter, coefficient, is_leading_digit)
fn parse_equation_to_coefficients(input: &str) -> Vec<(char, i64, bool)> {
    let mut letter_to_coefficient: HashMap<char, i64> = HashMap::new();
    let mut leading_letters: HashSet<char> = HashSet::new();
    let mut previous_letter = ' ';
    let mut positional_value = -1; // -1 for left side of "==", +1 for right side

    for ch in input.chars().rev() {
        match ch {
            'A'..='Z' => {
                *letter_to_coefficient.entry(ch).or_insert(0) += positional_value;
                positional_value *= 10;
                previous_letter = ch;
            }
            _ => {
                leading_letters.insert(previous_letter);
                positional_value = 1;
            }
        }
    }

    leading_letters.insert(previous_letter);

    letter_to_coefficient
        .iter()
        .map(|(&letter, &coefficient)| (letter, coefficient, leading_letters.contains(&letter)))
        .collect()
}

fn verify_solution(digits: &[u8], letter_coefficients: &[(char, i64, bool)]) -> bool {
    let mut polynomial_sum: i64 = 0;

    for i in 0..letter_coefficients.len() {
        let (_, coefficient, is_leading) = letter_coefficients[i];
        if digits[i] == 0 && is_leading {
            return false;
        }

        polynomial_sum += (digits[i] as i64) * coefficient;
    }

    polynomial_sum == 0
}

/// Iterator over all permutations of digits [0,1,2,...,9]
/// Uses Heap's Algorithm for efficient permutation generation
struct DigitsPermutations {
    digits: [u8; 10],
    c: [usize; 10],
    i: usize,
    first: bool,
}

impl DigitsPermutations {
    fn new() -> Self {
        DigitsPermutations {
            digits: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            c: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            i: 0,
            first: true,
        }
    }
}

impl Iterator for DigitsPermutations {
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
