class Solution {
    /**
     * @param {number[]} nums
     * @param {number} target
     * @return {number}
     */
    search(nums: number[], target: number): number {
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

    return -1; // Target was not found in the array
    }
}
