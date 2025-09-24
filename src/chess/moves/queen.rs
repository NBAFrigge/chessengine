use crate::chess::moves;

pub fn moves(b: u64, occupied: u64) -> u64{
    let mask = generate_mask(b.trailing_zeros() as u64);
    let blockers = get_blockers(mask, occupied);
    mask & !blockers
}

pub fn attacks(b: u64, opponent_piece: u64) -> u64{
    let mask = generate_mask(b);
    get_blockers(mask, opponent_piece)
}

fn get_blockers(mask : u64, occ : u64) -> u64 {
    mask & occ
}

fn generate_mask(index : u64) -> u64 {
    moves::bishop::generate_mask(index) | moves::rook::generate_mask(index)
}