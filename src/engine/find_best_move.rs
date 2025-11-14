use crate::{
    chess::{
        moves_gen::moves_struct::{self, Moves},
        table::Board,
    },
    engine::search::negamax,
};

const DEPTH: u8 = 5;
const MAX_MOVES: usize = 255;

pub fn find_best_move(b: &Board) -> Moves {
    let mut board_mut = *b;

    let mut move_buffers: Vec<Vec<Moves>> =
        (0..=DEPTH).map(|_| Vec::with_capacity(MAX_MOVES)).collect();

    let mut max_score = i32::MIN;
    let mut best_mv = Moves::new(0, 0, 0, 0, false);
    let (root_move_buffer, next_buffers) = move_buffers.split_at_mut(1);

    let root_move_vec = &mut root_move_buffer[0];

    let turn = board_mut.get_side();
    let moves = board_mut.get_legal_moves(turn, root_move_vec);

    for mv in moves.iter() {
        let undo_info = board_mut.make_move_with_undo(mv);

        let score = -negamax(&mut board_mut, DEPTH - 1, next_buffers);

        board_mut.unmake_move(mv, undo_info);

        if score > max_score {
            max_score = score;
            best_mv = *mv;
        }
    }

    best_mv
}
