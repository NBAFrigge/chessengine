use crate::chess::moves_gen::moves_struct::Moves;
use crate::chess::table::Board;
use crate::engine::quiescence::quiescence;
use crate::engine::trasposition_table::{BoundType, TT, TTEntry};

const MATE_SCORE: i32 = 20000;

const INFINITY: i32 = 30000;

pub fn negamax(
    b: &mut Board,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    tt: &mut TT,
    move_buffers: &mut [Vec<Moves>],
) -> i32 {
    let hash = b.get_hash();

    let mut tt_move: Option<Moves> = None;

    if let Some(entry) = tt.probe(hash) {
        tt_move = Some(entry.best_move);

        if entry.depth >= depth {
            match entry.bound {
                BoundType::Exact => {
                    return entry.score;
                }
                BoundType::Lower => {
                    if entry.score >= beta {
                        return beta;
                    }
                    alpha = alpha.max(entry.score);
                }
                BoundType::Upper => {
                    if entry.score <= alpha {
                        return alpha;
                    }
                    beta = beta.min(entry.score);
                }
            }

            if alpha >= beta {
                return alpha;
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

    let mut scored_moves: Vec<(Moves, i32)> = moves
        .iter()
        .map(|&mv| {
            let mut score = mv.score(b);
            if let Some(tt_mv) = tt_move {
                if mv.from() == tt_mv.from()
                    && mv.to() == tt_mv.to()
                    && mv.promotion_piece() == tt_mv.promotion_piece()
                {
                    score += 10_000_000;
                }
            }

            (mv, score)
        })
        .collect();

    scored_moves.sort_by_key(|(_, score)| -score);

    let mut best_score = -INFINITY;
    let mut best_move: Option<Moves> = None;

    for (mv, _) in scored_moves.iter() {
        let undo_info = b.make_move_with_undo(mv);

        let score = -negamax(b, depth - 1, -beta, -alpha, tt, next_buffers);

        b.unmake_move(mv, undo_info);

        if score > best_score {
            best_score = score;
            best_move = Some(*mv);
        }

        if best_score > alpha {
            alpha = best_score;
        }

        if alpha >= beta {
            break;
        }
    }

    let bound = if best_score >= beta {
        BoundType::Lower
    } else {
        BoundType::Upper
    };

    let tt_entry = TTEntry::new(
        hash,
        best_score,
        best_move.unwrap_or(Moves::new(0, 0, 0, 0, false)),
        depth,
        bound,
        tt.age,
    );

    tt.store(hash, tt_entry);

    best_score
}
