use std::io::empty;
use std::ops::Add;
use std::string;
use either::Either;
use crate::chess::moves;

pub struct Board {
    pawn: u64,
    bishop: u64,
    knight: u64,
    rook: u64,
    queen: u64,
    king: u64,
    white: u64,
    black: u64,

    is_white_turn : bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pawn:   0b0000000011111111000000000000000000000000000000001111111100000000,
            bishop: 0b0010010000000000000000000000000000000000000000000000000000100100,
            knight: 0b0100001000000000000000000000000000000000000000000000000001000010,
            rook:   0b1000000100000000000000000000000000000000000000000000000010000001,
            queen:  0b0000100000000000000000000000000000000000000000000000000000001000,
            king:   0b0001000000000000000000000000000000000000000000000000000000010000,
            white:  0b0000000000000000000000000000000000000000000000001111111111111111,
            black:  0b1111111111111111000000000000000000000000000000000000000000000000,
            is_white_turn : true,
        }
    }


    pub fn get_white(&self) -> u64 {
        self.white
    }

    pub fn get_black(&self) -> u64 {
        self.black
    }

    pub fn get_occupied_pos(&self) -> u64 {
        self.black | self.white
    }

    pub fn get_free_pos(&self) -> u64 {
        !self.get_occupied_pos()
    }

    // region get pieces per type
    pub fn get_king(&self) -> u64 {self.king}
    pub fn get_queen(&self) -> u64 {self.queen}
    pub fn get_rook(&self) -> u64 {self.rook}
    pub fn get_bishop(&self) -> u64 {self.bishop}
    pub fn get_knight(&self) -> u64 {self.knight}
    pub fn get_pawn(&self) -> u64 {self.pawn}

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
    
    // pub fn white_moves_bitboard_queen(&self)  -> u64 {}
    //
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
        let whiteKnight: u64 = self.get_white_knight();
        (moves::knight::north_northWest(whiteKnight) |
         moves::knight::north_northEast(whiteKnight) |
         moves::knight::north_westWest(whiteKnight) |
         moves::knight::north_eastEast(whiteKnight) |
         moves::knight::south_westWest(whiteKnight) |
         moves::knight::south_eastEast(whiteKnight) |
         moves::knight::south_southWest(whiteKnight) |
         moves::knight::south_southEast(whiteKnight))
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

    //
    // pub fn black_moves_bitboard_queen(&self)  -> u64 {}
    //
    // pub fn black_moves_bitboard_rook(&self)  -> u64 {}
    //
    // pub fn black_moves_bitboard_bishop(&self)  -> u64 {}
    //
    pub fn black_moves_bitboard_knight(&self)  -> u64 {
        let empty = self.get_free_pos();
        let blackKnight: u64 = self.get_black_knight();
        (moves::knight::north_northWest(blackKnight) |
            moves::knight::north_northEast(blackKnight) |
            moves::knight::north_westWest(blackKnight) |
            moves::knight::north_eastEast(blackKnight) |
            moves::knight::south_westWest(blackKnight) |
            moves::knight::south_eastEast(blackKnight) |
            moves::knight::south_southWest(blackKnight) |
            moves::knight::south_southEast(blackKnight))
            & empty
    }

    pub fn black_moves_bitboard_pawn(&self)  -> u64 {self.black_pawns_single_push() | self.black_pawns_double_push() }

    // endregion

    // region attacks
    pub fn white_pawns_attack(&self) -> u64 {self.white_pawns_west_attack() | self.white_pawns_east_attack()}

    pub fn whithe_pawn_double_attack(&self) -> u64 {self.white_pawns_west_attack() & self.white_pawns_east_attack()}

    pub fn whithe_pawn_single_attack(&self) -> u64 {self.white_pawns_west_attack() ^ self.white_pawns_east_attack()}

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


    pub fn bitboard_to_string(&self, b : u64) -> String {
        let string = format!("{:064b}", b);

        println!("{}", string);

        let s = string.chars()
            .collect::<Vec<char>>();

        let mut bitboard = string::String::from("");

        for chunk in s.chunks(8) {
            let mut s = chunk.iter().collect::<String>();
            s = s.chars().rev().collect::<String>();
            s.push_str("\n");
            bitboard.push_str(s.as_str());
        }

        bitboard.to_string()
    }

    pub fn to_string(&self) -> String {
        let mut string = String::from("■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■");
        // add pieces to the board

        // white pieces
        let white_pawns =  format!("{:064b}", self.pawn & self.white);
        let white_rooks =  format!("{:064b}", self.rook & self.white);
        let white_queens =  format!("{:064b}", self.queen & self.white);
        let white_kings =  format!("{:064b}", self.king & self.white);
        let white_bishops =  format!("{:064b}", self.bishop & self.white);
        let white_knights = format!("{:064b}", self.knight & self.white);


        // black pieces
        let black_pawns =  format!("{:064b}", self.pawn & self.black);
        let black_kings =  format!("{:064b}", self.king & self.black);
        let black_queens =  format!("{:064b}", self.queen & self.black);
        let black_bishops =  format!("{:064b}", self.bishop & self.black);
        let black_knights =  format!("{:064b}", self.knight & self.black);
        let black_rooks =  format!("{:064b}", self.rook & self.black);

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