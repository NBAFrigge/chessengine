use crate::chess::table::Board;
use crate::chess::table::{Color, Type};
pub fn evaluate(b: &Board) -> i32 {
    (b.get_pieces(Color::White, Type::Pawn).count_ones() as i32
        - b.get_pieces(Color::Black, Type::Pawn).count_ones() as i32)
        * 100
        + (b.get_pieces(Color::White, Type::King).count_ones() as i32
            - b.get_pieces(Color::Black, Type::King).count_ones() as i32)
            * 99999
        + (b.get_pieces(Color::White, Type::Knight).count_ones() as i32
            - b.get_pieces(Color::Black, Type::Knight).count_ones() as i32)
            * 320
        + (b.get_pieces(Color::White, Type::Queen).count_ones() as i32
            - b.get_pieces(Color::Black, Type::Queen).count_ones() as i32)
            * 900
        + (b.get_pieces(Color::White, Type::Bishop).count_ones() as i32
            - b.get_pieces(Color::Black, Type::Bishop).count_ones() as i32)
            * 330
        + (b.get_pieces(Color::White, Type::Rook).count_ones() as i32
            - b.get_pieces(Color::Black, Type::Rook).count_ones() as i32)
            * 500
}
