impl Solution {
    pub fn is_valid(s: String) -> bool {
// Convert the string into its underlying byte vector to mutate it in-place.
    // This consumes `s`, meaning no new heap memory is allocated.
    let mut bytes = s.into_bytes();
    let mut top = 0; // Our stack pointer

    for i in 0..bytes.len() {
        let b = bytes[i];
        
        // If it's an opening bracket, "push" it by moving it to the 'top' pointer
        if b == b'(' || b == b'[' || b == b'{' {
            bytes[top] = b;
            top += 1;
        } else {
            // It's a closing bracket. If our stack is empty, it's invalid.
            if top == 0 {
                return false;
            }
            
            // "Peek/Pop" the last open bracket from our in-place stack
            let last_open = bytes[top - 1];
            
            // Check for a mismatch
            if (b == b')' && last_open != b'(') ||
               (b == b']' && last_open != b'[') ||
               (b == b'}' && last_open != b'{') {
                return false;
            }
            
            // Successfully matched, decrement top pointer ("pop")
            top -= 1;
        }
    }

    // If top is 0, all opened brackets were successfully popped
    top == 0
    }
}
