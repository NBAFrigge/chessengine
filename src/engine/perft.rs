use crate::chess::table::Board;
use crate::chess::table::Color;

pub fn perft(b: &Board, depth: i32) -> i32 {
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
    for mv in moves {
        let mut new_board: Board = b.clone();
        new_board.perform_move(mv.old_pos, mv.new_pos);
        total_moves += perft(&new_board, depth - 1);
    }

    total_moves
}

pub fn perft_divide(b: &Board, depth: i32) {
    let turn = if b.is_white_turn {
        Color::White
    } else {
        Color::Black
    };
    let moves = b.get_legal_moves(turn);
    let mut total = 0;

    for mv in moves {
        let mut new_board = b.clone();
        new_board.perform_move(mv.old_pos, mv.new_pos);
        let count = perft(&new_board, depth - 1);
        println!("{:?} -> {:?}: {}", mv.old_pos, mv.new_pos, count);
        total += count;
    }
    println!("Total: {}", total);
}
