fn parse_code(code: &str) -> Option<Vec<u32>> {
    let mut chars = code.chars();

    if chars.any(|c| !c.is_numeric() && (c != ' ')) {
        return None;
    }

    let luhn_code: Vec<u32> = code
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    if luhn_code.len() < 2 {
        return None;
    }

    Some(luhn_code)
}

pub fn is_valid(code: &str) -> bool {
    let mut numbers = match parse_code(code) {
        None => return false,
        Some(res) => res,
    };

    numbers.reverse();
    numbers
        .chunks(2)
        .fold(0u32, |mut sum, chunk| {
            sum += chunk.first().unwrap_or(&0);
            sum += chunk
                .get(1)
                .map(|d| d * 2)
                .map(|d| if d > 9 { d - 9 } else { d })
                .unwrap_or(0);

            sum
        })
        .is_multiple_of(10)
}
