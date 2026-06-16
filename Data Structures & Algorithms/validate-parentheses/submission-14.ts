class Solution {
    /**
     * @param {string} s
     * @return {boolean}
     */
    isValid(s: string): boolean {
        const stack = [];
    // Map closing brackets to their matching opening brackets
    const closeToOpen = {
        ')': '(',
        ']': '[',
        '}': '{'
    };

    for (let char of s) {
        // Check if the character is a closing bracket
        if (char in closeToOpen) {
            // Get the top element of the stack (or null if stack is empty)
            const topElement = stack.length > 0 ? stack[stack.length - 1] : null;
            
            // If the top element matches the required opening bracket, pop it
            if (topElement === closeToOpen[char]) {
                stack.pop();
            } else {
                // Mismatch found, or stack was empty when it shouldn't be
                return false;
            }
        } else {
            // If it's an opening bracket, push it onto the stack
            stack.push(char);
        }
    }

    // If the stack is completely empty, all brackets were validly closed
    return stack.length === 0;
    }
}
