class Solution {
    /**
     * @param {number[]} heights
     * @return {number}
     */
    maxArea(heights) {

        let answer = 0;

        let left = 0
        let right = heights.length -1

        while (left < right ) {

            // calculate volume

            let height = Math.min(heights[left], heights[right])
            let width = right-left
            let calculation =  width * height

            answer = Math.max(answer, calculation)

            if (heights[left] < heights[right]) {
                left++
            } else {
                right--;
            }

        }
    return answer
    }
}
