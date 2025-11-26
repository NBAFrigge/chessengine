use crate::chess::moves_gen::moves_struct::Moves;
use crate::chess::table::Board;
use crate::engine::quiescence::quiescence;
use crate::engine::trasposition_table::{BoundType, TT, TTEntry};

const MATE_SCORE: i32 = 20000;

#[inline(always)]
fn is_repetition(history: &[u64], history_len: usize, current_hash: u64) -> bool {
    let mut count = 0;
    for i in 0..history_len {
        if history[i] == current_hash {
            count += 1;
            if count >= 2 {
                return true;
            }
        }
    }
    false
}

pub fn negamax(
    b: &mut Board,
    mut depth: u8,
    mut alpha: i32,
    beta: i32,
    tt: &mut TT,
    move_buffers: &mut [Vec<Moves>],
    position_history: &mut [u64; 128],
    history_len: usize,
) -> i32 {
    let current_hash = b.get_hash();

    if is_repetition(position_history, history_len, current_hash) {
        return 0;
    }

    let ply = history_len as i32;
    alpha = alpha.max(-MATE_SCORE + ply);
    let beta_adjusted = beta.min(MATE_SCORE - ply - 1);
    if alpha >= beta_adjusted {
        return alpha;
    }

    let alpha_orig = alpha;

    if let Some(entry) = tt.probe(current_hash) {
        if entry.depth >= depth {
            match entry.bound {
                BoundType::Exact => return entry.score,
                BoundType::Lower => {
                    if entry.score > alpha {
                        alpha = entry.score;
                    }
                }
                BoundType::Upper => {
                    if entry.score < beta {
                        return entry.score;
                    }
                }
            }
            if alpha >= beta {
                return entry.score;
            }
        }
    }

    if depth == 0 {
        if b.is_king_in_check(b.get_side()) {
            depth = 1;
        } else {
            return quiescence(b, alpha, beta, 0);
        }
    }

    if move_buffers.is_empty() {
        return quiescence(b, alpha, beta, 0);
    }

    let (current_buffer, next_buffers) = move_buffers.split_at_mut(1);
    current_buffer[0].clear();

    let turn = b.get_side();
    let moves = b.get_legal_moves(turn, &mut current_buffer[0]);

    if moves.is_empty() {
        if b.is_king_in_check(turn) {
            return -MATE_SCORE + (depth as i32);
        } else {
            return 0;
        }
    }

    let mut best_score = -MATE_SCORE - 1;
    let mut best_move = moves[0];

    let mut scored_moves: Vec<(Moves, i32)> = moves.iter().map(|&mv| (mv, mv.score(b))).collect();

    if let Some(entry) = tt.probe(current_hash) {
        for (mv, score) in scored_moves.iter_mut() {
            if mv.from() == entry.best_move.from() && mv.to() == entry.best_move.to() {
                *score += 10000;
                break;
            }
        }
    }

    scored_moves.sort_unstable_by_key(|(_, score)| -score);

    for (mv, _) in scored_moves.iter() {
        let undo_info = b.make_move_with_undo(mv);

        let new_history_len = if history_len < 32 {
            position_history[history_len] = b.get_hash();
            history_len + 1
        } else {
            history_len
        };

        let score = -negamax(
            b,
            depth - 1,
            -beta,
            -alpha,
            tt,
            next_buffers,
            position_history,
            new_history_len,
        );

        b.unmake_move(mv, undo_info);

        if score > best_score {
            best_score = score;
            best_move = *mv;
        }

        if best_score > alpha {
            alpha = best_score;
        }

        if alpha >= beta {
            break;
        }
    }

    let bound = if best_score <= alpha_orig {
        BoundType::Upper
    } else if best_score >= beta {
        BoundType::Lower
    } else {
        BoundType::Exact
    };

    let entry = TTEntry::new(current_hash, best_score, best_move, depth, bound, tt.age);
    tt.store(current_hash, entry);

    best_score
}
