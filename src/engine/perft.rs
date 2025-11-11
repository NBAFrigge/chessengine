use crate::chess::moves_gen::moves_struct;
use crate::chess::moves_gen::moves_struct::{FLAG_CAPTURE, FLAG_CASTLE, FLAG_EN_PASSANT, Moves};
use crate::chess::table::Board;
use crate::chess::table::Color;
use std::time::Instant;

struct PerftResult {
    pub nodes: u64,
    pub captures: u64,
    pub en_passant: u64,
    pub castles: u64,
    pub checks: u64,
}

impl PerftResult {
    #[inline(always)]
    fn new() -> Self {
        PerftResult {
            nodes: 0,
            captures: 0,
            en_passant: 0,
            castles: 0,
            checks: 0,
        }
    }

    #[inline(always)]
    fn add(&mut self, other: &PerftResult) {
        self.nodes += other.nodes;
        self.captures += other.captures;
        self.en_passant += other.en_passant;
        self.castles += other.castles;
        self.checks += other.checks;
    }
}

const MAX_MOVES: usize = 256;

#[inline]
fn perft_inner(b: &mut Board, depth: u8, move_buffers: &mut [Vec<Moves>]) -> u64 {
    if depth == 0 {
        return 1;
    }

    let turn = if b.is_white_turn {
        Color::White
    } else {
        Color::Black
    };

    let buffer_index = depth as usize;
    let move_buffer = &mut move_buffers[buffer_index];

    move_buffer.clear();
    let moves = b.get_all_moves_bitboard(turn, move_buffer);
    let move_count = moves.len();

    if depth == 1 {
        let mut legal_count = 0;
        for i in 0..move_count {
            let mv = move_buffers[buffer_index][i];
            let undo = b.make_move_with_undo(&mv);

            if !b.is_king_in_check(turn) {
                legal_count += 1;
            }

            b.unmake_move(&mv, undo);
        }
        return legal_count;
    }

    let mut total_moves = 0;

    for i in 0..move_count {
        let mv = move_buffers[buffer_index][i];
        let undo = b.make_move_with_undo(&mv);

        if !b.is_king_in_check(turn) {
            total_moves += perft_inner(b, depth - 1, move_buffers);
        }

        b.unmake_move(&mv, undo);
    }

    total_moves
}

pub fn perft(b: &mut Board, depth: u8) -> u64 {
    let mut move_buffers: Vec<Vec<Moves>> =
        (0..=depth).map(|_| Vec::with_capacity(MAX_MOVES)).collect();

    perft_inner(b, depth, &mut move_buffers)
}

fn perft_divide_inner(b: &mut Board, depth: u8, move_buffers: &mut [Vec<Moves>]) -> u64 {
    let turn = if b.is_white_turn {
        Color::White
    } else {
        Color::Black
    };

    let buffer_index = depth as usize;
    let move_buffer = &mut move_buffers[buffer_index];

    move_buffer.clear();
    let moves = b.get_all_moves_bitboard(turn, move_buffer);
    let move_count = moves.len();

    let mut total = 0;

    for i in 0..move_count {
        let mv = move_buffers[buffer_index][i];
        let undo = b.make_move_with_undo(&mv);

        if !b.is_king_in_check(turn) {
            let count = if depth <= 1 {
                1
            } else {
                perft_inner(b, depth - 1, move_buffers)
            };
            println!("{} -> {}: {}", mv.from(), mv.to(), count);
            total += count;
        }

        b.unmake_move(&mv, undo);
    }

    total
}

pub fn perft_divide(b: &mut Board, depth: u8) -> u64 {
    let mut move_buffers: Vec<Vec<Moves>> =
        (0..=depth).map(|_| Vec::with_capacity(MAX_MOVES)).collect();

    perft_divide_inner(b, depth, &mut move_buffers)
}

#[inline]
fn perft_plus_inner(b: &mut Board, depth: u8, move_buffers: &mut [Vec<Moves>]) -> PerftResult {
    if depth == 0 {
        return PerftResult {
            nodes: 1,
            captures: 0,
            en_passant: 0,
            castles: 0,
            checks: 0,
        };
    }

    let turn = if b.is_white_turn {
        Color::White
    } else {
        Color::Black
    };

    let buffer_index = depth as usize;
    let move_buffer = &mut move_buffers[buffer_index];

    move_buffer.clear();
    let moves = b.get_all_moves_bitboard(turn, move_buffer);
    let move_count = moves.len();

    let mut result = PerftResult::new();

    if move_count == 0 {
        return result;
    }

    if depth == 1 {
        for i in 0..move_count {
            let mv = move_buffers[buffer_index][i];
            let undo = b.make_move_with_undo(&mv);

            if !b.is_king_in_check(turn) {
                result.nodes += 1;

                let flags = mv.flags();
                if flags == FLAG_CAPTURE {
                    result.captures += 1;
                } else if flags == FLAG_EN_PASSANT {
                    result.en_passant += 1;
                } else if flags == FLAG_CASTLE {
                    result.castles += 1;
                }

                let opponent_color = if b.is_white_turn {
                    Color::Black
                } else {
                    Color::White
                };

                if b.is_king_in_check(opponent_color) {
                    result.checks += 1;
                }
            }

            b.unmake_move(&mv, undo);
        }
        return result;
    }

    for i in 0..move_count {
        let mv = move_buffers[buffer_index][i];
        let undo = b.make_move_with_undo(&mv);

        if !b.is_king_in_check(turn) {
            let sub_result = perft_plus_inner(b, depth - 1, move_buffers);
            result.add(&sub_result);
        }

        b.unmake_move(&mv, undo);
    }

    result
}

pub fn perft_plus(b: &mut Board, depth: u8) -> PerftResult {
    let mut move_buffers: Vec<Vec<Moves>> =
        (0..=depth).map(|_| Vec::with_capacity(MAX_MOVES)).collect();

    perft_plus_inner(b, depth, &mut move_buffers)
}

pub fn start_perft(depth: u8) {
    let mut board = Board::new();
    let start = Instant::now();

    let result = perft(&mut board, depth);

    let duration = start.elapsed();

    println!("perft({}) = {}", depth, result);
    println!("elapsed time: {:?}", duration);
    println!("elapsed time (ms): {}", duration.as_millis());
    println!(
        "{:.2} MNodes/s",
        (result as f64 / duration.as_secs_f64()) / 1_000_000.0
    )
}

pub fn start_perft_divide(depth: u8) {
    let mut board = Board::new();
    let start = Instant::now();

    let result = perft_divide(&mut board, depth);

    let duration = start.elapsed();

    println!("\nTotal: {}", result);
    println!("elapsed time: {:?}", duration);
    println!("elapsed time (ms): {}", duration.as_millis());
    println!(
        "{:.2} MNodes/s",
        (result as f64 / duration.as_secs_f64()) / 1_000_000.0
    )
}

pub fn start_perft_plus(depth: u8) {
    let mut board = Board::new();
    let start = Instant::now();

    let result = perft_plus(&mut board, depth);

    let duration = start.elapsed();

    println!("elapsed time: {:?}", duration);
    println!("elapsed time (ms): {}", duration.as_millis());
    println!(
        "{:.2} MNodes/s",
        (result.nodes as f64 / duration.as_secs_f64()) / 1_000_000.0
    );
    println!(
        "perft({}):\nnodes: {}\ncaptures: {}\ncastles: {}\nep: {}\nchecks: {}",
        depth, result.nodes, result.captures, result.castles, result.en_passant, result.checks
    );
}
