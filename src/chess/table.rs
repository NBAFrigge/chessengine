use std::cmp::PartialEq;
use either::Either;
use crate::bitboard::bitboard::Bitboard;
use crate::chess::moves_gen;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::chess::moves_gen::moves_struct::Moves;
use crate::chess::moves_gen::moves_struct::MoveType;

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
#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum Type {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
    Any,
}

pub enum Side{
    Long,
    Short,
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
    white_rook_long_side: bool,
    white_rook_short_side: bool,

    black_rook_long_side: bool,
    black_rook_short_side: bool,

    white_king: bool,
    black_king: bool,
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

            white_rook_long_side: true,
            white_rook_short_side: true,
            black_rook_long_side: true,
            black_rook_short_side: true,

            white_king: true,
            black_king: true,
        }
    }

    // get method
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

    pub fn get_occupied_pos(&self) -> Bitboard {
        self.get_free_pos().not()
    }

    // count pieces
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

    // legal moves clearing
    #[inline]
    pub fn get_legal_moves(&self, pseudo: Vec<Moves>, color: Color) -> Vec<Moves> {
        let mut legal_moves = Vec::new();
        for p in pseudo {
            legal_moves.push(Moves::new(p.old_pos, p.new_pos.and(self.get_pieces(color, Type::Any).not())));
        }
        legal_moves
    }

    // pseudo-legal moves_gen gen
    #[inline]
    pub fn get_all_moves_bitboard(&self, color: Color) -> Vec<Moves> {
        let mut vec = Vec::new();
        for t in Type::iter() {
            if t == Type::Any {
                continue;
            }
            vec.append(self.get_move(color, t).as_mut())
        }
        
        vec.append(self.castle(color).as_mut());

        vec
    }

    pub fn get_move(&self, color : Color, piece_type: Type) -> Vec<Moves> {
        let piece_bitboard = self.get_pieces(color, piece_type);
        match piece_type {
            Type::Any => panic!("get_move called on type Any"),
            Type::Pawn => {self.get_pawn_move(piece_bitboard, color)}
            Type::King => {self.get_king_move(piece_bitboard)}
            Type::Bishop => {self.get_bishop_move(piece_bitboard, self.get_occupied_pos())}
            Type::Knight => {self.get_knight_move(piece_bitboard)}
            Type::Rook => {self.get_rook_move(piece_bitboard, self.get_occupied_pos())}
            Type::Queen => {self.get_queen_move(piece_bitboard, self.get_occupied_pos())}
        }
    }

    fn get_pawn_move(&self, bitboard: Bitboard, color: Color) -> Vec<Moves> {
        let mut m = Vec::new();
        let empty = self.get_free_pos();
        match color {
            Color::White => {
                for p in bitboard.get_single_ones(){
                    let temp_bitboard = Bitboard::new(moves_gen::pawn::white_moves(p.get_value(), empty.get_value()));
                    let temp_move = Moves::new(p.clone(), temp_bitboard);
                    m.push(temp_move);
                }
            }
            Color::Black => {
                for p in bitboard.get_single_ones(){
                    let temp_bitboard = Bitboard::new(moves_gen::pawn::black_moves(p.get_value(), empty.get_value()));
                    let temp_move = Moves::new(p.clone(), temp_bitboard);
                    m.push(temp_move);
                }
            }
            Color::Any => {panic!("get_move called on type Any")}
        }
        m
    }

    fn get_knight_move(&self, bitboard: Bitboard) -> Vec<Moves> {
        let mut m = Vec::new();
        for p in bitboard.get_single_ones(){
            let temp_bitboard = Bitboard::new(moves_gen::knight::moves(p.get_value()));
            let temp_move = Moves::new(p.clone(), temp_bitboard);
            m.push(temp_move);
        }
        m
    }

    fn get_king_move(&self, bitboard: Bitboard) -> Vec<Moves> {
        let mut m = Vec::new();
        for p in bitboard.get_single_ones(){
            let temp_bitboard = Bitboard::new(moves_gen::king::moves(p.get_value()));
            let temp_move = Moves::new(p.clone(), temp_bitboard);
            m.push(temp_move);
        }
        m
    }

    fn get_rook_move(&self, bitboard: Bitboard, occupied: Bitboard) -> Vec<Moves> {
        let mut m = Vec::new();
        for p in bitboard.get_single_ones(){
            let temp_bitboard = Bitboard::new(moves_gen::rook::moves(p.get_value(), occupied.get_value()));
            let temp_move = Moves::new(p.clone(), temp_bitboard);
            m.push(temp_move);
        }
        m
    }

    fn get_bishop_move(&self, bitboard: Bitboard, occupied: Bitboard) -> Vec<Moves> {
        let mut m = Vec::new();
        for p in bitboard.get_single_ones(){
            let temp_bitboard = Bitboard::new(moves_gen::bishop::moves(p.get_value(), occupied.get_value()));
            let temp_move = Moves::new(p.clone(), temp_bitboard);
            m.push(temp_move);
        }
        m
    }

    fn get_queen_move(&self, bitboard: Bitboard, occupied: Bitboard) -> Vec<Moves> {
        let mut m = Vec::new();
        for p in bitboard.get_single_ones(){
            let temp_bitboard = Bitboard::new(moves_gen::queen::moves(p.get_value(), occupied.get_value()));
            let temp_move = Moves::new(p.clone(), temp_bitboard);
            m.push(temp_move);
        }
        m
    }

    // castling
    fn castle(&self, color: Color,) -> Vec<Moves> {
        let mut m = Vec::new();
        if self.can_castle(color, Side::Long) {
            m.push(Moves::castling(MoveType::LongCastle));
        }
        if self.can_castle(color, Side::Short) {
            m.push(Moves::castling(MoveType::ShortCastle));
        }
        
        m
    }
    fn can_castle(&self, color: Color, side: Side) -> bool {
        match side {
            Side::Long => {self.can_castle_long_side(color)}
            Side::Short => {self.can_castle_short_side(color)}
        }
    }

    fn can_castle_long_side(&self, color: Color) -> bool {
        match color {
            Color::White => {
                if self.white_rook_long_side && self.white_king && ((self.get_piece_any(Type::Any).get_value() & WHITELONGCASTLING) == 0) {
                    return true;
                }
                false
            }
            Color::Black => {
                if self.black_rook_long_side && self.black_king && ((self.get_piece_any(Type::Any).get_value() & BLACKLONGCASTLING) == 0) {
                    return true;
                }
                false
            }
            Color::Any => {panic!("rook side color can't be any")}
        }
    }
    fn can_castle_short_side(&self, color: Color) -> bool {
        match color {
            Color::White => {
                if self.white_rook_short_side && self.white_king && ((self.get_piece_any(Type::Any).get_value() & WHITESHORTCASTLING) == 0) {
                    return true;
                }
                false
            }
            Color::Black => {
                if self.black_rook_short_side && self.black_king && ((self.get_piece_any(Type::Any).get_value() & BLACKSHORTCASTLING) == 0) {
                    return true;
                }
                false
            }
            Color::Any => {panic!("rook side color can't be any")}
        }
    }

    fn check(&self, color: Color, moves: Vec<Bitboard>) -> bool {
        let king = self.get_pieces(color, Type::King);
        for m in moves {
            if m.and(king.clone()).get_value() > 0 {
                return true;
            }
        }
        false
    }

    //TODO checkmate
    // checkmate
    

    // promotion
    pub fn check_promotion(&self, color: Color) -> Bitboard {
        let pawn = self.get_pieces(color, Type::Pawn);
        match color {
            Color::White => {Bitboard::new(pawn.get_value() & LASTRANK)}
            Color::Black => {Bitboard::new(pawn.get_value() & FIRSTRANK)}
            Color::Any => {panic!("color can't be any")}
        }
    }

    // to string function
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