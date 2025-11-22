use crate::chess::moves_gen::moves_struct::Moves;
use crate::chess::table::Board;
use crate::engine::quiescence::quiescence;
use crate::engine::trasposition_table::{BoundType, TT, TTEntry};

const MATE_SCORE: i32 = 20000;

pub fn negamax(
    b: &mut Board,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    tt: &mut TT,
    move_buffers: &mut [Vec<Moves>],
) -> i32 {
    if depth == 0 {
        return quiescence(b, alpha, beta, 0);
    }

    let (current_buffer, next_buffers) = move_buffers.split_at_mut(1);
    current_buffer[0].clear();

    let turn = b.get_side();
    let moves = b.get_legal_moves(turn, &mut current_buffer[0]);

    if moves.is_empty() {
        if b.is_king_in_check(turn) {
            return -20000 + (depth as i32);
        } else {
            return 0;
        }
    }

    let mut best_score = -20001;

    let mut scored_moves: Vec<(Moves, i32)> = moves.iter().map(|&mv| (mv, mv.score(b))).collect();

    scored_moves.sort_by_key(|(_, score)| -score);

    for (mv, _) in scored_moves.iter() {
        let undo_info = b.make_move_with_undo(mv);

        let score = -negamax(b, depth - 1, -beta, -alpha, tt, next_buffers);

        b.unmake_move(mv, undo_info);

        if score > best_score {
            best_score = score;
        }

        if best_score > alpha {
            alpha = best_score;
        }

        if alpha >= beta {
            break;
        }
    }

    best_score
}
