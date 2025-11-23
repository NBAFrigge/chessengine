use crate::chess::table::{Board, Color, Type};

pub fn evaluate_rooks(b: &Board, color: Color) -> i32 {
    let mut score = 0;
    let rooks = b.get_pieces(color, Type::Rook);
    let own_pawns = b.get_pieces(color, Type::Pawn);
    let enemy_pawns = b.get_pieces(
        if color == Color::White {
            Color::Black
        } else {
            Color::White
        },
        Type::Pawn,
    );

    for rook in rooks.iter_bits() {
        let sq = rook.lsb() as u8;
        let file = sq % 8;
        let rank = sq / 8;
        let file_mask = 0x0101010101010101u64 << file;

        // Colonna aperta (nessun pedone)
        if (own_pawns.get_value() & file_mask) == 0 && (enemy_pawns.get_value() & file_mask) == 0 {
            score += 40;
        }
        // Colonna semi-aperta (solo pedoni nemici)
        else if (own_pawns.get_value() & file_mask) == 0 {
            score += 25;
        }

        // Torre sulla settima fila
        let seventh_rank = match color {
            Color::White => rank == 6,
            Color::Black => rank == 1,
        };
        if seventh_rank {
            score += 30;
        }

        // Torri connesse (sulla stessa fila/colonna senza ostacoli)
        if are_rooks_connected(b, color, sq) {
            score += 15;
        }
    }

    score
}

fn are_rooks_connected(b: &Board, color: Color, rook_sq: u8) -> bool {
    let rooks = b.get_pieces(color, Type::Rook);
    let occupied = b.get_occupied_pos().get_value();
    let rank = rook_sq / 8;
    let file = rook_sq % 8;

    // Controlla sulla stessa fila
    let rank_mask = 0xFFu64 << (rank * 8);
    let file_mask = 0x0101010101010101u64 << file;

    let rooks_on_rank = (rooks.get_value() & rank_mask).count_ones();
    let rooks_on_file = (rooks.get_value() & file_mask).count_ones();

    // Se ci sono 2 torri sulla stessa fila/colonna
    if rooks_on_rank >= 2 || rooks_on_file >= 2 {
        // Verifica che non ci siano pezzi tra loro
        // (implementazione semplificata)
        return true;
    }

    false
}
