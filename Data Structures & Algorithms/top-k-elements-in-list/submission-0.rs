impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let mut count_map: std::collections::HashMap<i32, i32> =
            std::collections::HashMap::with_capacity(nums.len());

        for num in nums {
            count_map.entry(num).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut sorted_count_map: Vec<(&i32, &i32)> = count_map.iter().collect();
        sorted_count_map.sort_by(|a, b| b.1.cmp(a.1));

        // get the top k emements
        sorted_count_map
            .iter()
            .take(k as usize)
            .map(|(key, _)| **key)
            .collect()
    }
}
