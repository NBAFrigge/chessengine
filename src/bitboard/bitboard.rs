use std::string;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Bitboard {
    pub board: u64,
}

impl Bitboard {
    pub fn new(value: u64) -> Self {
        Bitboard { board: value }
    }

    pub fn new_from_index(value: u64) -> Self {
        Bitboard { board: 1 << value }
    }

    pub fn empty() -> Self {
        Bitboard { board: 0 }
    }

    pub fn lsb(&self) -> u64 {
        let index = self.board.trailing_zeros() as u64;
        1 << index
    }

    pub fn count_ones(&self) -> u64 {
        self.board.count_ones() as u64
    }

    pub fn to_string(&self) -> String {
        format!("{:064b}", self.board)
    }

    pub fn to_formatted_string(&self) -> String {
        let string = format!("{:064b}", self.board);

        let s = string.chars().collect::<Vec<char>>();

        let mut bitboard = string::String::from("");

        for chunk in s.chunks(8) {
            let mut s = chunk.iter().collect::<String>();
            s = s.chars().rev().collect::<String>();
            s.push_str("\n");
            bitboard.push_str(s.as_str());
        }

        bitboard.to_string()
    }

    pub fn get_value(&self) -> u64 {
        self.board
    }

    pub fn clone(&self) -> Bitboard {
        Bitboard::new(self.get_value())
    }

    pub fn and(&self, other: Bitboard) -> Bitboard {
        Bitboard::new(self.board & other.board)
    }

    pub fn or(&self, other: Bitboard) -> Bitboard {
        Bitboard::new(self.board | other.board)
    }

    pub fn not(&self) -> Bitboard {
        Bitboard::new(!self.get_value())
    }

    pub fn xor(&self, other: Bitboard) -> Bitboard {
        Bitboard::new(self.board ^ other.get_value())
    }

    pub fn subtract(&self, other: Option<&Bitboard>) -> Bitboard {
        Bitboard::new(self.board - other.unwrap().board)
    }

    pub fn get_single_ones(&self) -> Vec<Bitboard> {
        let mut v = Vec::new();
        let mut temp = self.clone();
        while temp.count_ones() > 0 {
            v.push(Bitboard::new(temp.lsb()));
            temp = temp.subtract(v.get(v.len() - 1))
        }

        v
    }
}

