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
    beta: i32,
    tt: &mut TT,
    move_buffers: &mut [Vec<Moves>],
    position_history: &mut Vec<u64>,
    killer_moves: &mut [[Moves; 2]; 64],
    history: &mut [[i32; 64]; 64],
    ply: i32,
) -> i32 {
    let current_hash = b.get_hash();

    if is_repetition(position_history, current_hash) {
        return -CONTEMPT;
    }

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

    let mut best_score = -MATE_SCORE - 1;
    let mut best_move = moves[0];

    let mut scored_moves: Vec<(Moves, i32)> = moves
        .iter()
        .map(|&mv| {
            let mut score = 0;
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
                    let h_val = history[from][to];

                    score = if h_val > 700_000 { 700_000 } else { h_val };
                }
            }
            (mv, score)
        })
        .collect();

    if let Some(entry) = tt.probe(current_hash) {
        for (mv, score) in scored_moves.iter_mut() {
            if *mv == entry.best_move {
                *score += 20_000_000;
                break;
            }
        }
    }

    scored_moves.sort_unstable_by_key(|(_, score)| -score);

    for (i, (mv, _)) in scored_moves.iter().enumerate() {
        let undo_info = b.make_move_with_undo(mv);
        position_history.push(b.get_hash());

        let score = if depth >= 3
            && i > 4
            && !mv.is_capture()
            && !mv.is_promotion()
            && !b.is_king_in_check(turn)
        {
            let reduction = 1;
            let reduced_depth = if depth > reduction + 1 {
                depth - 1 - reduction
            } else {
                1
            };

            let temp_score = -negamax(
                b,
                reduced_depth,
                -beta,
                -alpha,
                tt,
                next_buffers,
                position_history,
                killer_moves,
                history,
                ply + 1,
            );

            if temp_score > alpha {
                -negamax(
                    b,
                    depth - 1,
                    -beta,
                    -alpha,
                    tt,
                    next_buffers,
                    position_history,
                    killer_moves,
                    history,
                    ply + 1,
                )
            } else {
                temp_score
            }
        } else {
            -negamax(
                b,
                depth - 1,
                -beta,
                -alpha,
                tt,
                next_buffers,
                position_history,
                killer_moves,
                history,
                ply + 1,
            )
        };

        position_history.pop();
        b.unmake_move(mv, undo_info);

        if score > best_score {
            best_score = score;
            best_move = *mv;
        }

        if best_score > alpha {
            alpha = best_score;
        }

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
