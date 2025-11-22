const NOT_AFILE: u64 = 0xfefefefefefefefe;
const NOT_HFILE: u64 = 0x7f7f7f7f7f7f7f7f;
const RANK4: u64 = 0x00000000FF000000;
const RANK5: u64 = 0x000000FF00000000;

pub fn white_moves(b: u64, empty: u64) -> u64 {
    let single_push = white_pawns_able_to_push(b, empty);
    let double_push = white_pawns_able_double_push(b, empty);

    white_single_push(single_push, empty) | white_double_push(double_push, empty)
}

pub fn black_moves(b: u64, empty: u64) -> u64 {
    let single_push = black_pawns_able_to_push(b, empty);
    let double_push = black_pawns_able_double_push(b, empty);

    black_single_push(single_push, empty) | black_double_push(double_push, empty)
}

pub fn white_attack(b: u64, black: u64) -> u64 {
    let att = northeast_one(b) | northwest_one(b);
    att & black
}

pub fn black_attack(b: u64, white: u64) -> u64 {
    let att = southeast_one(b) | southwest_one(b);
    att & white
}

fn white_single_push(b: u64, empty: u64) -> u64 {
    north_one(b) & empty
}

fn white_double_push(b: u64, empty: u64) -> u64 {
    let single = white_single_push(b, empty);
    north_one(single) & empty & RANK4
}

fn white_pawns_able_to_push(b: u64, empty: u64) -> u64 {
    south_one(empty) & b
}

fn white_pawns_able_double_push(b: u64, empty: u64) -> u64 {
    let empty_rank = south_one(empty & RANK4) & empty;
    white_pawns_able_to_push(b, empty_rank)
}

fn black_single_push(b: u64, empty: u64) -> u64 {
    south_one(b) & empty
}

fn black_double_push(b: u64, empty: u64) -> u64 {
    let single = black_single_push(b, empty);
    south_one(single) & empty & RANK5
}

fn black_pawns_able_to_push(b: u64, empty: u64) -> u64 {
    north_one(empty) & b
}

fn black_pawns_able_double_push(b: u64, empty: u64) -> u64 {
    let empty_rank = north_one(empty & RANK5) & empty;
    black_pawns_able_to_push(b, empty_rank)
}

fn north_one(b: u64) -> u64 {
    b << 8
}
fn south_one(b: u64) -> u64 {
    b >> 8
}
fn east_one(b: u64) -> u64 {
    (b << 1) & NOT_AFILE
}
fn northeast_one(b: u64) -> u64 {
    (b << 9) & NOT_AFILE
}
fn southeast_one(b: u64) -> u64 {
    (b >> 7) & NOT_AFILE
}
fn west_one(b: u64) -> u64 {
    (b >> 1) & NOT_HFILE
}
fn southwest_one(b: u64) -> u64 {
    (b >> 9) & NOT_HFILE
}
fn northwest_one(b: u64) -> u64 {
    (b << 7) & NOT_HFILE
}
