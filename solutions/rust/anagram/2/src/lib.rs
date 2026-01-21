use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_sorted: Vec<char> = word_lower.chars().collect();
    word_sorted.sort();

    possible_anagrams
        .iter()
        .map(|&cand| (cand, cand.to_lowercase()))
        .filter(|(_, cand_lower)| cand_lower != &word_lower)
        .filter(|(_, cand_lower)| {
            let mut cand_sorted: Vec<char> = cand_lower.chars().collect();
            cand_sorted.sort();
            cand_sorted == word_sorted
        })
        .map(|(cand, _)| cand)
        .collect()
}
