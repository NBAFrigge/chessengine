use crate::chess::table::Board;

use crate::engine::find_best_move::Engine;
use crate::engine::perft::{start_perft, start_perft_divide, start_perft_fen, start_perft_plus};
use crate::uci::uci::UciEngine;
mod bitboard;
mod chess;
mod engine;
mod uci;
use chess::moves_gen::magic_bitboards;
use std::{env, vec};
fn main() {
    magic_bitboards::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        start_uci();
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "perft" => {
            if args.len() < 3 {
                panic!("depth not found")
            }
            let mut flag = String::new();
            let mut depth_index = 2;
            match args[2].as_str() {
                "p" | "v" | "f" => {
                    flag = args[2].to_string();
                    depth_index = 3
                }
                _ => {}
            }
            let mut fen = String::new();
            if args.len() == 5 && flag == "f" {
                depth_index += 1;
                fen = args[3].to_string()
            }
            let depth: u8 = match args[depth_index].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("'{}' not a valid depth", args[depth_index]);
                    return;
                }
            };
            start_perft_analyses(flag, fen, depth);
        }
        "search" => start_search(),
        _ => {
            panic!("unknow argument {}", command.as_str())
        }
    }
}

fn start_perft_analyses(flag: String, fen: String, depth: u8) {
    match flag.as_str() {
        "p" => start_perft_plus(depth),
        "v" => start_perft_divide(depth),
        "f" => {
            if fen.is_empty() {
                eprintln!("FEN string is empty or missing.");
                return;
            }
            start_perft_fen(fen.as_str(), depth)
        }
        _ => start_perft(depth),
    }
}

fn start_search() {
    let test_cases = vec![
        (
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "1. Initial Position (Opening)",
        ),
        ("8/8/8/8/5R2/8/5k2/5K2 w - - 0 1", "2. Mate in 1 (White)"),
        (
            "8/8/5K2/8/8/7Q/6kr/8 b - - 0 1",
            "3. Forced Stalemate (Black)",
        ),
        (
            "rnbqkb1r/pp3ppp/3p1n2/2p5/4P3/5N2/PPPPBPPP/RNBQ1RK1 b kq - 3 6",
            "4. Tactical Gain (Black)",
        ),
        (
            "8/6pk/8/8/2B5/7P/2PP2P1/5RK1 w - - 0 35",
            "5. Mate in 2 (White)",
        ),
    ];

    let mut engine = Engine::new();

    for (fen, name) in test_cases {
        match Board::new_from_fen(fen) {
            Ok(b) => {
                let start_time = std::time::Instant::now();
                let game_history: Vec<u64> = Vec::new();

                let best_move = engine.find_best_move(&b, 5, &game_history, Some(10000000));
                let elapsed = start_time.elapsed();

                let from_sq = best_move.from();
                let to_sq = best_move.to();
                let promotion_type = best_move.promotion_piece();
                let is_promo = best_move.is_promotion();

                let from_file = (b'a' + (from_sq % 8)) as char;
                let from_rank = (b'1' + (from_sq / 8)) as char;
                let to_file = (b'a' + (to_sq % 8)) as char;
                let to_rank = (b'1' + (to_sq / 8)) as char;

                let mut move_str = format!("{}{}{}{}", from_file, from_rank, to_file, to_rank);
                if is_promo {
                    let promo_char = match promotion_type {
                        0 => 'q',
                        1 => 'r',
                        2 => 'b',
                        3 => 'n',
                        _ => '?',
                    };
                    move_str.push(promo_char);
                }

                println!("--- {} ---", name);
                println!("FEN: {}", fen);
                println!("Best Move Found: {}", move_str);
                println!("Search Time: {} ms", elapsed.as_millis());
                println!();
            }
            Err(e) => {
                println!("FEN PARSING ERROR for {}: {}", fen, e);
            }
        }
    }
}

fn start_uci() {
    let mut engine = UciEngine::new();
    engine.run();
}
