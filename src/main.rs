use crate::engine::perft::{perft, perft_divide};
mod bitboard;
mod chess;
mod engine;
use std::time::Instant;

fn main() {
    let board = chess::table::Board::new();
    let depth = 7;
    let start = Instant::now();
    let result = perft(&board, depth);
    let duration = start.elapsed();

    println!("perft({}) = {}", depth, result);
    println!("elapsed time: {:?}", duration);
    println!("elapsed time (ms): {}", duration.as_millis());
}
