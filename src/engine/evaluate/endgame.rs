use crate::chess::table::{Board, Color, Type};

pub fn evaluate_endgame_aggression(b: &Board, color: Color, phase: f32) -> i32 {
    if phase > 0.5 {
        return 0;
    }

    let mut score = 0;

    score += king_mobility(b, color);

    score += king_proximity_to_pawns(b, color);

    score += king_opposition(b, color);
    score += push_enemy_king_to_edge(b, color);

    score += advanced_pawns_bonus(b, color);

    score += control_key_squares(b, color);

    let endgame_multiplier = 1.0 + (0.5 - phase) * 2.0;

    (score as f32 * endgame_multiplier) as i32
}

pub fn king_mobility(b: &Board, color: Color) -> i32 {
    let king_sq = b.get_pieces(color, Type::King).lsb() as u8;
    let king_bb = 1u64 << king_sq;

    let king_moves = crate::chess::moves_gen::king::moves(king_bb);
    let own_pieces = b.get_pieces(color, Type::Any).get_value();

    let free_squares = (king_moves & !own_pieces).count_ones() as i32;

    free_squares * 3
}

pub fn king_proximity_to_pawns(b: &Board, color: Color) -> i32 {
    let king_sq = b.get_pieces(color, Type::King).lsb() as i32;
    let king_file = king_sq % 8;
    let king_rank = king_sq / 8;

    let own_pawns = b.get_pieces(color, Type::Pawn);

    let target_pawns = if own_pawns.count_ones() > 0 {
        own_pawns
    } else {
        let enemy_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        b.get_pieces(enemy_color, Type::Pawn)
    };

    if target_pawns.count_ones() == 0 {
        return 0;
    }

    let mut min_dist = 100;
    for pawn in target_pawns.iter_bits() {
        let p_sq = pawn.lsb() as i32;
        let p_file = p_sq % 8;
        let p_rank = p_sq / 8;

        let dist = (king_file - p_file).abs() + (king_rank - p_rank).abs(); // Distanza Manhattan
        if dist < min_dist {
            min_dist = dist;
        }
    }

    (14 - min_dist) * 10
}

pub fn king_opposition(b: &Board, color: Color) -> i32 {
    let enemy_color = match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let own_king_sq = b.get_pieces(color, Type::King).lsb() as i32;
    let enemy_king_sq = b.get_pieces(enemy_color, Type::King).lsb() as i32;

    let own_file = own_king_sq % 8;
    let own_rank = own_king_sq / 8;
    let enemy_file = enemy_king_sq % 8;
    let enemy_rank = enemy_king_sq / 8;

    let distance = (own_file - enemy_file).abs() + (own_rank - enemy_rank).abs();

    if distance <= 7 {
        10 * (14 - distance)
    } else {
        0
    }
}

pub fn push_enemy_king_to_edge(b: &Board, color: Color) -> i32 {
    let enemy_color = match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let enemy_king_sq = b.get_pieces(enemy_color, Type::King).lsb() as i32;
    let file = enemy_king_sq % 8;
    let rank = enemy_king_sq / 8;

    let center_file = if file < 4 { 3 } else { 4 };
    let center_rank = if rank < 4 { 3 } else { 4 };

    let file_dist = (file - center_file).abs();
    let rank_dist = (rank - center_rank).abs();
    let distance_from_center = file_dist.max(rank_dist);

    distance_from_center * 10
}

pub fn advanced_pawns_bonus(b: &Board, color: Color) -> i32 {
    let pawns = b.get_pieces(color, Type::Pawn);
    let mut score = 0;

    for pawn in pawns.iter_bits() {
        let sq = pawn.lsb() as i32;
        let rank = sq / 8;

        let advancement = match color {
            Color::White => rank,
            Color::Black => 7 - rank,
        };

        let mut bonus = advancement * advancement;

        if advancement == 6 {
            bonus += 50;
        }

        if is_passed_pawn(b, color, sq as u8) {
            bonus *= 2;

            if is_supported_by_rook(b, color, sq as u8) {
                bonus += 20;
            }
        }

        score += bonus * 5;
    }

    score
}

fn is_supported_by_rook(b: &Board, color: Color, pawn_sq: u8) -> bool {
    let file = pawn_sq % 8;
    let rooks = b.get_pieces(color, Type::Rook);
    let file_mask = 0x0101010101010101u64 << file;

    (rooks.get_value() & file_mask) != 0
}

fn is_passed_pawn(b: &Board, color: Color, square: u8) -> bool {
    let enemy_color = match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let enemy_pawns = b.get_pieces(enemy_color, Type::Pawn).get_value();
    let file = square % 8;
    let rank = square / 8;

    let files_to_check = if file == 0 {
        vec![0, 1]
    } else if file == 7 {
        vec![6, 7]
    } else {
        vec![file - 1, file, file + 1]
    };

    match color {
        Color::White => {
            for check_rank in (rank + 1)..8 {
                for &check_file in &files_to_check {
                    let check_sq = check_rank * 8 + check_file;
                    if (enemy_pawns & (1u64 << check_sq)) != 0 {
                        return false;
                    }
                }
            }
        }
        Color::Black => {
            for check_rank in (0..rank).rev() {
                for &check_file in &files_to_check {
                    let check_sq = check_rank * 8 + check_file;
                    if (enemy_pawns & (1u64 << check_sq)) != 0 {
                        return false;
                    }
                }
            }
        }
    }

    true
}

pub fn control_key_squares(b: &Board, color: Color) -> i32 {
    let key_squares = [27, 28, 35, 36];
    let mut score = 0;

    for &sq in &key_squares {
        if is_square_controlled(b, color, sq) {
            score += 10;
        }
    }

    score
}

fn is_square_controlled(b: &Board, color: Color, square: u8) -> bool {
    let sq_bb = 1u64 << square;
    let occupied = b.get_occupied_pos().get_value();

    let pawns = b.get_pieces(color, Type::Pawn).get_value();
    match color {
        Color::White => {
            if square >= 8 {
                let mut attacks = 0u64;
                if square % 8 != 0 {
                    attacks |= 1u64 << (square - 9);
                }
                if square % 8 != 7 {
                    attacks |= 1u64 << (square - 7);
                }
                if attacks & pawns != 0 {
                    return true;
                }
            }
        }
        Color::Black => {
            if square < 56 {
                let mut attacks = 0u64;
                if square % 8 != 0 {
                    attacks |= 1u64 << (square + 7);
                }
                if square % 8 != 7 {
                    attacks |= 1u64 << (square + 9);
                }
                if attacks & pawns != 0 {
                    return true;
                }
            }
        }
    }

    let knights = b.get_pieces(color, Type::Knight).get_value();
    let knight_attacks = crate::chess::moves_gen::knight::moves(sq_bb);
    if knight_attacks & knights != 0 {
        return true;
    }

    let bishops = b.get_pieces(color, Type::Bishop).get_value();
    let queens = b.get_pieces(color, Type::Queen).get_value();
    let bishop_attacks = crate::chess::moves_gen::bishop::moves(sq_bb, occupied);
    if bishop_attacks & (bishops | queens) != 0 {
        return true;
    }

    let rooks = b.get_pieces(color, Type::Rook).get_value();
    let rook_attacks = crate::chess::moves_gen::rook::moves(sq_bb, occupied);
    if rook_attacks & (rooks | queens) != 0 {
        return true;
    }

    let king = b.get_pieces(color, Type::King).get_value();
    let king_attacks = crate::chess::moves_gen::king::moves(sq_bb);
    if king_attacks & king != 0 {
        return true;
    }

    false
}
