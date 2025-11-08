use crate::engine::perft::{perft, perft_divide};
mod bitboard;
mod chess;
mod engine;
use chess::moves_gen::magic_bitboards;
use std::time::Instant;

fn main() {
    magic_bitboards::init();
    let board = chess::table::Board::new();
    let depth = 5;
    let start = Instant::now();
    let result = perft(&board, depth);
    let duration = start.elapsed();

    println!("perft({}) = {}", depth, result);
    println!("elapsed time: {:?}", duration);
    println!("elapsed time (ms): {}", duration.as_millis());
}
