use std::string;

pub struct bitboard {
    board: u64
}

impl bitboard{
    pub fn new(value :u64) -> Self {
        bitboard {
            board: value
        }
    }

    pub fn empty(&self) -> Self {
        bitboard {
            board: 0
        }
    }

    pub fn lsb(&self) -> u64 {
        let index = self.board.trailing_zeros() as u64;
        1 << index
    }

    pub fn to_string(&self) -> String {
        let string = format!("{:064b}", self.board);
        
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

    pub fn get_value(&self) -> u64 {
        self.board
    }
}