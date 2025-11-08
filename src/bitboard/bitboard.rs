
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Bitboard(pub u64);

impl Bitboard {
    #[inline(always)]
    pub const fn new(value: u64) -> Self {
        Bitboard(value)
    }

    #[inline(always)]
    pub const fn new_from_index(value: u64) -> Self {
        Bitboard(1 << value)
    }

    #[inline(always)]
    pub const fn empty() -> Self {
        Bitboard(0)
    }

    #[inline(always)]
    pub const fn get_value(&self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub fn set_empty(&mut self) {
        self.0 = 0;
    }

    #[inline(always)]
    pub fn count_ones(&self) -> u64 {
        self.0.count_ones() as u64
    }

    #[inline(always)]
    pub fn lsb(&self) -> u64 {
        self.0 & (!self.0 + 1)
    }

    #[inline(always)]
    pub fn pop_lsb(&mut self) -> Option<u32> {
        if self.0 == 0 {
            None
        } else {
            let idx = self.0.trailing_zeros();
            self.0 &= self.0 - 1;
            Some(idx)
        }
    }

    #[inline(always)]
    pub fn and(&self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & other.0)
    }

    #[inline(always)]
    pub fn or(&self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
    }

    #[inline(always)]
    pub fn xor(&self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 ^ other.0)
    }

    #[inline(always)]
    pub fn not(&self) -> Bitboard {
        Bitboard(!self.0)
    }

    #[inline(always)]
    pub fn iter_bits(self) -> impl Iterator<Item = Bitboard> {
        let mut bb = self;
        std::iter::from_fn(move || bb.pop_lsb().map(|idx| Bitboard::new_from_index(idx as u64)))
    }


    pub fn to_string(&self) -> String { format!("{:064b}", self.0) }

    // For debugging only
    pub fn to_formatted_string(&self) -> String {
        let s = format!("{:064b}", self.0);
        s.as_bytes()
            .chunks(8)
            .rev()
            .map(|chunk| {
                let mut part = String::new();
                for &b in chunk.iter().rev() {
                    part.push(b as char);
                }
                part.push('\n');
                part
            })
            .collect()
    }
}
