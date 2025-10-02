use crate::bitboard::bitboard::Bitboard;
pub struct Moves {
    pub old_pos: Bitboard,
    pub new_pos: Bitboard,
}

impl Moves {
    pub fn new(old : Bitboard, new : Bitboard) -> Self {
        Moves {
            old_pos : old,
            new_pos : new
        }
    }
}