use crate::bitboard::bitboard::Bitboard;
use crate::chess::moves_gen;
use crate::chess::moves_gen::moves_struct::MoveType;
use crate::chess::moves_gen::moves_struct::Moves;
use either::Either;
use std::cmp::PartialEq;
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

pub enum Side {
    Long,
    Short,
}

#[derive(Clone, Copy)]
pub struct Board {
    pawn: Bitboard,
    bishop: Bitboard,
    knight: Bitboard,
    rook: Bitboard,
    queen: Bitboard,
    king: Bitboard,
    white: Bitboard,
    black: Bitboard,

    pub is_white_turn: bool,
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
        self.get_pieces(Color::White, Type::Any)
            .or(self.get_pieces(Color::Black, Type::Any))
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
    pub fn get_legal_moves(&self, color: Color) -> Vec<Moves> {
        let mut legal_moves = Vec::new();
        let pseudo = self.get_all_moves_bitboard(color);

        for p in pseudo {
            if p.move_type == MoveType::Simple
                && p.new_pos.and(self.get_pieces(color, Type::Any)).get_value() > 0
            {
                continue;
            }
            let mut test_board = self.clone();
            test_board.perform_move(p.old_pos, p.new_pos);

            if !test_board.is_king_in_check(color) {
                legal_moves.push(p);
            }
        }

        legal_moves
    }

