use crate::engine::perft::{start_perft, start_perft_divide, start_perft_plus};
mod bitboard;
mod chess;
mod engine;
use chess::moves_gen::{magic_bitboards, moves_struct};
use std::env;
use std::time::Instant;

fn main() {
    magic_bitboards::init();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("no arguments")
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
                "p" | "v" => {
                    flag = args[2].to_string();
                    depth_index = 3
                }
                _ => {}
            }
            let depth: u8 = match args[depth_index].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("'{}' not a valid depth", args[depth_index]);
                    return;
                }
            };
            start_perft_analyses(flag, depth);
        }

        _ => {
            panic!("unknow argument {}", command.as_str())
        }
    }
}

fn start_perft_analyses(flag: String, depth: u8) {
    match flag.as_str() {
        "p" => start_perft_plus(depth),
        "v" => start_perft_divide(depth),
        _ => start_perft(depth),
    }
}
