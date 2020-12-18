use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut output: HashSet<&'a str> = HashSet::new();

    for anagram in possible_anagrams {
        // Condition check 1: The lengths of word and anagram should match.
        // Condition check 2: The lowercase of word and potential anagram are NOT the same.
        if check_length(word, anagram) && word.to_lowercase() != anagram.to_lowercase() {
            let mut split_word: Vec<char> = word.to_lowercase().chars().collect();
            for c in anagram.to_lowercase().chars() {
                split_word = remove_char_from_vec(split_word.clone(), c);
            }

            // If split_word is empty, insert it into the output Hashset.
            if split_word.len() == 0 {
                output.insert(anagram);
            }
        }
    }

    output
}

// Check if the length of a potential anagram matches that of word.
fn check_length(word: &str, anagram: &str) -> bool {
    word.len() == anagram.len()
}

// Search if a character exists in vec.
// If so, find its index and remove it.
fn remove_char_from_vec(mut v: Vec<char>, c: char) -> Vec<char> {
    if v.iter().any(|&i| i == c) {
        let index = v.iter().position(|x| *x == c).unwrap();
        v.remove(index);
    }
    v
}
