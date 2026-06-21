impl Solution{
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut my_set = std::collections::HashMap::with_capacity(nums.len());

        for num in nums {
            if let Some(boolean) = my_set.insert(num, num) {
                return true;
            }
        }
        false
    }
}
