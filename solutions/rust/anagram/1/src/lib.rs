use std::collections::{HashMap, HashSet};

fn get_char_count(word: &str) -> HashMap<char, u16> {
    let mut map = HashMap::new();

    for c in word.chars() {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    map
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_char_count = get_char_count(&word_lower);

    possible_anagrams
        .iter()
        .map(|&cand| (cand, cand.to_lowercase()))
        .filter(|(_, cand_lower)| cand_lower != &word_lower)
        .filter(|(_, cand_lower)| get_char_count(cand_lower) == word_char_count)
        .map(|(cand, _)| cand)
        .collect()
}
