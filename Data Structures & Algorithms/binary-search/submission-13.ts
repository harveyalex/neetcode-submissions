class Solution {
    /**
     * @param {number[]} nums
     * @param {number} target
     * @return {number}
     */
    search(nums: number[], target: number): number {
        // array is sorted, we need to just binary search for the number

        // binary search is where you take the middle of a sorted array,
        // if the middle is higher the number you are looking for, then it's in the left side,
        // if its lower, then it's in the right side,
        // then you take hte middle of the L or R and continue to use that to narrow down

        // get middle:
        let low = 0;
        let high = nums.length - 1;
        while (low <= high) {
            // Calculate the middle index
            // (low + high) / 2 can cause integer overflow in some languages,
            // so this math formula is the safest way to find the middle:
            let mid = low + Math.floor((high - low) / 2);

            if (nums[mid] === target) {
                return mid; // Found the target, return its index
            } else if (nums[mid] < target) {
                low = mid + 1; // Target is in the right half
            } else {
                high = mid - 1; // Target is in the left half
            }
        }
        return -1;
    }
}
