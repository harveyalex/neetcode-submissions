impl Solution {
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let mut low = 0;
        let mut high = nums.len() - 1;

        while low <= high {
            // Using bit-shift for max optimization: equivalent to low + (high - low) / 2
            let mid = low + ((high - low) >> 1);

            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Equal => return mid as i32,
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => {
                    // Prevent usize underflow if mid is 0 and target is smaller
                    if mid == 0 {
                        break;
                    }
                    high = mid - 1;
                }
            }
        }

        -1
    }
}
