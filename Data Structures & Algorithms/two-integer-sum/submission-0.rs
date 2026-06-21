impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // find where num[x] + num[y] = target

        let mut c_map: std::collections::HashMap<i32,i32> = std::collections::HashMap::new();

        for (index , num) in nums.iter().enumerate() {
            let compliment = target - num;

            if let Some(&compliment_index)= c_map.get(&compliment) {
                return vec![compliment_index, index as i32]
            }
            c_map.insert(*num, index as i32);
        }
        vec![]
    }
}
