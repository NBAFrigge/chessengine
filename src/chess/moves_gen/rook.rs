use crate::chess::moves_gen::magic_bitboards;

pub fn moves(b: u64, occ: u64) -> u64 {
    magic_bitboards::rook_moves(b, occ)
}
