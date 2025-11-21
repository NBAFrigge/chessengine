use crate::{
    chess::{moves_gen::moves_struct::Moves, table::Board},
    engine::{
        quiescence::quiescence,
        trasposition_table::{BoundType, TT, TTEntry},
    },
};
use std::cmp;

const MATE_SCORE: i32 = 20000;
const INFINITY: i32 = 30000;

pub fn negamax(
    b: &mut Board,
    depth: u8,
    alpha: i32,
    beta: i32,
    tt: &mut TT,
    move_buffers: &mut [Vec<Moves>],
) -> i32 {
    let hash = b.get_hash();
    let original_alpha = alpha;
    let mut alpha = alpha;
    let mut beta = beta;

    if let Some(entry) = tt.probe(hash) {
        if entry.depth >= depth {
            if entry.bound == BoundType::Exact {
                return entry.score;
            } else if entry.bound == BoundType::Lower {
                alpha = cmp::max(entry.score, alpha);
            } else if entry.bound == BoundType::Upper {
                beta = cmp::min(entry.score, beta);
            }

            if alpha >= beta {
                return alpha;
            }
        }
    }

    if depth == 0 {
        return quiescence(b, alpha, beta, 0);
    }

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

    let mut scored_moves: Vec<(Moves, i32)> = moves
        .iter()
        .map(|&mv| {
            let score = mv.score(b);
            (mv, score)
        })
        .collect();

    scored_moves.sort_by_key(|(_, score)| -score);

    let mut max_score = -INFINITY;
    let mut best_move: Option<Moves> = None;

    for mv in scored_moves {
        let undo_info = b.make_move_with_undo(&mv.0);
        let score = -negamax(b, depth - 1, -beta, -alpha, tt, next_buffers);
        b.unmake_move(&mv.0, undo_info);

        if score > max_score {
            max_score = score;
            best_move = Some(mv.0);
        }

        if max_score > alpha {
            alpha = max_score;
        }

        if alpha >= beta {
            let bound = determine_bound(max_score, original_alpha, beta);
            if let Some(mv) = best_move {
                let entry = TTEntry::new(hash, max_score, mv, depth, bound, 0);
                tt.store(hash, entry);
            }
            return max_score;
        }
    }

    let bound = determine_bound(max_score, original_alpha, beta);
    if let Some(mv) = best_move {
        let entry = TTEntry::new(hash, max_score, mv, depth, bound, 0);
        tt.store(hash, entry);
    }

    max_score
}

fn determine_bound(score: i32, original_alpha: i32, beta: i32) -> BoundType {
    if score <= original_alpha {
        BoundType::Upper
    } else if score >= beta {
        BoundType::Lower
    } else {
        BoundType::Exact
    }
}
