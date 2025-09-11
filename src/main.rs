use crate::chess::table::{Color, Type};

mod chess;
mod bitboard;
mod engine;

fn main() {
    let b = chess::table::Board::new();

    // let bb = Bitboard::Bitboard::Bitboard::new(258);
    //
    // println!("{}", bb.to_string());
    // println!("------------------------------");
    // let bb2 = Bitboard::Bitboard::Bitboard::new(bb.lsb());
    // println!("{}", bb2.to_string());

    println!("{}", b.to_string());
    let mut move_vec = b.get_move(Color::White, Type::Pawn);
    println!("{}", move_vec.len());
    for m in move_vec.iter() {
        println!("________________________________________");
        println!("{}", m.to_formatted_string());
    }

    
}