use lazy_static::lazy_static;
use rand::RngCore;

use crate::chess::{
    moves_gen::moves_struct::Moves,
    table::{Board, Color, Type},
};

pub struct ZobristTable {
    pub pieces: [[u64; 64]; 12], // 0-5:  White Pawn, Knight, Bishop, Rook, Queen, King | 6-11: Black Pawn, Knight, Bishop, Rook, Queen, King
    pub castling_rights: [u64; 16],
    en_passant_file: [u64; 8],
    black_to_move: u64,
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

    pub fn update_hash_before(&self, old_hash: u64, b: &Board, mv: &Moves) -> u64 {
        self.compute_hash(b)
        // let mut new_hash = old_hash;
        // let sq_to = mv.to();
        // let sq_from = mv.from();
        // let piece_type = b.get_piece_type_at_square(sq_from).unwrap();
        // let piece_color = b.get_piece_color_at_square(sq_from).unwrap();
        //
        // let piece_index = get_piece_index(piece_color, piece_type);
        //
        // new_hash ^= self.pieces[piece_index][sq_to as usize];
        // new_hash ^= self.pieces[piece_index][sq_from as usize];
        //
        // if mv.is_capture() {
        //     if mv.is_enpassant() {
        //         let captured_pawn_sq = if piece_color == Color::White {
        //             sq_to - 8
        //         } else {
        //             sq_to + 8
        //         };
        //         let opponent_color = if piece_color == Color::White {
        //             Color::Black
        //         } else {
        //             Color::White
        //         };
        //         let pawn_index = get_piece_index(opponent_color, Type::Pawn);
        //         new_hash ^= self.pieces[pawn_index][captured_pawn_sq as usize];
        //     } else {
        //         let capture_type = b.get_piece_type_at_square(sq_to).unwrap();
        //         let capture_color = b.get_piece_color_at_square(sq_to).unwrap();
        //         let capture_index = get_piece_index(capture_color, capture_type);
        //         new_hash ^= self.pieces[capture_index][sq_to as usize];
        //     }
        // }
        //
        // if mv.is_promotion() {
        //     let promoted_piece = mv.promotion_piece_type();
        //     let promoted_index = get_piece_index(piece_color, promoted_piece);
        //
        //     new_hash ^= self.pieces[piece_index][sq_to as usize];
        //
        //     new_hash ^= self.pieces[promoted_index][sq_to as usize];
        // }
        //
        // if mv.is_castle() {
        //     let (rook_from, rook_to) = get_rook_squares_for_castling(sq_from, sq_to);
        //     let rook_index = get_piece_index(piece_color, Type::Rook);
        //     new_hash ^= self.pieces[rook_index][rook_from];
        //     new_hash ^= self.pieces[rook_index][rook_to];
        // }
        //
        // if b.enpassant.get_value() != 0 {
        //     let old_ep_sq = b.enpassant.lsb();
        //     let file = (old_ep_sq % 8) as usize;
        //     new_hash ^= self.en_passant_file[file]
        // }
        //
        // if piece_type == Type::Pawn {
        //     let distance = (sq_to as i8 - sq_from as i8).abs();
        //     if distance == 16 {
        //         let new_ep_file = (sq_to % 8) as usize;
        //         new_hash ^= self.en_passant_file[new_ep_file];
        //     }
        // }
        //
        // new_hash ^= self.black_to_move;
        //
        // new_hash
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
