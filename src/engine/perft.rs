use crate::chess::table::Board;
use crate::chess::table::Color;

pub fn perft(b: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let turn = if b.is_white_turn {
        Color::White
    } else {
        Color::Black
    };

    let moves = b.get_legal_moves(turn);
    let mut total_moves = 0;
    if moves.len() == 0 {
        return 0;
    }

    if depth == 1 {
        return moves.len() as u64;
    }

    for mv in moves {
        let mut new_board: Board = b.clone();
        new_board.perform_move(mv.old_pos, mv.new_pos, mv.move_type);
        total_moves += perft(&new_board, depth - 1);
    }

    total_moves
}

pub fn perft_divide(b: &Board, depth: u8) {
    let turn = if b.is_white_turn {
        Color::White
    } else {
        Color::Black
    };
    let moves = b.get_legal_moves(turn);
    let mut total = 0;

    for mv in moves {
        let mut new_board = b.clone();
        new_board.perform_move(mv.old_pos, mv.new_pos, mv.move_type);
        let count = perft(&new_board, depth - 1);
        println!("{:?} -> {:?}: {}", mv.old_pos, mv.new_pos, count);
        total += count;
    }
    println!("Total: {}", total);
}
