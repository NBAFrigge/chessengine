use crate::chess::table::Board;
use crate::chess::table::Color;
use std::rc::Rc;
use crate::engine::perft::tree::Node;

#[derive(Clone)]
pub struct Perft {
    tree: Rc<Node<Board>>,
}

impl Perft {
    pub fn new(start: Board, depth: u32) -> Perft {
        Perft {
            tree: Node::new(start),
        }
    }

    pub fn perft(&self, start : Node<Board>, depth : u32) -> Option<Perft> {
        if depth == 0 {
            return None;
        }
        let color =  if (start.value().is_white_turn) {Color::White} else {Color::Black};
        let moves = start.value().get_legal_moves(color);
        let mut board = start.value();
        for p in moves {
            for m in p.new_pos.get_single_ones() {
                let node = Node::new(board.perform_move(p.old_pos, m));
                node.attach(*start);
            }
        }

        Some(self.Clone())
    }
}