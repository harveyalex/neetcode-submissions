class Solution {
    /**
     * @param {number[]} nums
     * @return {number[][]}
     */
    threeSum(nums) {
         // Sort first so we can use the two-pointer pattern.
        nums.sort((a, b) => a - b);

        const result = [];

        // Pick one number at a time as the "fixed" first number.
        for (let i = 0; i < nums.length; i++) {
            // Skip duplicate fixed numbers so we don't create duplicate triplets.
            if (i > 0 && nums[i] === nums[i - 1]) continue;

            // Look for the other two numbers after i.
            let left = i + 1;
            let right = nums.length - 1;

            while (left < right) {
                const sum = nums[i] + nums[left] + nums[right];

                if (sum === 0) {
                    // Found one valid triplet.
                    result.push([nums[i], nums[left], nums[right]]);

                    // Move both pointers so we can search for another pair.
                    left++;
                    right--;

                    // Skip duplicate left values.
                    while (left < right && nums[left] === nums[left - 1]) {
                        left++;
                    }

                    // Skip duplicate right values.
                    while (left < right && nums[right] === nums[right + 1]) {
                        right--;
                    }
                } else if (sum < 0) {
                    // Sum is too small, so we need a bigger number.
                    left++;
                } else {
                    // Sum is too large, so we need a smaller number.
                    right--;
                }
            }
        }

        return result;
    }
}
