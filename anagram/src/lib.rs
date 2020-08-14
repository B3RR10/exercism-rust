use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut sorted_word: Vec<char> = word.to_lowercase().chars().collect();
    sorted_word.sort();
    possible_anagrams
        .iter()
        .filter(|&a| {
            if a.len() != word.len() || a.to_lowercase() == word.to_lowercase() {
                false
            } else {
                let mut chars = a.to_lowercase().chars().collect::<Vec<char>>();
                chars.sort();
                chars.eq(&sorted_word)
            }
        })
        .cloned()
        .collect()
}
