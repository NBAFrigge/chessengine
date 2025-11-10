use crate::chess::moves_gen::moves_struct::{FLAG_CAPTURE, FLAG_CASTLE, FLAG_EN_PASSANT};
use crate::chess::table::Board;
use crate::chess::table::Color;

pub struct PerftResult {
    pub nodes: u64,
    pub captures: u64,
    pub en_passant: u64,
    pub castles: u64,
    pub checks: u64,
}

impl PerftResult {
    fn new() -> Self {
        return PerftResult {
            nodes: 0,
            captures: 0,
            en_passant: 0,
            castles: 0,
            checks: 0,
        };
    }
    fn add(&mut self, other: &PerftResult) {
        self.nodes += other.nodes;
        self.captures += other.captures;
        self.en_passant += other.en_passant;
        self.castles += other.castles;
        self.checks += other.checks;
    }
}

pub fn perft(b: &mut Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let turn = if b.is_white_turn {
        Color::White
    } else {
        Color::Black
    };

    let moves = b.get_all_moves_bitboard(turn);

    if depth == 1 {
        let mut count = 0;
        for mv in moves {
            let undo = b.make_move_with_undo(&mv);
            if !b.is_king_in_check(turn) {
                count += 1;
            }
            b.unmake_move(&mv, undo);
        }
        return count;
    }

    let mut total_moves = 0;
    for mv in moves {
        let undo = b.make_move_with_undo(&mv);
        if !b.is_king_in_check(turn) {
            total_moves += perft(b, depth - 1);
        }
        b.unmake_move(&mv, undo);
    }

    total_moves
}

pub fn perft_divide(b: &mut Board, depth: u8) {
    let turn = if b.is_white_turn {
        Color::White
    } else {
        Color::Black
    };

    let moves = b.get_all_moves_bitboard(turn);
    let mut total = 0;

    for mv in moves {
        let undo = b.make_move_with_undo(&mv);

        if !b.is_king_in_check(turn) {
            let count = if depth <= 1 { 1 } else { perft(b, depth - 1) };
            println!("{} -> {}: {}", mv.from(), mv.to(), count);
            total += count;
        }

        b.unmake_move(&mv, undo);
    }
    println!("Total: {}", total);
}

pub fn perft_plus(b: &mut Board, depth: u8) -> PerftResult {
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

    let moves = b.get_all_moves_bitboard(turn);
    let mut result = PerftResult::new();

    if moves.is_empty() {
        return result;
    }

    if depth == 1 {
        for mv in &moves {
            let undo = b.make_move_with_undo(&mv);

            if !b.is_king_in_check(turn) {
                result.nodes += 1;

                if mv.flags() == FLAG_CAPTURE {
                    result.captures += 1;
                } else if mv.flags() == FLAG_EN_PASSANT {
                    result.en_passant += 1;
                } else if mv.flags() == FLAG_CASTLE {
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

    for mv in moves {
        let undo = b.make_move_with_undo(&mv);

        if !b.is_king_in_check(turn) {
            let sub_result = perft_plus(b, depth - 1);
            result.add(&sub_result);
        }

        b.unmake_move(&mv, undo);
    }

    result
}
