impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {


        // Create answer answer map
        // Use sorting to create unique keys and thus an anagram check 
        // We can use the unique keys to just check if a word is an anagram of another by just
        // sorting it and then it should match the previous, if it does, add it to the map
        // After we do this for all words, we will have a map of letters to their words that are
        // anagrams using those letters
        // then we simply return the words as an array of arrays containing the words


        let mut answer_map: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for word in strs {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            let key = chars.into_iter().collect();

            if let Some(value) = answer_map.get_mut(&key) {
                value.push(word.clone());
            } else {
                answer_map.insert(key, vec![word.clone()]);
            }
        }

        answer_map.values().cloned().collect()
    }
}
