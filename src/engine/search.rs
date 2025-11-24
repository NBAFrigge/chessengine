use crate::chess::moves_gen::moves_struct::Moves;
use crate::chess::table::Board;
use crate::engine::quiescence::quiescence;
use crate::engine::trasposition_table::{BoundType, TT, TTEntry};

const MATE_SCORE: i32 = 20000;

#[inline(always)]
fn is_repetition(history: &[u64], history_len: usize, current_hash: u64) -> bool {
    let start = if history_len > 8 { history_len - 8 } else { 0 };

    let mut count = 0;
    for i in start..history_len {
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
    depth: u8,
    mut alpha: i32,
    beta: i32,
    tt: &mut TT,
    move_buffers: &mut [Vec<Moves>],
    position_history: &mut [u64; 32],
    history_len: usize,
) -> i32 {
    let current_hash = b.get_hash();

    if depth >= 3 && is_repetition(position_history, history_len, current_hash) {
        return 0;
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
