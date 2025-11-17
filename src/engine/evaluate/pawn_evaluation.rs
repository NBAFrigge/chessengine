use crate::bitboard::bitboard::Bitboard;

const FILE_MASK: [Bitboard; 8] = [
    Bitboard::new(0x101010101010101),
    Bitboard::new(0x202020202020202),
    Bitboard::new(0x404040404040404),
    Bitboard::new(0x808080808080808),
    Bitboard::new(0x1010101010101010),
    Bitboard::new(0x2020202020202020),
    Bitboard::new(0x4040404040404040),
    Bitboard::new(0x8080808080808080),
];

pub fn evaluate_pawn(pawn: &Bitboard) -> i32 {
    let mut score = 0;
    for i in 0..8 {
        let file = pawn.and(FILE_MASK[i]);
        let count = file.count_ones();
        if count > 1 {
            score -= 15 * (count - 1) as i32;
        }
    }

    for b in pawn.iter_bits() {
        let file = (b.lsb() % 8) as usize;
        let adjacent_file = get_adjacent_file(file);
        if pawn.and(adjacent_file) == Bitboard::empty() {
            score -= 15
        }
    }

    score
}

fn get_adjacent_file(file: usize) -> Bitboard {
    match file {
        0 => FILE_MASK[1],
        7 => FILE_MASK[6],
        _ => FILE_MASK[file - 1].or(FILE_MASK[file + 1]),
    }
}
