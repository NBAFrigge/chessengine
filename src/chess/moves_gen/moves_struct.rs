// Bit 0-5:  old pos
// Bit 6-11:  new pos
// Bit 12-13: prmotion flag
// Bit 14-15: specil flag (2 bit)

const FROM_MASK: u16 = 0x003F; // 0000000000111111
const TO_MASK: u16 = 0x0FC0; // 0000111111000000
const PROMOTION_MASK: u16 = 0x3000; // 0011000000000000
const FLAGS_MASK: u16 = 0xC000; // 1100000000000000

pub const FLAG_NORMAL: u8 = 0; // 00
pub const FLAG_CAPTURE: u8 = 1; // 01
pub const FLAG_CASTLE: u8 = 2; // 10
pub const FLAG_EN_PASSANT: u8 = 3; // 11

pub const PROMOTE_QUEEN: u8 = 0; // 00
pub const PROMOTE_ROOK: u8 = 1; // 01
pub const PROMOTE_BISHOP: u8 = 2; // 10
pub const PROMOTE_KNIGHT: u8 = 3; // 11

#[derive(Copy, Clone)]
pub struct Moves(u16);

impl Moves {
    pub fn new(from: u8, to: u8, promotion: u8, flags: u8) -> Self {
        let mut move_data = 0u16;
        move_data |= (from as u16) & FROM_MASK;
        move_data |= ((to as u16) << 6) & TO_MASK;
        move_data |= ((promotion as u16) << 12) & PROMOTION_MASK;
        move_data |= ((flags as u16) << 14) & FLAGS_MASK;
        Moves(move_data)
    }

    pub fn from(&self) -> u8 {
        (self.0 & FROM_MASK) as u8
    }

    pub fn to(&self) -> u8 {
        ((self.0 & TO_MASK) >> 6) as u8
    }

    pub fn promotion(&self) -> u8 {
        ((self.0 & PROMOTION_MASK) >> 12) as u8
    }

    pub fn flags(&self) -> u8 {
        ((self.0 & FLAGS_MASK) >> 14) as u8
    }
}
