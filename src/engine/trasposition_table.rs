use crate::chess::moves_gen::moves_struct::Moves;

#[derive(Debug, Clone, Copy)]
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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BoundType {
    Exact = 0,
    Lower = 1,
    Upper = 2,
}

pub struct TT {
    table: Vec<TTEntry>,
    size: usize,
    pub age: u8,
}

impl TT {
    pub fn new(size_mb: usize) -> Self {
        let entry_size = std::mem::size_of::<TTEntry>();
        let num_entries = (size_mb * 1024 * 1024) / entry_size;

        let empty_entry = TTEntry {
            hash: 0,
            score: 0,
            best_move: Moves::new(0, 0, 0, 0, false),
            depth: 0,
            bound: BoundType::Exact,
            age: 0,
        };

        TT {
            table: vec![empty_entry; num_entries],
            size: num_entries,
            age: 0,
        }
    }

    pub fn new_search(&mut self) {
        self.age = self.age.wrapping_add(1)
    }

    pub fn probe(&self, hash: u64) -> Option<&TTEntry> {
        let index = (hash as usize) % self.size;
        let entry = &self.table[index];

        if entry.hash == hash {
            Some(entry)
        } else {
            None
        }
    }

    pub fn store(&mut self, hash: u64, entry: TTEntry) {
        let index = (hash as usize) % self.size;
        let existing = self.table[index];

        if existing.hash == 0 || existing.age != self.age || entry.depth >= existing.depth {
            self.table[index] = entry;
        }
    }

    pub fn clear(&mut self) {
        for entry in self.table.iter_mut() {
            entry.hash = 0;
            entry.depth = 0;
            entry.age = 0;
        }
    }
}

