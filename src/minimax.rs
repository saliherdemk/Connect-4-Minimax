pub fn find_legal_moves(board: [[char; 9]; 9]) -> Vec<usize> {
    let mut legal_vec = Vec::new();
    for i in 0..board.len() {
        if board[i][0] == ' ' {
            legal_vec.push(i);
        }
    }
    return legal_vec;
}

pub fn result(board: &mut [[char; 9]; 9], move: usize, symbol: char) -> bool {
    for j in (0..SIZE).rev() {
        if board[i - 1][j] == ' ' {
            board[i - 1][j] = symbol;
            return board;
        }
    }
}

pub fn minimax(board: [[char; 9]; 9], depth: isize, alpha: isize, beta: isize, maximizing_player: Player):
