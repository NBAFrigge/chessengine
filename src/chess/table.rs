use either::Either;
use crate::bitboard::bitboard::bitboard;
use crate::chess::moves;

pub struct Board {
    pawn: bitboard,
    bishop: bitboard,
    knight: bitboard,
    rook: bitboard,
    queen: bitboard,
    king: bitboard,
    white: bitboard,
    black: bitboard,

    is_white_turn : bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pawn:   bitboard::new(0xff00000000ff00),
            bishop: bitboard::new(2400000000000024),
            knight: bitboard::new(0x4200000000000042),
            rook:   bitboard::new(0x8100000000000081),
            queen:  bitboard::new(0x800000000000008),
            king:   bitboard::new(0x1000000000000010),
            white:  bitboard::new(0xffff),
            black:  bitboard::new(0xffff000000000000),
            is_white_turn : true,
        }
    }


    pub fn get_white(&self) -> u64 {self.white.get_value()}

    pub fn get_black(&self) -> u64 {self.black.get_value()}

    pub fn get_occupied_pos(&self) -> u64 {self.black.get_value() | self.white.get_value()}

    pub fn get_free_pos(&self) -> u64 {
        !self.get_occupied_pos()
    }

    // region get pieces per type
    pub fn get_king(&self) -> u64 {self.king.get_value()}
    pub fn get_queen(&self) -> u64 {self.queen.get_value()}
    pub fn get_rook(&self) -> u64 {self.rook.get_value()}
    pub fn get_bishop(&self) -> u64 {self.bishop.get_value()}
    pub fn get_knight(&self) -> u64 {self.knight.get_value()}
    pub fn get_pawn(&self) -> u64 {self.pawn.get_value()}

    // endregion

    // region get pieces per type and color methods
    pub fn get_black_king(&self) -> u64 {self.get_black() & self.get_king()}
    pub fn get_black_queen(&self) -> u64 {self.get_black() & self.get_queen()}
    pub fn get_black_rook(&self) -> u64 {self.get_black() & self.get_rook()}
    pub fn get_black_bishop(&self) -> u64 {self.get_black() & self.get_bishop()}
    pub fn get_black_knight(&self) -> u64 {self.get_black() & self.get_knight()}
    pub fn get_black_pawn(&self) -> u64 {self.get_black() & self.get_pawn()}


    pub fn get_white_king(&self) -> u64 {self.get_white() & self.get_king()}
    pub fn get_white_queen(&self) -> u64 {self.get_white() & self.get_queen()}
    pub fn get_white_rook(&self) -> u64 {self.get_white() & self.get_rook()}
    pub fn get_white_bishop(&self) -> u64 {self.get_white() & self.get_bishop()}
    pub fn get_white_knight(&self) -> u64 {self.get_white() & self.get_knight()}
    pub fn get_white_pawn(&self) -> u64 {self.get_white() & self.get_pawn()}
    // endregion

    // region moves
    pub fn white_moves_bitboard_king(&self)  -> u64 {
        let empty = self.get_free_pos();
        let king = self.get_white_king();
        (
            moves::king::north_one(king) |
            moves::king::south_one(king) |
            moves::king::east_one(king) |
            moves::king::northeast_one(king) |
            moves::king::southeast_one(king) |
            moves::king::west_one(king) |
            moves::king::southwest_one(king) |
            moves::king::northwest_one(king)
        ) & empty
    }
    
    pub fn white_moves_bitboard_queen(&self)  -> u64 {
        let queenmask = moves::queen::generatemask(44);
        let blockers = self.get_occupied_pos() & queenmask;
        queenmask ^ blockers

    }

    pub fn white_moves_bitboard_rook(&self)  -> u64 {
        let rookmask = moves::rook::generateMask(45);
        let blockers = self.get_occupied_pos() & rookmask;
        rookmask ^  blockers
    }

    pub fn white_moves_bitboard_bishop(&self)  -> u64 {
        let bishopmask = moves::bishop::generateMask(44);
        let blockers = self.get_occupied_pos() & bishopmask;
        bishopmask ^  blockers
    }

    pub fn white_moves_bitboard_knight(&self)  -> u64 {
        let empty = self.get_free_pos();
        let white_knight: u64 = self.get_white_knight();
        (moves::knight::north_northWest(white_knight) |
         moves::knight::north_northEast(white_knight) |
         moves::knight::north_westWest(white_knight) |
         moves::knight::north_eastEast(white_knight) |
         moves::knight::south_westWest(white_knight) |
         moves::knight::south_eastEast(white_knight) |
         moves::knight::south_southWest(white_knight) |
         moves::knight::south_southEast(white_knight))
        & empty
    }
    //
    pub fn white_moves_bitboard_pawn(&self)  -> u64 {self.white_pawns_single_push() | self.white_pawns_double_push()}

    pub fn black_moves_bitboard_king(&self)  -> u64 {
        let empty = self.get_free_pos();
        let king = self.get_black_king();
        (
            moves::king::north_one(king) |
                moves::king::south_one(king) |
                moves::king::east_one(king) |
                moves::king::northeast_one(king) |
                moves::king::southeast_one(king) |
                moves::king::west_one(king) |
                moves::king::southwest_one(king) |
                moves::king::northwest_one(king)
        ) & empty
    }

    //pub fn black_moves_bitboard_queen(&self)  -> u64 {}
    //
    // pub fn black_moves_bitboard_rook(&self)  -> u64 {}
    //
    // pub fn black_moves_bitboard_bishop(&self)  -> u64 {}
    //
    pub fn black_moves_bitboard_knight(&self)  -> u64 {
        let empty = self.get_free_pos();
        let black_knight: u64 = self.get_black_knight();
        (moves::knight::north_northWest(black_knight) |
            moves::knight::north_northEast(black_knight) |
            moves::knight::north_westWest(black_knight) |
            moves::knight::north_eastEast(black_knight) |
            moves::knight::south_westWest(black_knight) |
            moves::knight::south_eastEast(black_knight) |
            moves::knight::south_southWest(black_knight) |
            moves::knight::south_southEast(black_knight))
            & empty
    }

    pub fn black_moves_bitboard_pawn(&self)  -> u64 {self.black_pawns_single_push() | self.black_pawns_double_push() }

    // endregion

    // region attacks
    pub fn white_pawns_attack(&self) -> u64 {self.white_pawns_west_attack() | self.white_pawns_east_attack()}

    pub fn white_pawn_double_attack(&self) -> u64 {self.white_pawns_west_attack() & self.white_pawns_east_attack()}

    pub fn white_pawn_single_attack(&self) -> u64 {self.white_pawns_west_attack() ^ self.white_pawns_east_attack()}

    pub fn black_pawns_attack(&self) -> u64 {self.black_pawns_east_attack() | self.black_pawns_west_attack()}

    pub fn black_pawn_double_attack(&self) -> u64 {self.black_pawns_east_attack() & self.black_pawns_west_attack()}

    pub fn black_pawn_single_attack(&self) -> u64 {self.black_pawns_east_attack() ^ self.black_pawns_west_attack()}

    // endregion

    // region captures
    fn white_able_capture(&self) -> u64 {
        self.get_white_pawn() & self.black_pawns_attack()
    }

    fn white_able_capture_west(&self) -> u64 {
        self.get_white_pawn() & self.black_pawns_west_attack()
    }

    fn white_able_capture_east(&self) -> u64 {
        self.get_white_pawn() & self.black_pawns_east_attack()
    }

    fn black_able_capture(&self) -> u64 {
        self.get_black_pawn() & self.white_pawns_attack()
    }

    fn black_able_capture_west(&self) -> u64 {
        self.get_black_pawn() & self.black_pawns_west_attack()
    }

    fn black_able_capture_east(&self) -> u64 {
        self.get_black_pawn() & self.black_pawns_east_attack()
    }
    // endregion

    // region pawns private methods
    fn white_pawns_single_push(&self) -> u64 { moves::pawn::north_one(self.white_pawns_able_to_push()) & self.get_free_pos() }

    fn white_pawns_double_push(&self) -> u64 {
        let able:u64 = 0x00000000FF000000;
        let singlepush = self.white_pawns_single_push();
        moves::pawn::north_one(singlepush) & able & self.get_free_pos()
    }

    fn white_pawns_able_to_push(&self) -> u64 { moves::pawn::south_one(self.get_free_pos()) & self.get_white_pawn()}

    fn white_pawns_east_attack(&self) -> u64 { moves::pawn::northeast_one(self.get_white_pawn())}

    fn white_pawns_west_attack(&self) -> u64 { moves::pawn::northwest_one(self.get_white_pawn())}


    fn black_pawns_single_push(&self) -> u64 { moves::pawn::south_one(self.black_pawns_able_to_push()) & self.get_free_pos()}

    fn black_pawns_double_push(&self) -> u64 {
        let able:u64 = 0x000000FF00000000;
        let singlepush = self.black_pawns_single_push();
        moves::pawn::south_one(singlepush) & able & self.get_free_pos()
    }

    fn black_pawns_able_to_push(&self) -> u64 { moves::pawn::north_one(self.get_free_pos()) & self.get_black_pawn()}

    fn black_pawns_east_attack(&self) -> u64 { moves::pawn::southeast_one(self.get_black_pawn())}

    fn black_pawns_west_attack(&self) -> u64 { moves::pawn::southwest_one(self.get_black_pawn())}

    // endregion

    pub fn to_string(&self) -> String {
        let mut string = String::from("■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■");
        // add pieces to the board

        // white pieces
        let white_pawns =  format!("{:064b}", self.get_white_pawn());
        let white_rooks =  format!("{:064b}", self.get_white_rook());
        let white_queens =  format!("{:064b}", self.get_white_queen());
        let white_kings =  format!("{:064b}", self.get_white_king());
        let white_bishops =  format!("{:064b}", self.get_white_bishop());
        let white_knights = format!("{:064b}", self.get_white_knight());


        // black pieces
        let black_pawns =  format!("{:064b}", self.get_black_pawn());
        let black_kings =  format!("{:064b}", self.get_black_king());
        let black_queens =  format!("{:064b}", self.get_black_queen());
        let black_bishops =  format!("{:064b}", self.get_black_bishop());
        let black_knights =  format!("{:064b}", self.get_black_knight());
        let black_rooks =  format!("{:064b}", self.get_black_rook());

        for i in 0..64 {
            if  white_pawns.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♙')
            }

            if white_rooks.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♖')
            }

            if white_kings.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♔')
            }

            if white_bishops.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♗')
            }

            if white_queens.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♕')
            }

            if white_knights.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♘')
            }

            if black_pawns.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♟')
            }

            if black_kings.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♚')
            }

            if black_queens.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♛')
            }

            if black_bishops.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♝')
            }

            if black_knights.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♞')
            }

            if black_rooks.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
                string = change_char_in_string(string, i, '♜')
            }
        }


        string = string.chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if i != 0 && i % 1 == 0 {
                    Some(' ')
                } else {
                    None
                }
                    .into_iter()
                    .chain(std::iter::once(c))
            })
            .collect::<String>();

        let iter_with_newlines = string.chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if i % 16 == 0 {
                    Either::Left(['\n', c].into_iter())
                } else {
                    Either::Right(std::iter::once(c))
                }
            })
            .skip(1);

        let string: String = iter_with_newlines.collect();

        string
    }

}


fn change_char_in_string(s: String, index: usize, new_char: char) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    if index < chars.len() {
        chars[index] = new_char;
    }
    String::from_iter(chars)
}