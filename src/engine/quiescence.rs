use crate::{
    chess::table::{Board, Color},
    engine::evaluate::evaluate::{calculate_game_phase, evaluate},
};

const MAX_DEPTH: i32 = -5;

pub fn quiescence(b: &mut Board, mut alpha: i32, beta: i32, depth: i32) -> i32 {
    if depth <= MAX_DEPTH {
        let phase = calculate_game_phase(b);
        return evaluate(b, phase);
    }

    let phase = calculate_game_phase(b);
    let stand_pat = evaluate(b, phase);

    if stand_pat >= beta {
        return beta;
    }

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    if stand_pat < alpha - 900 {
        return alpha;
    }

    let mut captures = Vec::new();
    b.gen_all_attacks(&mut captures);

    if captures.is_empty() {
        return stand_pat;
    }

    let mut best_score = stand_pat;
    for capture_move in captures {
        let undo = b.make_move_with_undo(&capture_move);

        let opponent = if b.is_white_turn {
            Color::Black
        } else {
            Color::White
        };

        if b.is_king_in_check(opponent) {
            b.unmake_move(&capture_move, undo);
            continue;
        }

        let score = -quiescence(b, -beta, -alpha, depth - 1);

        b.unmake_move(&capture_move, undo);

        if score > best_score {
            best_score = score;
        }

        if best_score > alpha {
            alpha = best_score;
        }

        if alpha >= beta {
            return beta;
        }
    }

    best_score
}
