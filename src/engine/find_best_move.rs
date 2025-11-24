use crate::{
    chess::{moves_gen::moves_struct::Moves, table::Board},
    engine::{search::negamax, trasposition_table::TT},
};

const DEPTH: u8 = 6;
const MAX_MOVES: usize = 255;
const INFINITY: i32 = 30000;

pub struct Engine {
    tt: TT,
}

impl Engine {
    pub fn new() -> Self {
        Engine { tt: TT::new(256) }
    }

    pub fn find_best_move(&mut self, b: &Board) -> Moves {
        let mut board_mut = *b;
        let mut move_buffers: Vec<Vec<Moves>> =
            (0..=DEPTH).map(|_| Vec::with_capacity(MAX_MOVES)).collect();

        let (root_move_buffer, next_buffers) = move_buffers.split_at_mut(1);
        let root_move_vec = &mut root_move_buffer[0];
        let turn = board_mut.get_side();
        let moves = board_mut.get_legal_moves(turn, root_move_vec);

        if moves.is_empty() {
            return Moves::new(0, 0, 0, 0, false);
        }

        let mut best_move = moves[0];
        let mut alpha = -INFINITY;
        let beta = INFINITY;

        let mut position_history = [0u64; 32];
        let mut history_len = 0;

        position_history[history_len] = board_mut.get_hash();
        history_len += 1;

        for mv in moves.iter() {
            let undo_info = board_mut.make_move_with_undo(mv);

            let score = -negamax(
                &mut board_mut,
                DEPTH - 1,
                -beta,
                -alpha,
                &mut self.tt,
                next_buffers,
                &mut position_history,
                history_len,
            );

            board_mut.unmake_move(mv, undo_info);

            if score > alpha {
                alpha = score;
                best_move = *mv;
            }
        }

        best_move
    }

    pub fn clear(&mut self) {
        self.tt.clear();
    }
}
