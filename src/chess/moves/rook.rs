const NOTBORDER: u64 = 0x7e7e7e7e7e7e00;

const FILE :u64 = 0x0101010101010101;
const RANK :u64 = 0xff;


pub fn moves(b: u64, empty: u64) -> u64 {
    let mask = generate_mask(b.trailing_zeros() as u64);
    let blockers = get_blockers(mask, !empty);
    mask & !blockers
}

pub fn attack(b : u64, opponent_piece: u64) -> u64 {
    let mask = generate_mask(b);
    get_blockers(mask, opponent_piece)
}

fn get_blockers(mask : u64, occ : u64) -> u64 {
    mask & occ
}

pub fn generate_mask(index: u64) -> u64 {
    let table :u64 = 1 << index;
    let rank = index / 8;
    let file = index % 8;

    ((FILE << file |
    RANK << 8 * rank) ^
    table)
}

