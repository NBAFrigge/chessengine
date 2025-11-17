use crate::chess::table::Board;
use crate::chess::table::{Color, Type};
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
    let mut piece_color;
    let mut piece_type;
    let mut score = 0;
    for sq in 0..64 {
        let tmp = b.get_piece_color_at_square(sq);
        match tmp {
            Some(c) => piece_color = c,
            None => continue,
        }
        piece_type = b.get_piece_type_at_square(sq).unwrap();
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

    score
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
