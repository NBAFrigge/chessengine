use crate::engine::perft::{perft, perft_divide};
mod bitboard;
mod chess;
mod engine;

fn main() {
    let b = chess::table::Board::new();

    // let bb = Bitboard::Bitboard::Bitboard::new(258);
    //
    // println!("{}", bb.to_string());
    // println!("------------------------------");
    // let bb2 = Bitboard::Bitboard::Bitboard::new(bb.lsb());
    // println!("{}", bb2.to_string());
    perft_divide(&b, 4);
    // println!("{}", total_moves)
}
