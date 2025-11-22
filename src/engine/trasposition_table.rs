use core::hash;
use std::collections::HashMap;

use crate::chess::moves_gen::moves_struct::Moves;

pub struct TTEntry {
    pub hash: u64,
    pub score: i32,
    pub best_move: Moves,
    pub depth: u8,
    pub bound: BoundType,
    pub age: u8,
}

impl TTEntry {
    pub fn new(
        hash: u64,
        score: i32,
        best_move: Moves,
        depth: u8,
        bound: BoundType,
        age: u8,
    ) -> Self {
        TTEntry {
            hash,
            score,
            best_move,
            depth,
            bound,
            age,
        }
    }
}

#[derive(PartialEq)]
pub enum BoundType {
    Exact = 0,
    Lower = 1, // Fail-high
    Upper = 2, // Fail-low
}

pub struct TT {
    map: HashMap<u64, TTEntry>,
    pub age: u8,
}

impl TT {
    pub fn new(size_mb: usize) -> Self {
        let capacity = (size_mb * 1024 * 1024) / 20;
        TT {
            map: HashMap::with_capacity(capacity),
            age: 0,
        }
    }

    pub fn new_search(&mut self) {
        self.age = self.age.wrapping_add(1)
    }

    pub fn probe(&self, hash: u64) -> Option<&TTEntry> {
        self.map.get(&hash)
    }

    pub fn store(&mut self, hash: u64, entry: TTEntry) {
        self.map.insert(hash, entry);
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}
