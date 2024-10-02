use std::collections::{HashMap, HashSet};

/// Функция для поиска множеств анаграмм
fn find_anagrams(words: &[&str]) -> HashMap<String, Vec<String>> {
    let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut seen_words: HashSet<String> = HashSet::new();

    for &word in words {
        let word_lower = word.to_lowercase();

        let mut chars: Vec<char> = word_lower.chars().collect();
        chars.sort_unstable();

        let key = chars.iter().collect::<String>();

        let entry = anagram_map.entry(key).or_insert_with(Vec::new);
        if !seen_words.contains(&word_lower) {
            entry.push(word_lower.clone());
            seen_words.insert(word_lower);
        }
    }

    // Remove sets with one entity
    anagram_map
        .into_iter()
        .filter_map(|(_, mut anagrams)| {
            if anagrams.len() > 1 {
                // Sort the entities
                let first_word = anagrams[0].clone();
                anagrams.sort();
                Some((first_word, anagrams))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    // Пример использования
    let words = vec![
        "пятак",
        "пятка",
        "тяпка",
        "слиТок",
        "лисТок",
        "столик",
        "кот",
        "ток",
        "окт",
    ];

    let result = find_anagrams(&words);

    for (key, anagrams) in result {
        println!("{}: {:?}", key, anagrams);
    }
}
