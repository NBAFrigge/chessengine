// Bit layout (u32):
// Bits 0-5:   from
// Bits 6-11:  to
// Bits 12-13: promotion piece
// Bit 14:     is_promotion flag (0/1)
// Bits 15-16: special flags

const FROM_MASK: u32 = 0x3F; // 000000...111111
const TO_MASK: u32 = 0xFC0; // 000000...111111000000
const PROMO_MASK: u32 = 0x3000; // promotion (bits 12–13)
const IS_PROMO_MASK: u32 = 0x4000; // bit 14
const FLAGS_MASK: u32 = 0x18000; // bits 15–16

pub const FLAG_NORMAL: u8 = 0;
pub const FLAG_CAPTURE: u8 = 1;
pub const FLAG_CASTLE: u8 = 2;
pub const FLAG_EN_PASSANT: u8 = 3;

pub const PROMOTE_QUEEN: u8 = 0;
pub const PROMOTE_ROOK: u8 = 1;
pub const PROMOTE_BISHOP: u8 = 2;
pub const PROMOTE_KNIGHT: u8 = 3;

#[derive(Copy, Clone)]
pub struct Moves(u32);

impl Moves {
    pub fn new(from: u8, to: u8, promotion: u8, flags: u8, is_promotion: bool) -> Self {
        let mut m = 0u32;

        m |= (from as u32) & FROM_MASK;
        m |= ((to as u32) << 6) & TO_MASK;
        m |= ((promotion as u32) << 12) & PROMO_MASK;
        m |= ((flags as u32) << 15) & FLAGS_MASK;

        if is_promotion {
            m |= IS_PROMO_MASK;
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

    pub fn is_promotion(&self) -> bool {
        (self.0 & IS_PROMO_MASK) != 0
    }

    pub fn flags(&self) -> u8 {
        ((self.0 & FLAGS_MASK) >> 15) as u8
    }

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

    fn index_to_algebraic(&self, index: u8) -> String {
        let file = (b'a' + (index % 8)) as char;
        let rank = (b'1' + (index / 8)) as char;
        format!("{}{}", file, rank)
    }
}
