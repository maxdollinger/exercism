pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_terminator(&[' ', '-'])
        .flat_map(|word| {
            let chars: Vec<char> = word.chars().filter(|c| c.is_alphabetic()).collect();
            let is_camel_case =
                chars.iter().any(|c| c.is_uppercase()) && chars.iter().any(|c| c.is_lowercase());

            chars
                .into_iter()
                .enumerate()
                .filter(|&(i, c)| i == 0 || (is_camel_case && c.is_uppercase()))
                .map(|(_, c)| c.to_uppercase().to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}
