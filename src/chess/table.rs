use crate::bitboard::bitboard::Bitboard;
use crate::chess::moves_gen;
use crate::chess::moves_gen::moves_struct::{
    FLAG_CAPTURE, FLAG_CASTLE, FLAG_EN_PASSANT, FLAG_NORMAL, Moves, PROMOTE_BISHOP, PROMOTE_KNIGHT,
    PROMOTE_QUEEN, PROMOTE_ROOK,
};
use either::Either;
use std::cmp::PartialEq;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const FIRSTRANK: u64 = 0xff;
const LASTRANK: u64 = 0xff00000000000000;
pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_H: u64 = 0x8080808080808080;

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
    Any,
}

#[derive(Debug, EnumIter, Copy, Clone, PartialEq)]
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

struct MoveContext {
    own_pieces: u64,
    enemy_pieces: u64,
    occupied: u64,
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
    enpassant: Bitboard,
}

#[derive(Clone, Copy)]
pub struct UndoInfo {
    captured_piece_type: u8,
    captured_on_white: bool,
    white_rook_long_side: bool,
    white_rook_short_side: bool,
    black_rook_long_side: bool,
    black_rook_short_side: bool,
    white_king: bool,
    black_king: bool,
    old_enpassant: u64,
    was_white_turn: bool,
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
            enpassant: Bitboard::new(0),
        }
    }

    #[inline]
    pub fn get_pieces(&self, color: Color, piece_type: Type) -> Bitboard {
        match color {
            Color::White => self.get_piece_white(piece_type),
            Color::Black => self.get_piece_black(piece_type),
            Color::Any => self.get_piece_any(piece_type),
        }
    }

    #[inline]
    fn get_piece_any(&self, piece_type: Type) -> Bitboard {
        match piece_type {
            Type::Any => self.white.or(self.black),
            Type::Pawn => self.pawn,
            Type::Bishop => self.bishop,
            Type::Knight => self.knight,
            Type::Rook => self.rook,
            Type::Queen => self.queen,
            Type::King => self.king,
        }
    }

    #[inline]
    fn get_piece_white(&self, piece_type: Type) -> Bitboard {
        match piece_type {
            Type::Any => self.white,
            Type::Pawn => self.pawn.and(self.white),
            Type::Bishop => self.bishop.and(self.white),
            Type::Knight => self.knight.and(self.white),
            Type::Rook => self.rook.and(self.white),
            Type::Queen => self.queen.and(self.white),
            Type::King => self.king.and(self.white),
        }
    }

    #[inline]
    fn get_piece_black(&self, piece_type: Type) -> Bitboard {
        match piece_type {
            Type::Any => self.black,
            Type::Pawn => self.pawn.and(self.black),
            Type::Bishop => self.bishop.and(self.black),
            Type::Knight => self.knight.and(self.black),
            Type::Rook => self.rook.and(self.black),
            Type::Queen => self.queen.and(self.black),
            Type::King => self.king.and(self.black),
        }
    }

    #[inline]
    pub fn get_free_pos(&self) -> Bitboard {
        self.white.or(self.black).not()
    }

    #[inline]
    pub fn get_occupied_pos(&self) -> Bitboard {
        self.white.or(self.black)
    }

    pub fn get_legal_moves<'a>(&mut self, color: Color, buffer: &'a mut Vec<Moves>) -> &'a [Moves] {
        buffer.clear();
        self.get_all_moves_bitboard(color, buffer);

        let mut legal_index = 0;
        for i in 0..buffer.len() {
            let mv = buffer[i];
            let undo = self.make_move_with_undo(&mv);
            if !self.is_king_in_check(color) {
                buffer[legal_index] = mv;
                legal_index += 1;
            }
            self.unmake_move(&mv, undo);
        }

        buffer.truncate(legal_index);
        &buffer[..]
    }

    pub fn get_all_moves_bitboard<'a>(
        &self,
        color: Color,
        buffer: &'a mut Vec<Moves>,
    ) -> &'a [Moves] {
        buffer.clear();
        let context = self.create_move_context(color);

        for piece_type in Type::iter() {
            if piece_type == Type::Any {
                continue;
            }
            self.get_moves(color, piece_type, &context, buffer);
        }
        self.castle(color, buffer);
        &buffer[..]
    }

    #[inline]
    fn create_move_context(&self, color: Color) -> MoveContext {
        match color {
            Color::White => MoveContext {
                own_pieces: self.white.get_value(),
                enemy_pieces: self.black.get_value(),
                occupied: self.white.or(self.black).get_value(),
            },
            Color::Black => MoveContext {
                own_pieces: self.black.get_value(),
                enemy_pieces: self.white.get_value(),
                occupied: self.white.or(self.black).get_value(),
            },
            _ => panic!("Invalid color"),
        }
    }

    pub fn get_moves(
        &self,
        color: Color,
        piece_type: Type,
        context: &MoveContext,
        buffer: &mut Vec<Moves>,
    ) {
        let piece_bitboard = self.get_pieces(color, piece_type);

        match piece_type {
            Type::Any => panic!("get_move called on type Any"),
            Type::Pawn => self.get_pawn_moves(piece_bitboard, color, context, buffer),
            Type::King => self.get_king_move(piece_bitboard, context, buffer),
            Type::Bishop => self.get_bishop_move(piece_bitboard, context, buffer),
            Type::Knight => self.get_knight_move(piece_bitboard, context, buffer),
            Type::Rook => self.get_rook_move(piece_bitboard, context, buffer),
            Type::Queen => self.get_queen_moves(piece_bitboard, context, buffer),
        }
    }

    fn get_pawn_moves(
        &self,
        bitboard: Bitboard,
        color: Color,
        context: &MoveContext,
        buffer: &mut Vec<Moves>,
    ) {
        let empty = self.white.or(self.black).not().get_value();
        let enpassant = self.enpassant.get_value();

        for p in bitboard.iter_bits() {
            let from_square = p.lsb() as u8;
            let p_val = p.get_value();

            match color {
                Color::White => {
                    let temp_moves = moves_gen::pawn::white_moves(p_val, empty)
                        | moves_gen::pawn::white_attack(p_val, context.enemy_pieces);

                    let mut temp_bb = temp_moves;
                    while temp_bb != 0 {
                        let to_square = temp_bb.trailing_zeros() as u8;
                        let to_bit = 1u64 << to_square;
                        temp_bb &= temp_bb - 1;

                        let is_capture = (to_bit & context.enemy_pieces) != 0;
                        let is_promotion = (to_bit & LASTRANK) != 0;

                        if is_promotion {
                            let flag = if is_capture {
                                FLAG_CAPTURE
                            } else {
                                FLAG_NORMAL
                            };
                            buffer.push(Moves::new(from_square, to_square, PROMOTE_QUEEN, flag));
                            buffer.push(Moves::new(from_square, to_square, PROMOTE_ROOK, flag));
                            buffer.push(Moves::new(from_square, to_square, PROMOTE_BISHOP, flag));
                            buffer.push(Moves::new(from_square, to_square, PROMOTE_KNIGHT, flag));
                        } else {
                            let flag = if is_capture {
                                FLAG_CAPTURE
                            } else {
                                FLAG_NORMAL
                            };
                            buffer.push(Moves::new(from_square, to_square, 0, flag));
                        }
                    }

                    if enpassant != 0 {
                        let landing = enpassant << 8;
                        let left_hit = (p_val << 7) & landing & !FILE_H;
                        let right_hit = (p_val << 9) & landing & !FILE_A;

                        if left_hit != 0 || right_hit != 0 {
                            let to_square = landing.trailing_zeros() as u8;
                            buffer.push(Moves::new(from_square, to_square, 0, FLAG_EN_PASSANT));
                        }
                    }
                }
                Color::Black => {
                    let temp_moves = moves_gen::pawn::black_moves(p_val, empty)
                        | moves_gen::pawn::black_attack(p_val, context.enemy_pieces);

                    let mut temp_bb = temp_moves;
                    while temp_bb != 0 {
                        let to_square = temp_bb.trailing_zeros() as u8;
                        let to_bit = 1u64 << to_square;
                        temp_bb &= temp_bb - 1;

                        let is_capture = (to_bit & context.enemy_pieces) != 0;
                        let is_promotion = (to_bit & FIRSTRANK) != 0;

                        if is_promotion {
                            let flag = if is_capture {
                                FLAG_CAPTURE
                            } else {
                                FLAG_NORMAL
                            };
                            buffer.push(Moves::new(from_square, to_square, PROMOTE_QUEEN, flag));
                            buffer.push(Moves::new(from_square, to_square, PROMOTE_ROOK, flag));
                            buffer.push(Moves::new(from_square, to_square, PROMOTE_BISHOP, flag));
                            buffer.push(Moves::new(from_square, to_square, PROMOTE_KNIGHT, flag));
                        } else {
                            let flag = if is_capture {
                                FLAG_CAPTURE
                            } else {
                                FLAG_NORMAL
                            };
                            buffer.push(Moves::new(from_square, to_square, 0, flag));
                        }
                    }

                    if enpassant != 0 {
                        let landing = enpassant >> 8;
                        let left_hit = (p_val >> 9) & landing & !FILE_H;
                        let right_hit = (p_val >> 7) & landing & !FILE_A;

                        if left_hit != 0 || right_hit != 0 {
                            let to_square = landing.trailing_zeros() as u8;
                            buffer.push(Moves::new(from_square, to_square, 0, FLAG_EN_PASSANT));
                        }
                    }
                }
                Color::Any => panic!("get_pawn_move called with Color::Any"),
            }
        }
    }

    fn get_knight_move(&self, bitboard: Bitboard, context: &MoveContext, buffer: &mut Vec<Moves>) {
        for p in bitboard.iter_bits() {
            let from_square = p.lsb() as u8;
            let temp_moves = moves_gen::knight::moves(p.get_value()) & !context.own_pieces;

            let mut temp_bb = temp_moves;
            while temp_bb != 0 {
                let to_square = temp_bb.trailing_zeros() as u8;
                let to_bit = 1u64 << to_square;
                temp_bb &= temp_bb - 1;

                let flag = if (to_bit & context.enemy_pieces) != 0 {
                    FLAG_CAPTURE
                } else {
                    FLAG_NORMAL
                };

                buffer.push(Moves::new(from_square, to_square, 0, flag));
            }
        }
    }

    fn get_queen_moves(&self, bitboard: Bitboard, context: &MoveContext, buffer: &mut Vec<Moves>) {
        for p in bitboard.iter_bits() {
            let from_square = p.lsb() as u8;
            let occ_without_piece = context.occupied & !p.get_value();
            let temp_moves =
                moves_gen::queen::moves(p.get_value(), occ_without_piece) & !context.own_pieces;

            let mut temp_bb = temp_moves;
            while temp_bb != 0 {
                let to_square = temp_bb.trailing_zeros() as u8;
                let to_bit = 1u64 << to_square;
                temp_bb &= temp_bb - 1;

                let flag = if (to_bit & context.enemy_pieces) != 0 {
                    FLAG_CAPTURE
                } else {
                    FLAG_NORMAL
                };

                buffer.push(Moves::new(from_square, to_square, 0, flag));
            }
        }
    }

    fn get_bishop_move(&self, bitboard: Bitboard, context: &MoveContext, buffer: &mut Vec<Moves>) {
        for p in bitboard.iter_bits() {
            let from_square = p.lsb() as u8;
            let occ_without_piece = context.occupied & !p.get_value();
            let temp_moves =
                moves_gen::bishop::moves(p.get_value(), occ_without_piece) & !context.own_pieces;

            let mut temp_bb = temp_moves;
            while temp_bb != 0 {
                let to_square = temp_bb.trailing_zeros() as u8;
                let to_bit = 1u64 << to_square;
                temp_bb &= temp_bb - 1;

                let flag = if (to_bit & context.enemy_pieces) != 0 {
                    FLAG_CAPTURE
                } else {
                    FLAG_NORMAL
                };

                buffer.push(Moves::new(from_square, to_square, 0, flag));
            }
        }
    }

    fn get_rook_move(&self, bitboard: Bitboard, context: &MoveContext, buffer: &mut Vec<Moves>) {
        for p in bitboard.iter_bits() {
            let from_square = p.lsb() as u8;
            let occ_without_piece = context.occupied & !p.get_value();
            let temp_moves =
                moves_gen::rook::moves(p.get_value(), occ_without_piece) & !context.own_pieces;

            let mut temp_bb = temp_moves;
            while temp_bb != 0 {
                let to_square = temp_bb.trailing_zeros() as u8;
                let to_bit = 1u64 << to_square;
                temp_bb &= temp_bb - 1;

                let flag = if (to_bit & context.enemy_pieces) != 0 {
                    FLAG_CAPTURE
                } else {
                    FLAG_NORMAL
                };

                buffer.push(Moves::new(from_square, to_square, 0, flag));
            }
        }
    }

    fn get_king_move(&self, bitboard: Bitboard, context: &MoveContext, buffer: &mut Vec<Moves>) {
        for p in bitboard.iter_bits() {
            let from_square = p.lsb() as u8;
            let temp_moves = moves_gen::king::moves(p.get_value()) & !context.own_pieces;

            let mut temp_bb = temp_moves;
            while temp_bb != 0 {
                let to_square = temp_bb.trailing_zeros() as u8;
                let to_bit = 1u64 << to_square;
                temp_bb &= temp_bb - 1;

                let flag = if (to_bit & context.enemy_pieces) != 0 {
                    FLAG_CAPTURE
                } else {
                    FLAG_NORMAL
                };

                buffer.push(Moves::new(from_square, to_square, 0, flag));
            }
        }
    }

    fn castle(&self, color: Color, buffer: &mut Vec<Moves>) {
        match color {
            Color::White => {
                if self.can_castle(color, Side::Long) {
                    buffer.push(Moves::new(4, 2, 0, FLAG_CASTLE));
                }
                if self.can_castle(color, Side::Short) {
                    buffer.push(Moves::new(4, 6, 0, FLAG_CASTLE));
                }
            }
            Color::Black => {
                if self.can_castle(color, Side::Long) {
                    buffer.push(Moves::new(60, 58, 0, FLAG_CASTLE));
                }
                if self.can_castle(color, Side::Short) {
                    buffer.push(Moves::new(60, 62, 0, FLAG_CASTLE));
                }
            }
            _ => {}
        }
    }

    fn can_castle(&self, color: Color, side: Side) -> bool {
        match side {
            Side::Long => self.can_castle_long_side(color),
            Side::Short => self.can_castle_short_side(color),
        }
    }

    fn can_castle_long_side(&self, color: Color) -> bool {
        let opponent_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
            Color::Any => return false,
        };

        match color {
            Color::White => {
                if !self.white_king || !self.white_rook_long_side {
                    return false;
                }
                if (self.get_occupied_pos().get_value() & 0x0E) != 0 {
                    return false;
                }
                if self.is_king_in_check(color) {
                    return false;
                }
                if self.is_square_attacked_by(3, opponent_color)
                    || self.is_square_attacked_by(2, opponent_color)
                {
                    return false;
                }
                true
            }
            Color::Black => {
                if !self.black_king || !self.black_rook_long_side {
                    return false;
                }
                if (self.get_occupied_pos().get_value() & 0x0E00000000000000) != 0 {
                    return false;
                }
                if self.is_king_in_check(color) {
                    return false;
                }
                if self.is_square_attacked_by(59, opponent_color)
                    || self.is_square_attacked_by(58, opponent_color)
                {
                    return false;
                }
                true
            }
            Color::Any => panic!("rook side color can't be any"),
        }
    }

    fn can_castle_short_side(&self, color: Color) -> bool {
        let opponent_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
            Color::Any => return false,
        };

        match color {
            Color::White => {
                if !self.white_king || !self.white_rook_short_side {
                    return false;
                }
                if (self.get_occupied_pos().get_value() & 0x60) != 0 {
                    return false;
                }
                if self.is_king_in_check(color) {
                    return false;
                }
                if self.is_square_attacked_by(5, opponent_color)
                    || self.is_square_attacked_by(6, opponent_color)
                {
                    return false;
                }
                true
            }
            Color::Black => {
                if !self.black_king || !self.black_rook_short_side {
                    return false;
                }
                if (self.get_occupied_pos().get_value() & 0x6000000000000000) != 0 {
                    return false;
                }
                if self.is_king_in_check(color) {
                    return false;
                }
                if self.is_square_attacked_by(61, opponent_color)
                    || self.is_square_attacked_by(62, opponent_color)
                {
                    return false;
                }
                true
            }
            Color::Any => panic!("rook side color can't be any"),
        }
    }

    #[inline(always)]
    pub fn is_king_in_check(&self, color: Color) -> bool {
        let king_bb = self.get_pieces(color, Type::King);
        let king_square = king_bb.get_value().trailing_zeros() as u8;

        self.is_square_attacked_by(
            king_square,
            match color {
                Color::White => Color::Black,
                Color::Black => Color::White,
                _ => panic!(),
            },
        )
    }

    fn is_square_attacked_by(&self, square: u8, by_color: Color) -> bool {
        let square_bb = 1u64 << square;
        let occupied = self.get_occupied_pos().get_value();

        let enemy_pawns = self.get_pieces(by_color, Type::Pawn).get_value();
        if self.pawn_attacks_square(square, by_color, enemy_pawns) {
            return true;
        }

        let knight_attacks = moves_gen::knight::moves(square_bb);
        let enemy_knights = self.get_pieces(by_color, Type::Knight).get_value();
        if knight_attacks & enemy_knights != 0 {
            return true;
        }

        let king_attacks = moves_gen::king::moves(square_bb);
        let enemy_king = self.get_pieces(by_color, Type::King).get_value();
        if king_attacks & enemy_king != 0 {
            return true;
        }

        if self.sliding_attacks_square(square, by_color, occupied) {
            return true;
        }

        false
    }

    #[inline(always)]
    fn pawn_attacks_square(&self, square: u8, by_color: Color, enemy_pawns: u64) -> bool {
        match by_color {
            Color::White => {
                let attacks = if square >= 8 {
                    let mut att = 0u64;
                    if square % 8 != 0 {
                        att |= 1u64 << (square - 9);
                    }
                    if square % 8 != 7 {
                        att |= 1u64 << (square - 7);
                    }
                    att
                } else {
                    0
                };
                attacks & enemy_pawns != 0
            }
            Color::Black => {
                let attacks = if square < 56 {
                    let mut att = 0u64;
                    if square % 8 != 0 {
                        att |= 1u64 << (square + 7);
                    }
                    if square % 8 != 7 {
                        att |= 1u64 << (square + 9);
                    }
                    att
                } else {
                    0
                };
                attacks & enemy_pawns != 0
            }
            _ => false,
        }
    }

    #[inline(always)]
    fn sliding_attacks_square(&self, square: u8, by_color: Color, occupied: u64) -> bool {
        let square_bb = 1u64 << square;

        let rook_attacks = moves_gen::rook::moves(square_bb, occupied);
        let enemy_rooks = self.get_pieces(by_color, Type::Rook).get_value();
        let enemy_queens = self.get_pieces(by_color, Type::Queen).get_value();

        if rook_attacks & (enemy_rooks | enemy_queens) != 0 {
            return true;
        }

        let bishop_attacks = moves_gen::bishop::moves(square_bb, occupied);
        let enemy_bishops = self.get_pieces(by_color, Type::Bishop).get_value();

        if bishop_attacks & (enemy_bishops | enemy_queens) != 0 {
            return true;
        }

        false
    }

    pub fn perform_move(&mut self, mv: &Moves) -> &Board {
        let old_enpassant = self.enpassant;
        self.enpassant.set_empty();

        match mv.flags() {
            FLAG_CASTLE => {
                self.perform_castle_move(mv);
            }
            FLAG_EN_PASSANT => {
                self.perform_enpassant_move(mv, old_enpassant);
            }
            _ => {
                self.perform_normal_move(mv);
            }
        }

        self.is_white_turn = !self.is_white_turn;
        self
    }

    #[inline]
    fn perform_castle_move(&mut self, mv: &Moves) {
        match (mv.from(), mv.to()) {
            (4, 6) => {
                self.king = self.king.xor(Bitboard::new(0x50));
                self.white = self.white.xor(Bitboard::new(0x50));
                self.rook = self.rook.xor(Bitboard::new(0xA0));
                self.white = self.white.xor(Bitboard::new(0xA0));
                self.white_king = false;
                self.white_rook_short_side = false;
            }
            (4, 2) => {
                self.king = self.king.xor(Bitboard::new(0x14));
                self.white = self.white.xor(Bitboard::new(0x14));
                self.rook = self.rook.xor(Bitboard::new(0x09));
                self.white = self.white.xor(Bitboard::new(0x09));
                self.white_king = false;
                self.white_rook_long_side = false;
            }
            (60, 62) => {
                self.king = self.king.xor(Bitboard::new(0x5000000000000000));
                self.black = self.black.xor(Bitboard::new(0x5000000000000000));
                self.rook = self.rook.xor(Bitboard::new(0xA000000000000000));
                self.black = self.black.xor(Bitboard::new(0xA000000000000000));
                self.black_king = false;
                self.black_rook_short_side = false;
            }
            (60, 58) => {
                self.king = self.king.xor(Bitboard::new(0x1400000000000000));
                self.black = self.black.xor(Bitboard::new(0x1400000000000000));
                self.rook = self.rook.xor(Bitboard::new(0x0900000000000000));
                self.black = self.black.xor(Bitboard::new(0x0900000000000000));
                self.black_king = false;
                self.black_rook_long_side = false;
            }
            _ => {}
        }
    }

    #[inline]
    fn perform_enpassant_move(&mut self, mv: &Moves, old_enpassant: Bitboard) {
        let from_bb = 1u64 << mv.from();
        let to_bb = 1u64 << mv.to();

        self.pawn = self.pawn.xor(old_enpassant);

        if self.is_white_turn {
            self.black = self.black.xor(old_enpassant);
            self.pawn = self.pawn.xor(Bitboard::new(from_bb | to_bb));
            self.white = self.white.xor(Bitboard::new(from_bb | to_bb));
        } else {
            self.white = self.white.xor(old_enpassant);
            self.pawn = self.pawn.xor(Bitboard::new(from_bb | to_bb));
            self.black = self.black.xor(Bitboard::new(from_bb | to_bb));
        }
    }

    #[inline]
    fn perform_normal_move(&mut self, mv: &Moves) {
        let from_bb = 1u64 << mv.from();
        let to_bb = 1u64 << mv.to();

        self.queen = self.queen.and(Bitboard::new(!to_bb));
        self.rook = self.rook.and(Bitboard::new(!to_bb));
        self.bishop = self.bishop.and(Bitboard::new(!to_bb));
        self.knight = self.knight.and(Bitboard::new(!to_bb));
        self.pawn = self.pawn.and(Bitboard::new(!to_bb));
        self.king = self.king.and(Bitboard::new(!to_bb));

        let old_pos_bb = Bitboard::new(from_bb);
        let new_pos_bb = Bitboard::new(to_bb);

        if self.queen.and(old_pos_bb).get_value() != 0 {
            self.queen = self.queen.xor(Bitboard::new(from_bb | to_bb));
        } else if self.rook.and(old_pos_bb).get_value() != 0 {
            self.rook = self.rook.xor(Bitboard::new(from_bb | to_bb));
            match from_bb {
                0x1 => self.white_rook_long_side = false,
                0x80 => self.white_rook_short_side = false,
                0x100000000000000 => self.black_rook_long_side = false,
                0x8000000000000000 => self.black_rook_short_side = false,
                _ => {}
            }
        } else if self.bishop.and(old_pos_bb).get_value() != 0 {
            self.bishop = self.bishop.xor(Bitboard::new(from_bb | to_bb));
        } else if self.knight.and(old_pos_bb).get_value() != 0 {
            self.knight = self.knight.xor(Bitboard::new(from_bb | to_bb));
        } else if self.pawn.and(old_pos_bb).get_value() != 0 {
            let white_double_move = (from_bb & 0xFF00) != 0 && (to_bb & 0xFF000000) != 0;
            let black_double_move =
                (from_bb & 0xFF000000000000) != 0 && (to_bb & 0xFF00000000) != 0;

            if white_double_move || black_double_move {
                self.enpassant = new_pos_bb;
            }

            if mv.promotion() > 0 {
                self.pawn = self.pawn.and(Bitboard::new(!from_bb));
                match mv.promotion() {
                    PROMOTE_QUEEN => self.queen = self.queen.or(new_pos_bb),
                    PROMOTE_ROOK => self.rook = self.rook.or(new_pos_bb),
                    PROMOTE_BISHOP => self.bishop = self.bishop.or(new_pos_bb),
                    PROMOTE_KNIGHT => self.knight = self.knight.or(new_pos_bb),
                    _ => {}
                }
            } else {
                self.pawn = self.pawn.xor(Bitboard::new(from_bb | to_bb));
            }
        } else if self.king.and(old_pos_bb).get_value() != 0 {
            self.king = self.king.xor(Bitboard::new(from_bb | to_bb));
            if self.is_white_turn {
                self.white_king = false;
            } else {
                self.black_king = false;
            }
        }

        self.white = self.white.and(Bitboard::new(!to_bb));
        self.black = self.black.and(Bitboard::new(!to_bb));

        if self.is_white_turn {
            self.white = self.white.xor(Bitboard::new(from_bb | to_bb));
        } else {
            self.black = self.black.xor(Bitboard::new(from_bb | to_bb));
        }
    }

    #[inline(always)]
    pub fn make_move_with_undo(&mut self, mv: &Moves) -> UndoInfo {
        let undo_info = UndoInfo {
            captured_piece_type: self.get_piece_type_at_square(mv.to()),
            captured_on_white: (self.white.get_value() & (1u64 << mv.to())) != 0,
            white_rook_long_side: self.white_rook_long_side,
            white_rook_short_side: self.white_rook_short_side,
            black_rook_long_side: self.black_rook_long_side,
            black_rook_short_side: self.black_rook_short_side,
            white_king: self.white_king,
            black_king: self.black_king,
            old_enpassant: self.enpassant.get_value(),
            was_white_turn: self.is_white_turn,
        };

        self.perform_move(mv);
        undo_info
    }
    #[inline(always)]
    pub fn unmake_move(&mut self, mv: &Moves, undo_info: UndoInfo) {
        self.is_white_turn = undo_info.was_white_turn;
        self.enpassant = Bitboard::new(undo_info.old_enpassant);
        self.white_rook_long_side = undo_info.white_rook_long_side;
        self.white_rook_short_side = undo_info.white_rook_short_side;
        self.black_rook_long_side = undo_info.black_rook_long_side;
        self.black_rook_short_side = undo_info.black_rook_short_side;
        self.white_king = undo_info.white_king;
        self.black_king = undo_info.black_king;

        match mv.flags() {
            FLAG_NORMAL | FLAG_CAPTURE => {
                self.unmake_simple_move(mv, &undo_info);
            }
            FLAG_CASTLE => {
                self.unmake_castle(mv, &undo_info);
            }
            FLAG_EN_PASSANT => {
                self.unmake_enpassant_move(mv, &undo_info);
            }
            _ => {}
        }
    }

    #[inline]
    fn get_piece_type_at_square(&self, square: u8) -> u8 {
        let bb = 1u64 << square;
        if self.pawn.get_value() & bb != 0 {
            return 0;
        }
        if self.knight.get_value() & bb != 0 {
            return 1;
        }
        if self.bishop.get_value() & bb != 0 {
            return 2;
        }
        if self.rook.get_value() & bb != 0 {
            return 3;
        }
        if self.queen.get_value() & bb != 0 {
            return 4;
        }
        if self.king.get_value() & bb != 0 {
            return 5;
        }
        255
    }

    fn unmake_simple_move(&mut self, mv: &Moves, undo_info: &UndoInfo) {
        let from_bb = 1u64 << mv.from();
        let to_bb = 1u64 << mv.to();

        if mv.promotion() > 0 {
            match mv.promotion() {
                PROMOTE_QUEEN => self.queen = self.queen.and(Bitboard::new(!to_bb)),
                PROMOTE_ROOK => self.rook = self.rook.and(Bitboard::new(!to_bb)),
                PROMOTE_BISHOP => self.bishop = self.bishop.and(Bitboard::new(!to_bb)),
                PROMOTE_KNIGHT => self.knight = self.knight.and(Bitboard::new(!to_bb)),
                _ => {}
            }
            self.pawn = self.pawn.or(Bitboard::new(from_bb));
        } else {
            let moving_piece_type = self.get_piece_type_at_square(mv.to());

            match moving_piece_type {
                0 => {
                    self.pawn = self
                        .pawn
                        .and(Bitboard::new(!to_bb))
                        .or(Bitboard::new(from_bb))
                }
                1 => {
                    self.knight = self
                        .knight
                        .and(Bitboard::new(!to_bb))
                        .or(Bitboard::new(from_bb))
                }
                2 => {
                    self.bishop = self
                        .bishop
                        .and(Bitboard::new(!to_bb))
                        .or(Bitboard::new(from_bb))
                }
                3 => {
                    self.rook = self
                        .rook
                        .and(Bitboard::new(!to_bb))
                        .or(Bitboard::new(from_bb))
                }
                4 => {
                    self.queen = self
                        .queen
                        .and(Bitboard::new(!to_bb))
                        .or(Bitboard::new(from_bb))
                }
                5 => {
                    self.king = self
                        .king
                        .and(Bitboard::new(!to_bb))
                        .or(Bitboard::new(from_bb))
                }
                _ => {}
            }
        }

        self.white = self.white.and(Bitboard::new(!to_bb));
        self.black = self.black.and(Bitboard::new(!to_bb));

        if undo_info.was_white_turn {
            self.white = self.white.or(Bitboard::new(from_bb));
        } else {
            self.black = self.black.or(Bitboard::new(from_bb));
        }

        if undo_info.captured_piece_type != 255 {
            match undo_info.captured_piece_type {
                0 => self.pawn = self.pawn.or(Bitboard::new(to_bb)),
                1 => self.knight = self.knight.or(Bitboard::new(to_bb)),
                2 => self.bishop = self.bishop.or(Bitboard::new(to_bb)),
                3 => self.rook = self.rook.or(Bitboard::new(to_bb)),
                4 => self.queen = self.queen.or(Bitboard::new(to_bb)),
                5 => self.king = self.king.or(Bitboard::new(to_bb)),
                _ => {}
            }

            if undo_info.captured_on_white {
                self.white = self.white.or(Bitboard::new(to_bb));
            } else {
                self.black = self.black.or(Bitboard::new(to_bb));
            }
        }
    }

    fn unmake_castle(&mut self, mv: &Moves, undo_info: &UndoInfo) {
        match (mv.from(), mv.to()) {
            // White short castle
            (4, 6) => {
                self.king = self.king.xor(Bitboard::new(0x50));
                self.white = self.white.xor(Bitboard::new(0x50));
                self.rook = self.rook.xor(Bitboard::new(0xA0));
                self.white = self.white.xor(Bitboard::new(0xA0));
            }
            // White long castle
            (4, 2) => {
                self.king = self.king.xor(Bitboard::new(0x14));
                self.white = self.white.xor(Bitboard::new(0x14));
                self.rook = self.rook.xor(Bitboard::new(0x09));
                self.white = self.white.xor(Bitboard::new(0x09));
            }
            // Black short castle
            (60, 62) => {
                self.king = self.king.xor(Bitboard::new(0x5000000000000000));
                self.black = self.black.xor(Bitboard::new(0x5000000000000000));
                self.rook = self.rook.xor(Bitboard::new(0xA000000000000000));
                self.black = self.black.xor(Bitboard::new(0xA000000000000000));
            }
            // Black long castle
            (60, 58) => {
                self.king = self.king.xor(Bitboard::new(0x1400000000000000));
                self.black = self.black.xor(Bitboard::new(0x1400000000000000));
                self.rook = self.rook.xor(Bitboard::new(0x0900000000000000));
                self.black = self.black.xor(Bitboard::new(0x0900000000000000));
            }
            _ => {}
        }
    }

    fn unmake_enpassant_move(&mut self, mv: &Moves, undo_info: &UndoInfo) {
        let from_bb = 1u64 << mv.from();
        let to_bb = 1u64 << mv.to();

        self.pawn = self
            .pawn
            .and(Bitboard::new(!to_bb))
            .or(Bitboard::new(from_bb));

        if undo_info.was_white_turn {
            self.white = self
                .white
                .and(Bitboard::new(!to_bb))
                .or(Bitboard::new(from_bb));
            let captured_pawn_pos = to_bb >> 8;
            self.pawn = self.pawn.or(Bitboard::new(captured_pawn_pos));
            self.black = self.black.or(Bitboard::new(captured_pawn_pos));
        } else {
            self.black = self
                .black
                .and(Bitboard::new(!to_bb))
                .or(Bitboard::new(from_bb));
            let captured_pawn_pos = to_bb << 8;
            self.pawn = self.pawn.or(Bitboard::new(captured_pawn_pos));
            self.white = self.white.or(Bitboard::new(captured_pawn_pos));
        }
    }

    pub fn to_string(&self) -> String {
        let mut string =
            String::from("■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■■□■□■□■□□■□■□■□■");

        let white_pawns = self.get_pieces(Color::White, Type::Pawn).to_string();
        let white_rooks = self.get_pieces(Color::White, Type::Rook).to_string();
        let white_queens = self.get_pieces(Color::White, Type::Queen).to_string();
        let white_kings = self.get_pieces(Color::White, Type::King).to_string();
        let white_bishops = self.get_pieces(Color::White, Type::Bishop).to_string();
        let white_knights = self.get_pieces(Color::White, Type::Knight).to_string();

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
