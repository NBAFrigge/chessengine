use crate::chess::table::{Board, Color, Type};

pub fn evaluate_bishop_pair(b: &Board, color: Color) -> i32 {
    let bishops = b.get_pieces(color, Type::Bishop);

    if bishops.count_ones() >= 2 {
        return 50;
    }

    0
}
