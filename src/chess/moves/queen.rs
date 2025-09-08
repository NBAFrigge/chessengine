use crate::chess::moves;

pub fn generatemask(index : u64) -> u64 {
    moves::bishop::generateMask(index) | moves::rook::generateMask(index)
}