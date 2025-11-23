use crate::chess::moves_gen::{bishop, knight, queen, rook};
use crate::chess::table::{Board, Color, Type};

pub fn evaluate_mobility(b: &Board, color: Color, phase: f32) -> i32 {
    let mut score = 0;
    let occupied = b.get_occupied_pos().get_value();

    let knights = b.get_pieces(color, Type::Knight);
    for knight in knights.iter_bits() {
        let moves = knight::moves(knight.get_value());
        let legal_moves = moves & !b.get_pieces(color, Type::Any).get_value();
        score += (legal_moves.count_ones() as i32) * 4;
    }

    let bishops = b.get_pieces(color, Type::Bishop);
    for bishop in bishops.iter_bits() {
        let moves = bishop::moves(bishop.get_value(), occupied);
        let legal_moves = moves & !b.get_pieces(color, Type::Any).get_value();
        score += (legal_moves.count_ones() as i32) * 3;
    }

    let rooks = b.get_pieces(color, Type::Rook);
    for rook in rooks.iter_bits() {
        let moves = rook::moves(rook.get_value(), occupied);
        let legal_moves = moves & !b.get_pieces(color, Type::Any).get_value();
        score += (legal_moves.count_ones() as i32) * 2;
    }

    let queens = b.get_pieces(color, Type::Queen);
    for queen in queens.iter_bits() {
        let moves = queen::moves(queen.get_value(), occupied);
        let legal_moves = moves & !b.get_pieces(color, Type::Any).get_value();
        score += (legal_moves.count_ones() as i32) * 1;
    }

    (score as f32 * phase) as i32
}