    // pseudo-legal moves_gen gen
    fn get_all_moves_bitboard(&self, color: Color) -> Vec<Moves> {
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

    pub fn get_move(&self, color: Color, piece_type: Type) -> Vec<Moves> {
        let piece_bitboard = self.get_pieces(color, piece_type);
        match piece_type {
            Type::Any => panic!("get_move called on type Any"),
            Type::Pawn => self.get_pawn_move(piece_bitboard, color),
            Type::King => self.get_king_move(piece_bitboard),
            Type::Bishop => self.get_bishop_move(piece_bitboard, self.get_occupied_pos()),
            Type::Knight => self.get_knight_move(piece_bitboard),
            Type::Rook => self.get_rook_move(piece_bitboard, self.get_occupied_pos()),
            Type::Queen => self.get_queen_move(piece_bitboard, self.get_occupied_pos()),
        }
    }

    fn get_pawn_move(&self, bitboard: Bitboard, color: Color) -> Vec<Moves> {
        let mut m = Vec::new();
        let empty = self.get_free_pos();
        match color {
            Color::White => {
                for p in bitboard.get_single_ones() {
                    let temp_bitboard = Bitboard::new(
                        moves_gen::pawn::white_moves(p.get_value(), empty.get_value())
                            | moves_gen::pawn::white_attack(p.get_value(), self.black.get_value()),
                    );
                    for new_mv in temp_bitboard.get_single_ones() {
                        let temp_move = Moves::new(p.clone(), new_mv);
                        m.push(temp_move);
                    }
                }
            }
            Color::Black => {
                for p in bitboard.get_single_ones() {
                    let temp_bitboard = Bitboard::new(
                        moves_gen::pawn::black_moves(p.get_value(), empty.get_value())
                            | moves_gen::pawn::black_attack(p.get_value(), self.white.get_value()),
                    );
                    for new_mv in temp_bitboard.get_single_ones() {
                        let temp_move = Moves::new(p.clone(), new_mv);
                        m.push(temp_move);
                    }
                }
            }
            Color::Any => {
                panic!("get_move called on type Any")
            }
        }
        m
    }

    fn get_knight_move(&self, bitboard: Bitboard) -> Vec<Moves> {
        let mut m = Vec::new();
        for p in bitboard.get_single_ones() {
            let temp_bitboard = Bitboard::new(moves_gen::knight::moves(p.get_value()));
            for new_mv in temp_bitboard.get_single_ones() {
                let temp_move = Moves::new(p.clone(), new_mv);
                m.push(temp_move);
            }
        }
        m
    }

    fn get_king_move(&self, bitboard: Bitboard) -> Vec<Moves> {
        let mut m = Vec::new();
        for p in bitboard.get_single_ones() {
            let temp_bitboard = Bitboard::new(moves_gen::king::moves(p.get_value()));
            for new_mv in temp_bitboard.get_single_ones() {
                let temp_move = Moves::new(p.clone(), new_mv);
                m.push(temp_move);
            }
        }
        m
    }

    fn get_bishop_move(&self, bitboard: Bitboard, occupied: Bitboard) -> Vec<Moves> {
        let mut m = Vec::new();
        for p in bitboard.get_single_ones() {
            let occ_without_piece = occupied.and(p.not());
            let temp_bitboard = Bitboard::new(moves_gen::bishop::moves(
                p.get_value(),
                occ_without_piece.get_value(),
            ));
            for new_mv in temp_bitboard.get_single_ones() {
                let temp_move = Moves::new(p.clone(), new_mv);
                m.push(temp_move);
            }
        }
        m
    }

    fn get_rook_move(&self, bitboard: Bitboard, occupied: Bitboard) -> Vec<Moves> {
        let mut m = Vec::new();
        for p in bitboard.get_single_ones() {
            let occ_without_piece = occupied.and(p.not());
            let temp_bitboard = Bitboard::new(moves_gen::rook::moves(
                p.get_value(),
                occ_without_piece.get_value(),
            ));
            for new_mv in temp_bitboard.get_single_ones() {
                let temp_move = Moves::new(p.clone(), new_mv);
                m.push(temp_move);
            }
        }
        m
    }

    fn get_queen_move(&self, bitboard: Bitboard, occupied: Bitboard) -> Vec<Moves> {
        let mut m = Vec::new();
        for p in bitboard.get_single_ones() {
            let occ_without_piece = occupied.and(p.not());
            let temp_bitboard = Bitboard::new(moves_gen::queen::moves(
                p.get_value(),
                occ_without_piece.get_value(),
            ));
            for new_mv in temp_bitboard.get_single_ones() {
                let temp_move = Moves::new(p.clone(), new_mv);
                m.push(temp_move);
            }
        }
        m
    }

    // get all attack
    pub fn get_all_attacks(&self, color: Color) -> Bitboard {
        let mut attacks = Bitboard::empty();
        let occupied = self.get_occupied_pos();

        let pawns = self.get_pieces(color, Type::Pawn);
        for p in pawns.get_single_ones() {
            let att = match color {
                Color::White => Bitboard::new(moves_gen::pawn::white_attack(p.get_value(), !0)),
                Color::Black => Bitboard::new(moves_gen::pawn::black_attack(p.get_value(), !0)),
                _ => panic!(),
            };
            attacks = attacks.or(att);
        }

        let knights = self.get_pieces(color, Type::Knight);
        for k in knights.get_single_ones() {
            attacks = attacks.or(Bitboard::new(moves_gen::knight::moves(k.get_value())));
        }

        let bishops = self.get_pieces(color, Type::Bishop);
        for b in bishops.get_single_ones() {
            let occ_without_piece = occupied.and(b.not());
            attacks = attacks.or(Bitboard::new(moves_gen::bishop::moves(
                b.get_value(),
                occ_without_piece.get_value(),
            )));
        }

        let rooks = self.get_pieces(color, Type::Rook);
        for r in rooks.get_single_ones() {
            let occ_without_piece = occupied.and(r.not());
            attacks = attacks.or(Bitboard::new(moves_gen::rook::moves(
                r.get_value(),
                occ_without_piece.get_value(),
            )));
        }

        let queens = self.get_pieces(color, Type::Queen);
        for q in queens.get_single_ones() {
            let occ_without_piece = occupied.and(q.not());
            attacks = attacks.or(Bitboard::new(moves_gen::queen::moves(
                q.get_value(),
                occ_without_piece.get_value(),
            )));
        }

        let king = self.get_pieces(color, Type::King);
        for k in king.get_single_ones() {
            attacks = attacks.or(Bitboard::new(moves_gen::king::moves(k.get_value())));
        }

        attacks
    }

    // castling
    fn castle(&self, color: Color) -> Vec<Moves> {
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
            Side::Long => self.can_castle_long_side(color),
            Side::Short => self.can_castle_short_side(color),
        }
    }

    fn can_castle_long_side(&self, color: Color) -> bool {
        match color {
            Color::White => {
                if self.white_rook_long_side
                    && self.white_king
                    && ((self.get_piece_any(Type::Any).get_value() & WHITELONGCASTLING) == 0)
                {
                    return true;
                }
                false
            }
            Color::Black => {
                if self.black_rook_long_side
                    && self.black_king
                    && ((self.get_piece_any(Type::Any).get_value() & BLACKLONGCASTLING) == 0)
                {
                    return true;
                }
                false
            }
            Color::Any => {
                panic!("rook side color can't be any")
            }
        }
    }

