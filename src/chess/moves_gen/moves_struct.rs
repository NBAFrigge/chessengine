// Bit layout (u32):
// Bits 0-5:   from
// Bits 6-11:  to
// Bits 12-13: promotion piece
// Bit 14:     is_promotion flag (0/1)
// Bits 15-16: special flags
// Bit 17:     is_null_move_flag
use crate::chess::table::{Board, Type};

const FROM_MASK: u32 = 0x3F; // 000000...111111
const TO_MASK: u32 = 0xFC0; // 000000...111111000000
const PROMO_MASK: u32 = 0x3000; // promotion (bits 12–13)
const IS_PROMO_MASK: u32 = 0x4000; // bit 14
const FLAGS_MASK: u32 = 0x18000; // bits 15–16
const NULL_MOVE_MASK: u32 = 0x20000; // null move

pub const FLAG_NORMAL: u8 = 0;
pub const FLAG_CAPTURE: u8 = 1;
pub const FLAG_CASTLE: u8 = 2;
pub const FLAG_EN_PASSANT: u8 = 3;

pub const PROMOTE_QUEEN: u8 = 0;
pub const PROMOTE_ROOK: u8 = 1;
pub const PROMOTE_BISHOP: u8 = 2;
pub const PROMOTE_KNIGHT: u8 = 3;

const MVV_LVA: [[i32; 6]; 6] = [
    // P   N    B    R    Q    K
    [105, 205, 305, 405, 505, 605], // P
    [104, 204, 304, 404, 504, 604], // N
    [103, 203, 303, 403, 503, 603], // B
    [102, 202, 302, 402, 502, 602], // R
    [101, 201, 301, 401, 501, 601], // Q
    [100, 200, 300, 400, 500, 600], // K
];

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Moves(u32);

impl Moves {
    pub fn new(
        from: u8,
        to: u8,
        promotion: u8,
        flags: u8,
        is_promotion: bool,
        is_null_move: bool,
    ) -> Self {
        let mut m = 0u32;

        m |= (from as u32) & FROM_MASK;
        m |= ((to as u32) << 6) & TO_MASK;
        m |= ((promotion as u32) << 12) & PROMO_MASK;
        m |= ((flags as u32) << 15) & FLAGS_MASK;

        if is_promotion {
            m |= IS_PROMO_MASK;
        }

        if is_null_move {
            m |= NULL_MOVE_MASK
        }

        Moves(m)
    }

    pub fn from(&self) -> u8 {
        (self.0 & FROM_MASK) as u8
    }

    pub fn to(&self) -> u8 {
        ((self.0 & TO_MASK) >> 6) as u8
    }

    pub fn promotion_piece(&self) -> u8 {
        ((self.0 & PROMO_MASK) >> 12) as u8
    }

    pub fn promotion_piece_type(&self) -> Type {
        match self.promotion_piece() {
            PROMOTE_BISHOP => Type::Bishop,
            PROMOTE_QUEEN => Type::Queen,
            PROMOTE_ROOK => Type::Rook,
            PROMOTE_KNIGHT => Type::Knight,
            _ => panic!("promotion_piece_type error"),
        }
    }

    pub fn is_promotion(&self) -> bool {
        (self.0 & IS_PROMO_MASK) != 0
    }

    pub fn is_capture(&self) -> bool {
        (self.flags() == FLAG_CAPTURE) || (self.flags() == FLAG_EN_PASSANT)
    }

    pub fn is_castle(&self) -> bool {
        self.flags() == FLAG_CASTLE
    }

    pub fn is_enpassant(&self) -> bool {
        self.flags() == FLAG_EN_PASSANT
    }

    pub fn is_null_move(&self) -> bool {
        self.0 & NULL_MOVE_MASK > 0
    }

    pub fn flags(&self) -> u8 {
        ((self.0 & FLAGS_MASK) >> 15) as u8
    }

    pub fn score(&self, b: &Board) -> i32 {
        if self.is_promotion() && self.is_capture() {
            let promo_value = match self.promotion_piece() {
                PROMOTE_QUEEN => 9000,
                PROMOTE_ROOK => 500,
                PROMOTE_BISHOP => 330,
                PROMOTE_KNIGHT => 320,
                _ => 100,
            };

            if let Some(victim) = b.get_piece_type_at_square(self.to()) {
                let victim_value = get_piece_value(victim);
                return 2_000_000 + promo_value + victim_value;
            }
        }

        if self.is_promotion() {
            let promo_bonus = match self.promotion_piece() {
                PROMOTE_QUEEN => 900,
                PROMOTE_ROOK => 500,
                PROMOTE_BISHOP => 330,
                PROMOTE_KNIGHT => 320,
                _ => 100,
            };
            return 800_000 + promo_bonus;
        }

        if self.flags() == FLAG_CAPTURE {
            if let Some(victim) = b.get_piece_type_at_square(self.to()) {
                if let Some(attacker) = b.get_piece_type_at_square(self.from()) {
                    let a = attacker.id() as usize;
                    let v = victim.id() as usize;
                    return 1_000_000 + MVV_LVA[a][v];
                }
            }
        }

        if self.flags() == FLAG_EN_PASSANT {
            return 1_000_000 + MVV_LVA[0][0];
        }

        if self.flags() == FLAG_CASTLE {
            return 500_000;
        }

        0
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let from = self.index_to_algebraic(self.from());
        let to = self.index_to_algebraic(self.to());

        let mut s = format!("{}{}", from, to);

        if self.is_promotion() {
            let p = match self.promotion_piece() {
                0 => 'q',
                1 => 'r',
                2 => 'b',
                3 => 'n',
                _ => '?',
            };
            s.push(p);
        }

        s
    }

    #[allow(dead_code)]
    fn index_to_algebraic(&self, index: u8) -> String {
        let file = (b'a' + (index % 8)) as char;
        let rank = (b'1' + (index / 8)) as char;
        format!("{}{}", file, rank)
    }
}
fn get_piece_value(t: Type) -> i32 {
    match t {
        Type::Pawn => 100,
        Type::Knight => 320,
        Type::Bishop => 330,
        Type::Rook => 500,
        Type::Queen => 900,
        Type::King => 20000,
        _ => 0,
    }
}
