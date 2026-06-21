
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut frequency_counter = std::collections::HashMap::with_capacity(s.len());

        // we have a frequeency counter, we can loop over the s string, and count the frequencies
        // we can then loop over t the t string, if the frequencies don't match, then we don't have
        // an anagram
        //

        for letter in s.chars() {
            frequency_counter
                .entry(letter)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        // now that we have the s frequency map, we can check t??

        for letter in t.chars() {
            frequency_counter
                .entry(letter)
                .and_modify(|count| *count -= 1);
        }
        // only retain if not 0 so that we can say, if not empty, it's not an anagram
        frequency_counter.retain(|_, v| *v != 0);

        // check if there are any entries in the hashmap with count > 0
        if frequency_counter.is_empty() {
            return true;
        }
        false
    }
}
// @leet end

