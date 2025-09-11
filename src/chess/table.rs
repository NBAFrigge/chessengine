use either::Either;
use crate::bitboard::bitboard::Bitboard;
use crate::chess::moves;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const FIRSTRANK: u64 = 0xff;
const LASTRANK: u64 = 0xff00000000000000;
const WHITESHORTCASTLING: u64 = 0x60;
const WHITELONGCASTLING: u64 = 0xe;
const BLACKSHORTCASTLING: u64 = 0x600000000000000;
const BLACKLONGCASTLING: u64 = 0x7000000000000000;

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
    Any,
}
#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Type {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
    Any,
}

pub struct Board {
    pawn: Bitboard,
    bishop: Bitboard,
    knight: Bitboard,
    rook: Bitboard,
    queen: Bitboard,
    king: Bitboard,
    white: Bitboard,
    black: Bitboard,

    is_white_turn: bool,

    whithe_castling: bool,
    black_castling: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pawn: Bitboard::new(0xff00000000ff00),
            bishop: Bitboard::new(0x2400000000000024),
            knight: Bitboard::new(0x4200000000000042),
            rook: Bitboard::new(0x8100000000000081),
            queen: Bitboard::new(0x800000000000008),
            king: Bitboard::new(0x1000000000000010),
            white: Bitboard::new(0xffff),
            black: Bitboard::new(0xffff000000000000),
            is_white_turn: true,
            black_castling: true,
            whithe_castling: true,
        }
    }

    pub fn get_pieces(&self, color: Color, piece_type: Type) -> Bitboard {
        match color {
            Color::White => self.get_piece_white(piece_type),
            Color::Black => self.get_piece_black(piece_type),
            Color::Any => self.get_piece_any(piece_type),
        }
    }

    fn get_piece_any(&self, piece_type: Type) -> Bitboard {
        match piece_type {
            Type::Any => self.white.or(self.black.clone()),
            Type::Pawn => self.pawn.clone(),
            Type::Bishop => self.bishop.clone(),
            Type::Knight => self.knight.clone(),
            Type::Rook => self.rook.clone(),
            Type::Queen => self.queen.clone(),
            Type::King => self.king.clone(),
        }
    }

    fn get_piece_white(&self, piece_type: Type) -> Bitboard {
        match piece_type {
            Type::Any => self.white.clone(),
            Type::Pawn => self.pawn.and(self.white.clone()),
            Type::Bishop => self.bishop.and(self.white.clone()),
            Type::Knight => self.knight.and(self.white.clone()),
            Type::Rook => self.rook.and(self.white.clone()),
            Type::Queen => self.queen.and(self.white.clone()),
            Type::King => self.king.and(self.white.clone()),
        }
    }

    fn get_piece_black(&self, piece_type: Type) -> Bitboard {
        match piece_type {
            Type::Any => self.black.clone(),
            Type::Pawn => self.pawn.and(self.black.clone()),
            Type::Bishop => self.bishop.and(self.black.clone()),
            Type::Knight => self.knight.and(self.black.clone()),
            Type::Rook => self.rook.and(self.black.clone()),
            Type::Queen => self.queen.and(self.black.clone()),
            Type::King => self.king.and(self.black.clone()),
        }
    }

    pub fn get_free_pos(&self) -> Bitboard {
        self.get_pieces(Color::White, Type::Any).or(self.get_pieces(Color::Black, Type::Any))
            .not()
    }

    pub fn get_occupied_post(&self) -> Bitboard {
        self.get_free_pos().not()
    }

    // region count pieces
    pub fn count_pieces(&self, color: Color, piece_type: Type) -> u64 {
        match color {
            Color::White => self.count_white(piece_type),
            Color::Black => self.count_black(piece_type),
            Color::Any => self.count_any(piece_type),
        }
    }

    fn count_any(&self, piece_type: Type) -> u64 {
        match piece_type {
            Type::Any => self.white.or(self.black.clone()).count_ones(),
            Type::Pawn => self.pawn.count_ones(),
            Type::Bishop => self.bishop.count_ones(),
            Type::Knight => self.knight.count_ones(),
            Type::Rook => self.rook.count_ones(),
            Type::Queen => self.queen.count_ones(),
            Type::King => self.king.count_ones(),
        }
    }

    fn count_white(&self, piece_type: Type) -> u64 {
        match piece_type {
                Type::Any => self.white.count_ones(),
                Type::Pawn => self.pawn.and(self.white.clone()).count_ones(),
                Type::Bishop => self.bishop.and(self.white.clone()).count_ones(),
                Type::Knight => self.knight.and(self.white.clone()).count_ones(),
                Type::Rook => self.rook.and(self.white.clone()).count_ones(),
                Type::Queen => self.queen.and(self.white.clone()).count_ones(),
                Type::King => self.king.and(self.white.clone()).count_ones(),
        }
    }

    fn count_black(&self, piece_type: Type) -> u64 {
        match piece_type {
            Type::Any => self.black.count_ones(),
            Type::Pawn => self.pawn.and(self.black.clone()).count_ones(),
            Type::Bishop => self.bishop.and(self.black.clone()).count_ones(),
            Type::Knight => self.knight.and(self.black.clone()).count_ones(),
            Type::Rook => self.rook.and(self.black.clone()).count_ones(),
            Type::Queen => self.queen.and(self.black.clone()).count_ones(),
            Type::King => self.king.and(self.black.clone()).count_ones(),
        }
    }
    // endregion


    // region moves
    pub fn get_all_moves_bitboard(&self, color: Color) -> Vec<Bitboard> {
        let empty = self.get_free_pos();
        let vec = Vec::new();
        for t in Type::iter() {

            //vec.append()
        }

        vec
    }

    pub fn get_move(&self, color : Color, piece_type: Type) -> Vec<Bitboard> {
        let moves = Vec::new();
        let piece_bitboard = self.get_pieces(color, piece_type);
        match piece_type {
            // TODO implement all moves
            Type::Any => moves,
            Type::Pawn => {self.get_pawn_move(piece_bitboard, color)}
            Type::King => {Vec::new()}
            Type::Bishop => {Vec::new()}
            Type::Knight => {Vec::new()}
            Type::Rook => {Vec::new()}
            Type::Queen => {Vec::new()}
        }
    }

    fn get_pawn_move(&self, bitboard: Bitboard, color: Color) -> Vec<Bitboard> {
        let mut moves = Vec::new();
        let empty = self.get_free_pos();
        match color {
            Color::White => {
                for p in bitboard.get_single_ones(){
                    let temp_bitboard = Bitboard::new(moves::pawn::white_moves(p.get_value(), empty.get_value()));
                    moves.push(temp_bitboard);
                }
            }
            Color::Black => {
                for p in bitboard.get_single_ones(){
                    let temp_bitboard = Bitboard::new(moves::pawn::black_moves(p.get_value(), empty.get_value()));
                    moves.push(temp_bitboard);
                }
            }
            Color::Any => {}
        }
        moves
    }


    // // endregion
    //
    // // region pawns private methods


    // fn black_pawns_single_push(&self) -> u64 { moves::pawn::south_one(self.black_pawns_able_to_push()) & self.get_free_pos() }
    //
    // fn black_pawns_double_push(&self) -> u64 {
    //     let able: u64 = 0x000000FF00000000;
    //     let singlepush = self.black_pawns_single_push();
    //     moves::pawn::south_one(singlepush) & able & self.get_free_pos()
    // }
    //
    // fn black_pawns_able_to_push(&self) -> u64 { moves::pawn::north_one(self.get_free_pos()) & self.get_black_pawn() }
    //
    // fn black_pawns_east_attack(&self) -> u64 { moves::pawn::southeast_one(self.get_black_pawn()) }
    //
    // fn black_pawns_west_attack(&self) -> u64 { moves::pawn::southwest_one(self.get_black_pawn()) }
    //
    // fn black_pawns_promotion_check(&self) -> bool {
    //     (!(self.get_black_pawn() ^ FIRSTRANK)) > 0
    // }

    // endregion

    // region castling
    // pub fn white_can_castle(&self) -> bool {}
    // endregion

    pub fn to_string(&self) -> String {
        let mut string = String::from("■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■");
        // add pieces to the board

        // white pieces
        let white_pawns =  self.get_pieces(Color::White, Type::Pawn).to_string();
        let white_rooks = self.get_pieces(Color::White, Type::Rook).to_string();
        let white_queens = self.get_pieces(Color::White, Type::Queen).to_string();
        let white_kings = self.get_pieces(Color::White, Type::King).to_string();
        let white_bishops = self.get_pieces(Color::White, Type::Bishop).to_string();
        let white_knights = self.get_pieces(Color::White, Type::Knight).to_string();


        // black pieces
        let black_pawns = self.get_pieces(Color::Black, Type::Pawn).to_string();
        let black_kings = self.get_pieces(Color::Black, Type::Rook).to_string();
        let black_queens = self.get_pieces(Color::Black, Type::Queen).to_string();
        let black_bishops = self.get_pieces(Color::Black, Type::King).to_string();
        let black_knights = self.get_pieces(Color::Black, Type::Bishop).to_string();
        let black_rooks = self.get_pieces(Color::Black, Type::Knight).to_string();

        for i in 0..64 {
            if white_pawns.chars().skip(i).take(1).collect::<Vec<_>>()[0] == '1' {
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