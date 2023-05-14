pub fn find_legal_moves(board: [[char; 9]; 9]) -> Vec<usize> {
    let mut legal_vec = Vec::new();
    for i in 0..board.len() {
        if board[i][0] == ' ' {
            legal_vec.push(i);
        }
    }
    return legal_vec;
}