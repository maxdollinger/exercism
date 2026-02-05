pub fn is_valid(code: &str) -> bool {
    let chars: Vec<char> = code.chars().filter(|&c| c != ' ').collect();
    let len = chars.len();
    if len < 2 {
        return false;
    }

    let mut sum: usize = 0;
    for (idx, &char) in chars.iter().enumerate() {
        let digit = match char.to_digit(10) {
            None => return false,
            Some(d) => d as usize,
        };

        let n = if (len - 1 - idx) % 2 == 1 {
            digit * 2
        } else {
            digit
        };

        sum += if n > 9 { n - 9 } else { n };
    }

    sum.is_multiple_of(10)
}
