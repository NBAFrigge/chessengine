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
                eprintln!("  White bitboard: {:064b}", b.white.get_value());
                eprintln!("  Black bitboard: {:064b}", b.black.get_value());
                eprintln!("  Pawn: {:064b}", b.pawn.get_value());
                eprintln!("  Knight: {:064b}", b.knight.get_value());
                eprintln!("  Bishop: {:064b}", b.bishop.get_value());
                eprintln!("  Rook: {:064b}", b.rook.get_value());
                eprintln!("  Queen: {:064b}", b.queen.get_value());
                eprintln!("  King: {:064b}", b.king.get_value());
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
