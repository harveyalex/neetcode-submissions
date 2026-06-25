impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // Initialise max_streak
        let nums_set: std::collections::HashSet<i32> = nums.into_iter().collect();

        // Initialise max_streak
        let mut max_streak: i32 = 0;

        // Iterate over numbers and evaluate

        for (index, num) in nums_set.iter().enumerate() {
            // check if streak starter
            if !nums_set.contains(&(num - 1)) {
                // we have a streak starter
                let mut current_num = *num;
                let mut current_streak = 1;
                // While the streak continues:

                // conintue to find the current_num + 1 in set
                while nums_set.contains(&(current_num + 1)) {
                    current_streak += 1;
                    current_num += 1;
                }
                max_streak = i32::max(current_streak, max_streak)
            }
        }

        max_streak
    }
}
