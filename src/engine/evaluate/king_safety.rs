use crate::{
    bitboard::bitboard::Bitboard,
    chess::table::{Board, Color, Type},
};

pub fn evaluate_king_safety(b: &Board, color: Color, phase: f32) -> i32 {
    let mut score = 0;
    let king_sq = b.get_pieces(color, Type::King).lsb() as u8;

    if phase > 0.5 && b.has_castled(color) {
        // ← Aggiungi has_castled!
        let shelter = evaluate_pawn_shelter(b, king_sq, color);
        score += (shelter as f32 * phase * 3.0) as i32;
    }

    let king_file = king_sq % 8;
    for file in (king_file.saturating_sub(1))..=(king_file + 1).min(7) {
        if is_open_file(b, file) {
            score -= 25;
        }
    }

    if b.has_castled(color) {
        score += 40;
    }

    score
}

fn evaluate_pawn_shelter(b: &Board, king_sq: u8, color: Color) -> i32 {
    let mut shelter = 0;
    let pawns = b.get_pieces(color, Type::Pawn);
    let king_file = king_sq % 8;
    let king_rank = king_sq / 8;

    for file in (king_file.saturating_sub(1))..=(king_file + 1).min(7) {
        if has_pawn_shield(pawns, file, king_rank, color) {
            shelter += 10;
        }
    }

    shelter
}

fn has_pawn_shield(pawns: Bitboard, file: u8, king_rank: u8, color: Color) -> bool {
    let check_ranks = match color {
        Color::White => {
            if king_rank >= 6 {
                return false;
            }
            (king_rank + 1)..=(king_rank + 2).min(7)
        }
        Color::Black => {
            if king_rank <= 1 {
                return false;
            }
            (king_rank - 2).max(0)..=king_rank - 1
        }
    };

    for rank in check_ranks {
        let sq = rank * 8 + file;
        if (pawns.0 & (1u64 << sq)) != 0 {
            return true;
        }
    }

    false
}

fn is_open_file(b: &Board, file: u8) -> bool {
    let file_mask = 0x0101010101010101u64 << file;
    let white_pawns = b.get_pieces(Color::White, Type::Pawn);
    let black_pawns = b.get_pieces(Color::Black, Type::Pawn);

    (white_pawns.0 & file_mask) == 0 && (black_pawns.0 & file_mask) == 0
}
