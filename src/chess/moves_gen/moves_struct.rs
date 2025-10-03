use crate::bitboard::bitboard::Bitboard;
pub enum   MoveType {
    Simple,
    LongCastle,
    ShortCastle,
}

pub struct Moves {
    pub old_pos: Bitboard,
    pub new_pos: Bitboard,
    pub move_type: MoveType,
}

impl Moves {
    pub fn new(old : Bitboard, new : Bitboard) -> Self {
        Moves {
            old_pos : old,
            new_pos : new,
            move_type : MoveType::Simple,
        }
    }

    pub fn castling(castle_type: MoveType) -> Self {
        Moves {
            old_pos : Bitboard::empty(),
            new_pos : Bitboard::empty(),
            move_type : castle_type,
        }
    }
}