    fn can_castle_short_side(&self, color: Color) -> bool {
        match color {
            Color::White => {
                if self.white_rook_short_side
                    && self.white_king
                    && ((self.get_piece_any(Type::Any).get_value() & WHITESHORTCASTLING) == 0)
                {
                    return true;
                }
                false
            }
            Color::Black => {
                if self.black_rook_short_side
                    && self.black_king
                    && ((self.get_piece_any(Type::Any).get_value() & BLACKSHORTCASTLING) == 0)
                {
                    return true;
                }
                false
            }
            Color::Any => {
                panic!("rook side color can't be any")
            }
        }
    }

    pub fn is_king_in_check(&self, color: Color) -> bool {
        let king_pos = self.get_pieces(color, Type::King);
        let opponent_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
            _ => panic!("Invalid color"),
        };

        let opponent_attacks = self.get_all_attacks(opponent_color);

        king_pos.and(opponent_attacks).get_value() > 0
    }

    //TODO checkmate
    // checkmate

    // promotion
    fn promotion(&self, color: Color) {
        let promotion_ready = self.check_promotion(color);
        if promotion_ready.get_value() == 0 {
            return;
        }
        for pawn in promotion_ready.get_single_ones() {
            self.pawn.xor(pawn);
            self.queen.or(pawn);
        }
    }

    fn check_promotion(&self, color: Color) -> Bitboard {
        let pawn = self.get_pieces(color, Type::Pawn);
        match color {
            Color::White => Bitboard::new(pawn.get_value() & LASTRANK),
            Color::Black => Bitboard::new(pawn.get_value() & FIRSTRANK),
            Color::Any => {
                panic!("color can't be any")
            }
        }
    }

    // move
    pub fn perform_move(&mut self, old_pos: Bitboard, new_pos: Bitboard) -> &Board {
        let (current_color_bb, opponent_color_bb) = if self.is_white_turn {
            (&mut self.white, &mut self.black)
        } else {
            (&mut self.black, &mut self.white)
        };

        let new_pos_value = new_pos.get_value();
        let old_pos_value = old_pos.get_value();

        if (opponent_color_bb.get_value() & new_pos_value) != 0 {
            self.queen = self.queen.and(new_pos.not());
            self.rook = self.rook.and(new_pos.not());
            self.bishop = self.bishop.and(new_pos.not());
            self.knight = self.knight.and(new_pos.not());
            self.pawn = self.pawn.and(new_pos.not());
            self.king = self.king.and(new_pos.not());

            *opponent_color_bb = opponent_color_bb.and(new_pos.not());
        }

        if (self.queen.get_value() & old_pos_value) != 0 {
            self.queen = self.queen.xor(old_pos).or(new_pos);
        } else if (self.rook.get_value() & old_pos_value) != 0 {
            self.rook = self.rook.xor(old_pos).or(new_pos);
        } else if (self.bishop.get_value() & old_pos_value) != 0 {
            self.bishop = self.bishop.xor(old_pos).or(new_pos);
        } else if (self.knight.get_value() & old_pos_value) != 0 {
            self.knight = self.knight.xor(old_pos).or(new_pos);
        } else if (self.pawn.get_value() & old_pos_value) != 0 {
            self.pawn = self.pawn.xor(old_pos).or(new_pos);
        } else if (self.king.get_value() & old_pos_value) != 0 {
            self.king = self.king.xor(old_pos).or(new_pos);
        }

        *current_color_bb = current_color_bb.and(old_pos.not()).or(new_pos);

        if self.is_white_turn {
            self.promotion(Color::White);
        } else {
            self.promotion(Color::Black);
        }

        self.is_white_turn = !self.is_white_turn;
        self
    }
    // to string function
    pub fn to_string(&self) -> String {
        let mut string =
            String::from("■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■");
        // add pieces to the board

        // white pieces
        let white_pawns = self.get_pieces(Color::White, Type::Pawn).to_string();
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

        string = string
            .chars()
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

        let iter_with_newlines = string
            .chars()
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
