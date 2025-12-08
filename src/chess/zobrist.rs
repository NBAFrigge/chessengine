use lazy_static::lazy_static;
use rand::RngCore;

use crate::chess::{
    moves_gen::moves_struct::Moves,
    table::{Board, Color, Type},
};

pub struct ZobristTable {
    pub pieces: [[u64; 64]; 12], // 0-5:  White Pawn, Knight, Bishop, Rook, Queen, King | 6-11: Black Pawn, Knight, Bishop, Rook, Queen, King
    pub castling_rights: [u64; 16],
    pub en_passant_file: [u64; 8],
    pub black_to_move: u64,
}

lazy_static! {
    pub static ref ZOBRIST: ZobristTable = ZobristTable::new();
}

impl ZobristTable {
    pub fn new() -> Self {
        let mut z = ZobristTable {
            pieces: [[0; 64]; 12],
            castling_rights: [0; 16],
            en_passant_file: [0; 8],
            black_to_move: 0,
        };
        let mut rng = rand::rng();
        for i in 0..12 {
            for j in 0..64 {
                z.pieces[i][j] = rng.next_u64()
            }
        }

        for i in 0..16 {
            z.castling_rights[i] = rng.next_u64()
        }

        for i in 0..8 {
            z.en_passant_file[i] = rng.next_u64()
        }

        z.black_to_move = rng.next_u64();
        z
    }

    pub fn compute_hash(&self, b: &Board) -> u64 {
        let mut hash = 0;

        for p in b.white.iter_bits() {
            let sq = p.lsb() as usize;
            let piece_type = b.get_piece_type_at_square(sq as u8).unwrap();
            let piece_type_id = piece_type.id() as usize;
            hash ^= self.pieces[piece_type_id][sq];
        }

        for p in b.black.iter_bits() {
            let sq = p.lsb() as usize;
            let piece_type = b.get_piece_type_at_square(sq as u8).unwrap();
            let piece_type_id = piece_type.id() as usize;
            hash ^= self.pieces[piece_type_id + 6][sq];
        }

        let mut castle_index = 0;
        if b.white_rook_long_side {
            castle_index |= 0b0001
        }
        if b.white_rook_short_side {
            castle_index |= 0b0010
        }
        if b.black_rook_long_side {
            castle_index |= 0b0100
        }
        if b.black_rook_short_side {
            castle_index |= 0b1000
        }

        hash ^= self.castling_rights[castle_index];

        if b.enpassant.get_value() != 0 {
            let sq = b.enpassant.lsb();
            let file = (sq % 8) as usize;
            hash ^= self.en_passant_file[file]
        }

        if !b.is_white_turn {
            hash ^= self.black_to_move
        }

        hash
    }

    pub fn update_hash_incremental(
        &self,
        old_hash: u64,
        b: &Board,
        mv: &Moves,
        moving_color: Color,
        captured_piece_type: Option<Type>,
        captured_on_white: bool,
        old_enpassant: u64,
        old_castling_index: usize,
    ) -> u64 {
        let mut new_hash = old_hash;

        let from_sq = mv.from() as usize;
        let to_sq = mv.to() as usize;

        // enpassant
        let old_ep_file = if old_enpassant != 0 {
            Some((old_enpassant.trailing_zeros() % 8) as usize)
        } else {
            None
        };

        let new_ep_file = if b.enpassant.get_value() != 0 {
            Some((b.enpassant.lsb() % 8) as usize)
        } else {
            None
        };

        if old_ep_file != new_ep_file {
            if let Some(old_file) = old_ep_file {
                new_hash ^= self.en_passant_file[old_file];
            }
            if let Some(new_file) = new_ep_file {
                new_hash ^= self.en_passant_file[new_file];
            }
        }

        // castling
        let new_castling_index = b.get_castling_index();
        if old_castling_index != new_castling_index {
            new_hash ^= self.castling_rights[old_castling_index];
            new_hash ^= self.castling_rights[new_castling_index];
        }

        // move
        if mv.is_castle() {
            let king_index = get_piece_index(moving_color, Type::King);
            let rook_index = get_piece_index(moving_color, Type::Rook);

            new_hash ^= self.pieces[king_index][from_sq];
            new_hash ^= self.pieces[king_index][to_sq];

            let (rook_from, rook_to) = get_rook_squares_for_castling(from_sq as u8, to_sq as u8);
            new_hash ^= self.pieces[rook_index][rook_from];
            new_hash ^= self.pieces[rook_index][rook_to];
        } else {
            let moving_piece_type = if mv.is_promotion() {
                Type::Pawn
            } else {
                b.get_piece_type_at_square(to_sq as u8)
                    .unwrap_or(Type::Pawn)
            };

            let moving_piece_index = get_piece_index(moving_color, moving_piece_type);

            new_hash ^= self.pieces[moving_piece_index][from_sq];

            if mv.is_capture() && !mv.is_enpassant() {
                if let Some(captured_type) = captured_piece_type {
                    let captured_color = if captured_on_white {
                        Color::White
                    } else {
                        Color::Black
                    };
                    let captured_index = get_piece_index(captured_color, captured_type);
                    new_hash ^= self.pieces[captured_index][to_sq];
                }
            }

            if mv.is_enpassant() {
                let captured_pawn_sq = if moving_color == Color::White {
                    to_sq - 8
                } else {
                    to_sq + 8
                };
                let opponent_color = if moving_color == Color::White {
                    Color::Black
                } else {
                    Color::White
                };
                let pawn_index = get_piece_index(opponent_color, Type::Pawn);
                new_hash ^= self.pieces[pawn_index][captured_pawn_sq];
            }

            if mv.is_promotion() {
                let promoted_type = mv.promotion_piece_type();
                let promoted_index = get_piece_index(moving_color, promoted_type);
                new_hash ^= self.pieces[promoted_index][to_sq];
            } else {
                new_hash ^= self.pieces[moving_piece_index][to_sq];
            }
        }

        new_hash ^= self.black_to_move;

        new_hash
    }
}

//helper
fn get_piece_index(c: Color, t: Type) -> usize {
    match c {
        Color::White => t.id() as usize,
        Color::Black => (t.id() + 6) as usize,
    }
}

fn get_rook_squares_for_castling(king_from: u8, king_to: u8) -> (usize, usize) {
    match (king_from, king_to) {
        (4, 6) => (7, 5),
        (4, 2) => (0, 3),
        (60, 62) => (63, 61),
        (60, 58) => (56, 59),
        _ => panic!("Invalid castling move: from {} to {}", king_from, king_to),
    }
}
