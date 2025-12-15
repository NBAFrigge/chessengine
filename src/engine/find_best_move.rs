use std::time::{Duration, Instant};

use crate::{
    chess::{moves_gen::moves_struct::Moves, table::Board},
    engine::{evaluate::evaluate::calculate_game_phase, search::negamax, trasposition_table::TT},
};

const MAX_MOVES: usize = 255;
const INFINITY: i32 = 30000;
const MAX_PLY: usize = 64;

pub struct Engine {
    pub tt: TT,
}

impl Engine {
    pub fn new() -> Self {
        Engine { tt: TT::new(256) }
    }

    pub fn find_best_move(
        &mut self,
        b: &Board,
        max_depth: u8,
        game_history: &[u64],
        time_limit: Option<u64>,
    ) -> Moves {
        let mut board_mut = *b;
        let start_time = Instant::now();
        let mut killer_moves = [[Moves::new(0, 0, 0, 0, false); 2]; MAX_PLY];
        let mut history = [[0i32; 64]; 64];

        let limit_duration = time_limit.map(|ms| Duration::from_millis(ms));

        let mut move_buffers: Vec<Vec<Moves>> = (0..=max_depth + 1)
            .map(|_| Vec::with_capacity(MAX_MOVES))
            .collect();

        let (root_move_buffer, _) = move_buffers.split_at_mut(1);
        let root_move_vec = &mut root_move_buffer[0];
        let turn = board_mut.get_side();

        board_mut.get_legal_moves(turn, root_move_vec);
        let moves = root_move_vec; // Ora `moves` è di tipo &mut Vec<Moves>

        if moves.is_empty() {
            return Moves::new(0, 0, 0, 0, false);
        }

        let mut global_best_move = moves[0];

        let mut search_history = game_history.to_vec();
        search_history.push(board_mut.get_hash());

        for current_depth in 1..=max_depth {
            if current_depth > 1 {
                if let Some(idx) = moves.iter().position(|&m| m == global_best_move) {
                    moves.swap(0, idx);
                }
            }

            let mut best_move_this_depth = global_best_move;
            let mut alpha = -INFINITY;
            let beta = INFINITY;
            let mut best_score = -INFINITY;

            let mut depth_completed = true;

            let mut iter_buffers: Vec<Vec<Moves>> = (0..=current_depth + 1)
                .map(|_| Vec::with_capacity(MAX_MOVES))
                .collect();
            let (_, next_buffers) = iter_buffers.split_at_mut(1);

            for mv in moves.iter() {
                if let Some(limit) = limit_duration {
                    if start_time.elapsed() > limit {
                        depth_completed = false;
                        break;
                    }
                }

                let undo_info = board_mut.make_move_with_undo(mv);
                search_history.push(board_mut.get_hash());

                let score = -negamax(
                    &mut board_mut,
                    current_depth - 1,
                    -beta,
                    -alpha,
                    &mut self.tt,
                    next_buffers,
                    &mut search_history,
                    &mut killer_moves,
                    &mut history,
                    1,
                );

                search_history.pop();
                board_mut.unmake_move(mv, undo_info);

                if score > best_score {
                    best_score = score;
                    best_move_this_depth = *mv;
                }

                if score > alpha {
                    alpha = score;
                }
            }

            if depth_completed {
                global_best_move = best_move_this_depth;
            } else {
                break;
            }

            if let Some(limit) = limit_duration {
                if start_time.elapsed() > limit / 2 {
                    break;
                }
            }
        }

        global_best_move
    }
    pub fn clear(&mut self) {
        self.tt.clear();
    }
}
