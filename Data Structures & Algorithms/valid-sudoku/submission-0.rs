impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];

        for row in 0..9 {
            for col in 0..9 {
                let ch = board[row][col];

                if ch == '.' {
                    continue;
                }

                let digit = ch as u8 - b'1';
                let bit = 1u16 << digit;

                let box_index = (row / 3) * 3 + (col / 3);

                if rows[row] & bit != 0 {
                    return false;
                }

                if cols[col] & bit != 0 {
                    return false;
                }

                if boxes[box_index] & bit != 0 {
                    return false;
                }

                rows[row] |= bit;
                cols[col] |= bit;
                boxes[box_index] |= bit;
            }
        }

        true
    }
}