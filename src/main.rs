use crate::engine::perft::{perft, perft_divide};
mod bitboard;
mod chess;
mod engine;
use chess::moves_gen::{magic_bitboards, moves_struct};
use std::time::Instant;

fn main() {
    magic_bitboards::init();
    let mut board = chess::table::Board::new();
    let depth = 6;
    let start = Instant::now();
    let mut move_buffer: Vec<moves_struct::Moves> = Vec::with_capacity(218);
    let result = perft(&mut board, depth, &mut move_buffer);
    let duration = start.elapsed();

    println!("perft({}) = {}", depth, result);
    //println!(
    //    "perft({}): \nnodes: {}\ncaptures: {}\ncastles: {}\nep: {}\nchecks: {}",
    //    depth, result.nodes, result.captures, result.castles, result.en_passant, result.checks
    //);
    println!("elapsed time: {:?}", duration);
    println!("elapsed time (ms): {}", duration.as_millis());
    println!(
        "{} MNodes/S",
        (result as f64 / duration.as_secs_f64()) / 1_000_000 as f64
    )
}
