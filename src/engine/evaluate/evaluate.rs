use crate::chess::table::Board;
use crate::chess::table::{Color, Type};
use crate::engine::evaluate::bishop_pair::{self, evaluate_bishop_pair};
use crate::engine::evaluate::endgame::evaluate_endgame_aggression;
use crate::engine::evaluate::king_safety::evaluate_king_safety;
use crate::engine::evaluate::pawn_evaluation::evaluate_pawn;
use crate::engine::evaluate::pst::get_pst_value;

const PAWN_WEIGHT: i32 = 100;
const KING_WEIGHT: i32 = 9999;
const KNIGHT_WEIGHT: i32 = 320;
const QUEEN_WEIGHT: i32 = 999;
const BISHOP_WEIGTH: i32 = 330;
const ROOK_WEIGHT: i32 = 500;

const KNIGHT_PHASE_WEIGHT: i32 = 1;
const QUEEN_PHASE_WEIGHT: i32 = 4;
const BISHOP_PHASE_WEIGTH: i32 = 1;
const ROOK_PHASE_WEIGHT: i32 = 2;

pub fn evaluate(b: &Board, phase: f32) -> i32 {
    let mut score = 0;

    for sq in 0..64 {
        let piece_color = match b.get_piece_color_at_square(sq) {
            Some(c) => c,
            None => continue,
        };
        let piece_type = match b.get_piece_type_at_square(sq) {
            Some(t) => t,
            None => continue,
        };

        match piece_color {
            Color::White => {
                score += get_piece_value(piece_type)
                    + get_pst_value(piece_type, sq as usize, piece_color, phase)
            }
            Color::Black => {
                score -= get_piece_value(piece_type)
                    + get_pst_value(piece_type, sq as usize, piece_color, phase)
            }
        }
    }

    score += evaluate_pawn(&b.get_pieces(Color::White, Type::Pawn));
    score -= evaluate_pawn(&b.get_pieces(Color::Black, Type::Pawn));
    score += evaluate_king_safety(b, Color::White, phase);
    score -= evaluate_king_safety(b, Color::Black, phase);
    score += evaluate_bishop_pair(b, Color::White);
    score -= evaluate_bishop_pair(b, Color::Black);

    if phase > 0.5 {
        score += evaluate_center_control(b, Color::White);
        score -= evaluate_center_control(b, Color::Black);
        score += evaluate_premature_pawns(b, Color::White);
        score -= evaluate_premature_pawns(b, Color::Black);
    }

    if phase < 0.5 {
        score += evaluate_endgame_aggression(b, Color::White, phase);
        score -= evaluate_endgame_aggression(b, Color::Black, phase);
    }

    if b.is_white_turn { score } else { -score }
}

pub fn calculate_game_phase(b: &Board) -> f32 {
    ((b.get_pieces(Color::White, Type::Knight).count_ones() as i32
        + b.get_pieces(Color::Black, Type::Knight).count_ones() as i32)
        * KNIGHT_PHASE_WEIGHT
        + (b.get_pieces(Color::White, Type::Queen).count_ones() as i32
            + b.get_pieces(Color::Black, Type::Queen).count_ones() as i32)
            * QUEEN_PHASE_WEIGHT
        + (b.get_pieces(Color::White, Type::Bishop).count_ones() as i32
            + b.get_pieces(Color::Black, Type::Bishop).count_ones() as i32)
            * BISHOP_PHASE_WEIGTH
        + (b.get_pieces(Color::White, Type::Rook).count_ones() as i32
            + b.get_pieces(Color::Black, Type::Rook).count_ones() as i32)
            * ROOK_PHASE_WEIGHT) as f32
        / 24.0
}

pub fn evaluate_development(b: &Board, color: Color) -> i32 {
    let mut score = 0;
    let back_rank = match color {
        Color::White => 0x00000000000000FF,
        Color::Black => 0xFF00000000000000,
    };

    let knights = b.get_pieces(color, Type::Knight);
    let undeveloped_knights = (knights.get_value() & back_rank).count_ones();
    score -= (undeveloped_knights as i32) * 50;

    let bishops = b.get_pieces(color, Type::Bishop);
    let undeveloped_bishops = (bishops.get_value() & back_rank).count_ones();
    score -= (undeveloped_bishops as i32) * 50;

    let queens = b.get_pieces(color, Type::Queen);
    if (queens.get_value() & back_rank) == 0 {
        if undeveloped_knights > 0 || undeveloped_bishops > 0 {
            score -= 80;
        }
    }

    if b.has_castled(color) {
        score += 60;
    }

    score
}

pub fn evaluate_center_control(b: &Board, color: Color) -> i32 {
    let center_squares = 0x0000001818000000u64;
    let pawns = b.get_pieces(color, Type::Pawn).get_value();
    let center_pawns = (pawns & center_squares).count_ones();
    (center_pawns as i32) * 50
}

pub fn evaluate_premature_pawns(b: &Board, color: Color) -> i32 {
    let pawns = b.get_pieces(color, Type::Pawn);
    let back_rank = match color {
        Color::White => 0x000000000000FFFF,
        Color::Black => 0xFFFF000000000000,
    };

    let moved_pawns = pawns.get_value() & !back_rank;
    let moved_count = moved_pawns.count_ones();

    let knights = b.get_pieces(color, Type::Knight);
    let bishops = b.get_pieces(color, Type::Bishop);
    let developed_knights = (knights.get_value() & !back_rank).count_ones();
    let developed_bishops = (bishops.get_value() & !back_rank).count_ones();
    let developed_pieces = developed_knights + developed_bishops;

    if moved_count > developed_pieces + 2 {
        let excess = moved_count - developed_pieces - 2;
        return -(excess as i32) * 50;
    }

    0
}
pub fn get_piece_value(t: Type) -> i32 {
    match t {
        Type::Pawn => PAWN_WEIGHT,
        Type::Bishop => BISHOP_WEIGTH,
        Type::Knight => KNIGHT_WEIGHT,
        Type::King => KING_WEIGHT,
        Type::Rook => ROOK_WEIGHT,
        Type::Queen => QUEEN_WEIGHT,
        _ => 0,
    }
}
