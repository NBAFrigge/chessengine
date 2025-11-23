use crate::chess::table::{Board, Color, Type};

pub fn evaluate_knights(b: &Board, color: Color) -> i32 {
    let mut score = 0;
    let knights = b.get_pieces(color, Type::Knight);
    let enemy_pawns = b.get_pieces(
        if color == Color::White {
            Color::Black
        } else {
            Color::White
        },
        Type::Pawn,
    );

    for knight in knights.iter_bits() {
        let sq = knight.lsb() as u8;

        // Outpost: cavaliere su casella forte non attaccabile da pedoni nemici
        if is_outpost(sq, enemy_pawns.get_value(), color) {
            score += 30;

            // Bonus extra se difeso da un pedone
            let own_pawns = b.get_pieces(color, Type::Pawn);
            if is_defended_by_pawn(sq, own_pawns.get_value(), color) {
                score += 15;
            }
        }
    }

    score
}

fn is_outpost(sq: u8, enemy_pawns: u64, color: Color) -> bool {
    let file = sq % 8;
    let rank = sq / 8;

    // Deve essere nella metà campo avversaria
    let in_enemy_territory = match color {
        Color::White => rank >= 4,
        Color::Black => rank <= 3,
    };

    if !in_enemy_territory {
        return false;
    }

    // Controlla se può essere attaccato da pedoni nemici
    let attack_files = if file == 0 {
        vec![1]
    } else if file == 7 {
        vec![6]
    } else {
        vec![file - 1, file + 1]
    };

    for f in attack_files {
        let file_mask = 0x0101010101010101u64 << f;
        let file_pawns = enemy_pawns & file_mask;

        // Controlla se ci sono pedoni nemici che possono attaccare
        match color {
            Color::White => {
                // Pedoni neri che possono scendere e attaccare
                for r in 0..=rank {
                    if (file_pawns & (1u64 << (r * 8 + f))) != 0 {
                        return false;
                    }
                }
            }
            Color::Black => {
                // Pedoni bianchi che possono salire e attaccare
                for r in rank..8 {
                    if (file_pawns & (1u64 << (r * 8 + f))) != 0 {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn is_defended_by_pawn(sq: u8, own_pawns: u64, color: Color) -> bool {
    let defender_sqs = match color {
        Color::White => {
            let mut sqs = Vec::new();
            if sq >= 8 && sq % 8 > 0 {
                sqs.push(sq - 9);
            }
            if sq >= 8 && sq % 8 < 7 {
                sqs.push(sq - 7);
            }
            sqs
        }
        Color::Black => {
            let mut sqs = Vec::new();
            if sq < 56 && sq % 8 > 0 {
                sqs.push(sq + 7);
            }
            if sq < 56 && sq % 8 < 7 {
                sqs.push(sq + 9);
            }
            sqs
        }
    };

    for def_sq in defender_sqs {
        if (own_pawns & (1u64 << def_sq)) != 0 {
            return true;
        }
    }

    false
}
