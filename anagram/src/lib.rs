use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut sorted_word = word.to_lowercase().chars().collect::<Vec<char>>();
    sorted_word.sort_by(|a, b| b.cmp(a));
    possible_anagrams
        .iter()
        .filter(|&a| {
            if a.to_lowercase() == word.to_lowercase() || a.len() != word.len() {
                return false;
            }

            let mut chars = a.to_lowercase().chars().collect::<Vec<char>>();
            chars.sort_by(|a, b| b.cmp(a));
            chars.eq(&sorted_word)
        })
        .cloned()
        .collect::<HashSet<&'a str>>()
}
