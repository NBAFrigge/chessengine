use crate::bitboard::bitboard::Bitboard;

pub const RANKS: [u64; 8] = [
    255,                  // 8th rank
    65280,                // 7th rank
    16711680,             // 6th rank
    4278190080,           // 5th rank
    1095216660480,        // 4th rank
    280375465082880,      // 3rd rank
    71776119061217280,    // 2nd rank
    18374686479671623680, // 1st rank
];

pub const FILES: [u64; 8] = [
    72340172838076673,   // a file
    144680345676153346,  // b file
    289360691352306692,  // c file
    578721382704613384,  // d file
    1157442765409226768, // e file
    2314885530818453536, // f file
    4629771061636907072, // g file
    9259542123273814144, // h file
];

pub const DIAG: [u64; 15] = [
    0x80,
    0x8040,
    0x804020,
    0x80402010,
    0x8040201008,
    0x804020100804,
    0x80402010080402,
    0x8040201008040201,
    0x4020100804020100,
    0x2010080402010000,
    0x1008040201000000,
    0x804020100000000,
    0x402010000000000,
    0x201000000000000,
    0x100000000000000,
];

// anti-diagonal masks
pub const ANTI_DIAG: [u64; 15] = [
    0x1,
    0x102,
    0x10204,
    0x1020408,
    0x102040810,
    0x10204081020,
    0x1020408102040,
    0x102040810204080,
    0x204081020408000,
    0x408102040800000,
    0x810204080000000,
    0x1020408000000000,
    0x2040800000000000,
    0x4080000000000000,
    0x8000000000000000,
];

pub fn hyp_quint(sq: u64, occ: u64, mask: u64) -> u64 {
    let mut forward = occ & mask;
    let mut reverse = forward.reverse_bits();

    forward = forward.wrapping_sub(Bitboard::new_from_index(sq.clone()).get_value());
    reverse = reverse.wrapping_sub(Bitboard::new_from_index(sq.clone()).get_value().reverse_bits());
    forward ^= reverse.reverse_bits();
    forward &= mask;

    forward
}