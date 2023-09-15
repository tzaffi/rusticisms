use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower = word.to_lowercase();
    let counts = count_chars(lower.as_str());

    possible_anagrams
        .iter()
        .filter(|anagram| {
            if lower.len() != anagram.len() {
                return false;
            }
            let a_lower = anagram.to_lowercase();
            lower != a_lower && counts == count_chars(a_lower.as_str())
        })
        .copied()
        .collect()
}

fn count_chars(input: &str) -> HashMap<char, usize> {
    input.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}
