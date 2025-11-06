use crate::chess::table::{Color, Type};
use crate::engine::perft::{self, perft};
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
    let total_moves = perft(&b, 2);
    println!("{}", total_moves)
}
