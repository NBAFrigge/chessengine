use crate::{
    chess::{moves_gen::moves_struct::Moves, table::Board},
    engine::evaluate,
};

pub fn negamax(b: &mut Board, depth: u8, move_buffers: &mut [Vec<Moves>]) -> i32 {
    if depth == 0 {
        return evaluate::evaluate(b);
    }

    let (current_moves_buffer, next_buffers) = move_buffers.split_first_mut().unwrap();

    let turn = b.get_side();
    let mut max_score = i32::MIN;

    let moves = b.get_legal_moves(turn, current_moves_buffer);

    if moves.is_empty() {
        if b.is_king_in_check(turn) {
            return i32::MIN + (100 - depth as i32);
        } else {
            return 0;
        }
    }

    for mv in moves.iter() {
        let undo_info = b.make_move_with_undo(mv);

        let score = -negamax(b, depth - 1, next_buffers);

        b.unmake_move(mv, undo_info);

        if score > max_score {
            max_score = score;
        }
    }

    max_score
}
