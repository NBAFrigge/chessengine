use crate::{
    chess::{moves_gen::moves_struct::Moves, table::Board},
    engine::{
        evaluate::evaluate::{calculate_game_phase, evaluate},
        quiescence::quiescence,
    },
};

const MATE_SCORE: i32 = 20000;
const INFINITY: i32 = 30000;
pub fn negamax(
    b: &mut Board,
    depth: u8,
    alpha: i32,
    beta: i32,
    move_buffers: &mut [Vec<Moves>],
) -> i32 {
    if depth == 0 {
        return quiescence(b, alpha, beta, 0);
    }

    let mut alpha = alpha;
    let (current_moves_buffer, next_buffers) = move_buffers.split_first_mut().unwrap();
    current_moves_buffer.clear();

    let turn = b.get_side();
    let moves = b.get_legal_moves(turn, current_moves_buffer);

    if moves.is_empty() {
        return if b.is_king_in_check(turn) {
            -MATE_SCORE + depth as i32
        } else {
            0
        };
    }

    // TODO (Pick Best One at a Time)
    let mut scored_moves: Vec<(Moves, i32)> = moves
        .iter()
        .map(|&mv| {
            let score = mv.score(b);
            (mv, score)
        })
        .collect();

    scored_moves.sort_by_key(|(_, score)| -score);

    let mut max_score = -INFINITY;
    for mv in scored_moves {
        let undo_info = b.make_move_with_undo(&mv.0);
        let score = -negamax(b, depth - 1, -beta, -alpha, next_buffers);
        b.unmake_move(&mv.0, undo_info);

        if score > max_score {
            max_score = score;
        }

        if max_score > alpha {
            alpha = max_score;
        }

        if alpha >= beta {
            return beta;
        }
    }
    max_score
}
