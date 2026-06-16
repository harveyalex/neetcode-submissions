class Solution {
    /**
     * @param {number[]} numbers
     * @param {number} target
     * @return {number[]}
     */
    twoSum(numbers, target) {
        // if target was 3, the answer would be [0,1]

        // start with L being arr[0] and R being arr[-1]
        // calculate L + R, if L + R > target, Reduce R
        // If we don't get target from arr[0] + R
        // we are going to have to start with arr[-1] and increase L

        let left = 0;
        let right = numbers.length - 1;
        while (left < right) {
            let calculation = numbers[left] + numbers[right];

            console.log({ calculation, left, right });

            if (calculation === target) return [left + 1, right +1];

            if ( calculation > target ) {
                right--;
            }
            if (calculation < target) {
                left++;
            }

            // if (calculation === target) {
            //     return [left + 1, right + 1];
            // } else if (calculation > target) {
            //     right--;
            // } else {
            //     left++;
            //     right--;
            // }
        }
    }
}
