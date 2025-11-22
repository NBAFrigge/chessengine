use crate::chess::table::Board;
use crate::chess::table::{Color, Type};
use crate::engine::evaluate::bishop_pair::{self, evaluate_bishop_pair};
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
    let mut piece_color: Color;
    let mut piece_type: Type;
    let mut score = 0;
    for sq in 0..64 {
        let tmp = b.get_piece_color_at_square(sq);
        match tmp {
            Some(c) => piece_color = c,
            None => continue,
        }
        let piece_type = match b.get_piece_type_at_square(sq) {
            Some(t) => t,
            None => {
                eprintln!(
                    "ERROR at square {}: color={:?}, but no type!",
                    sq, piece_color
                );
                continue;
            }
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

    if !b.is_white_turn { -score } else { score }
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

fn evaluate_center_control(b: &Board, color: Color) -> i32 {
    let center_squares = 0x0000001818000000u64;
    let extended_center = 0x00003C3C3C3C0000u64;
    let pawns = b.get_pieces(color, Type::Pawn).get_value();
    let all_pieces = b.get_pieces(color, Type::Any).get_value();

    let center_pawns = (pawns & center_squares).count_ones();
    let center_pieces = (all_pieces & extended_center).count_ones();

    (center_pawns as i32) * 30 + (center_pieces as i32) * 10
}

fn evaluate_development(b: &Board, color: Color) -> i32 {
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
    if (queens.get_value() & back_rank) == 0 && undeveloped_knights > 0 {
        score -= 40;
    }

    if b.has_castled(color) {
        score += 60;
    }

    score
}
fn get_piece_value(t: Type) -> i32 {
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
