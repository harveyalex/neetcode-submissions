impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
    
    // Create a lookup map for closing-to-opening brackets
    let close_to_open: HashMap<char, char> = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
    ]);

    for c in s.chars() {
        // Check if the character is a closing bracket
        if let Some(&matching_open) = close_to_open.get(&c) {
            // Pop the top of the stack and check if it matches
            if stack.pop() != Some(matching_open) {
                return false;
            }
        } else {
            // It's an opening bracket, push it onto the stack
            stack.push(c);
        }
    }

    // The string is valid only if the stack is completely empty
    stack.is_empty()
    }
}
