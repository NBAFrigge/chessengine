use crate::chess::moves_gen::moves_struct::Moves;
use crate::chess::table::Board;
use crate::engine::quiescence::quiescence;
use crate::engine::trasposition_table::{BoundType, TT, TTEntry};

const MATE_SCORE: i32 = 20000;
const CONTEMPT: i32 = 0;

#[inline(always)]
fn is_repetition(history: &[u64], current_hash: u64) -> bool {
    let mut count = 0;
    for &h in history {
        if h == current_hash {
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
    mut beta: i32,
    tt: &mut TT,
    move_buffers: &mut [Vec<Moves>],
    position_history: &mut Vec<u64>,
    killer_moves: &mut [[Moves; 2]; 64],
    history: &mut [[i32; 64]; 64],
    ply: i32,
) -> i32 {
    let alpha_orig = alpha;
    // let beta_orig = beta;
    let current_hash = b.get_hash();

    if is_repetition(position_history, current_hash) {
        return -CONTEMPT;
    }

    alpha = alpha.max(-MATE_SCORE + ply);
    beta = beta.min(MATE_SCORE - ply - 1);
    if alpha >= beta {
        return alpha;
    }

    let mut tt_move = None;
    if let Some(entry) = tt.probe(current_hash) {
        tt_move = Some(entry.best_move);
        if entry.depth >= depth {
            match entry.bound {
                BoundType::Exact => return entry.score,
                BoundType::Lower => {
                    alpha = alpha.max(entry.score);
                }
                BoundType::Upper => {
                    beta = beta.min(entry.score);
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

    // Null Move Pruning
    if depth >= 3
        && beta < MATE_SCORE
        && ply > 0
        && b.has_non_pawn_material(turn)
        && !b.is_king_in_check(turn)
    {
        let r = 2;
        let undo_null = b.make_null_move();

        let score = -negamax(
            b,
            depth - 1 - r,
            -beta,
            -beta + 1,
            tt,
            next_buffers,
            position_history,
            killer_moves,
            history,
            ply + 1,
        );

        b.unmake_null_move(undo_null);

        if score >= beta {
            return beta;
        }
    }

    let moves = b.get_legal_moves(turn, &mut current_buffer[0]);

    if moves.is_empty() {
        if b.is_king_in_check(turn) {
            return -MATE_SCORE + ply;
        } else {
            return 0;
        }
    }

    let mut scored_moves: Vec<(Moves, i32)> = moves
        .iter()
        .map(|&mv| {
            let mut score = 0;
            if let Some(tm) = tt_move {
                if mv == tm {
                    return (mv, 2_000_000_000);
                }
            }

            if mv.is_capture() {
                score = mv.score(b);
            } else {
                let mut is_killer = false;
                if (ply as usize) < 64 {
                    if mv == killer_moves[ply as usize][0] {
                        score = 900_000;
                        is_killer = true;
                    } else if mv == killer_moves[ply as usize][1] {
                        score = 800_000;
                        is_killer = true;
                    }
                }
                if !is_killer {
                    let from = mv.from() as usize;
                    let to = mv.to() as usize;
                    score = history[from][to].min(700_000);
                }
            }
            (mv, score)
        })
        .collect();

    scored_moves.sort_unstable_by_key(|(_, score)| -score);

    let mut best_score = -MATE_SCORE;
    let mut best_move = scored_moves[0].0;

    for (i, (mv, _)) in scored_moves.iter().enumerate() {
        let undo_info = b.make_move_with_undo(mv);
        position_history.push(b.get_hash());

        let in_check = b.is_king_in_check(turn);
        let extension = if in_check { 1 } else { 0 };

        let new_depth = (depth as i8 - 1 + extension).max(0) as u8;

        let mut score;

        if i == 0 {
            score = -negamax(
                b,
                new_depth,
                -beta,
                -alpha,
                tt,
                next_buffers,
                position_history,
                killer_moves,
                history,
                ply + 1,
            );
        } else {
            let mut reduction = 0;
            if depth >= 3 && i >= 4 && !mv.is_capture() && !mv.is_promotion() && !in_check {
                reduction = 1;
                if depth >= 6 && i > 10 {
                    reduction = 2;
                }
            }

            let research_depth = new_depth.saturating_sub(reduction);

            score = -negamax(
                b,
                research_depth,
                -alpha - 1,
                -alpha,
                tt,
                next_buffers,
                position_history,
                killer_moves,
                history,
                ply + 1,
            );

            if score > alpha && reduction > 0 {
                score = -negamax(
                    b,
                    new_depth,
                    -alpha - 1,
                    -alpha,
                    tt,
                    next_buffers,
                    position_history,
                    killer_moves,
                    history,
                    ply + 1,
                );
            }

            if score > alpha && score < beta {
                score = -negamax(
                    b,
                    new_depth,
                    -beta,
                    -alpha,
                    tt,
                    next_buffers,
                    position_history,
                    killer_moves,
                    history,
                    ply + 1,
                );
            }
        }

        position_history.pop();
        b.unmake_move(mv, undo_info);

        if score > best_score {
            best_score = score;
            if score > alpha {
                alpha = score;
                best_move = *mv;
            }
        }

        // Beta Cutoff
        if alpha >= beta {
            if !mv.is_capture() {
                if (ply as usize) < 64 {
                    if killer_moves[ply as usize][0] != *mv {
                        killer_moves[ply as usize][1] = killer_moves[ply as usize][0];
                        killer_moves[ply as usize][0] = *mv;
                    }
                }
                let bonus = (depth as i32) * (depth as i32);
                let from = mv.from() as usize;
                let to = mv.to() as usize;
                if history[from][to] < 1_000_000 {
                    history[from][to] += bonus;
                }
            }
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
