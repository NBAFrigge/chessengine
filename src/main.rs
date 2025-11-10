use crate::engine::perft::{perft, perft_divide};
mod bitboard;
mod chess;
mod engine;
use chess::moves_gen::magic_bitboards;
use std::time::Instant;

fn main() {
    magic_bitboards::init();
    let mut board = chess::table::Board::new();
    let depth = 7;
    let start = Instant::now();
    let result = perft(&mut board, depth);
    let duration = start.elapsed();

    println!("perft({}) = {}", depth, result);
    //println!(
    //    "perft({}): \nnodes: {}\ncaptures: {}\ncastles: {}\nep: {}\nchecks: {}",
    //    depth, result.nodes, result.captures, result.castles, result.en_passant, result.checks
    //);
    println!("elapsed time: {:?}", duration);
    println!("elapsed time (ms): {}", duration.as_millis());
}
