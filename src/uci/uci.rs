use crate::{
    chess::{
        moves_gen::moves_struct::{
            Moves, PROMOTE_BISHOP, PROMOTE_KNIGHT, PROMOTE_QUEEN, PROMOTE_ROOK,
        },
        table::Board,
    },
    engine::find_best_move::find_best_move,
};
use std::io::{self, BufRead, Write};

pub struct UciEngine {
    board: Board,
    debug: bool,
}

impl UciEngine {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            debug: false,
        }
    }

    pub fn run(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            if let Ok(command) = line {
                let trimmed = command.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let response = self.process_command(trimmed);
                if let Some(resp) = response {
                    writeln!(stdout, "{}", resp).unwrap();
                    stdout.flush().unwrap();
                }

                if trimmed == "quit" {
                    break;
                }
            }
        }
    }

    fn process_command(&mut self, command: &str) -> Option<String> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "uci" => Some(self.handle_uci()),
            "isready" => Some("readyok".to_string()),
            "ucinewgame" => {
                self.board = Board::new();
                None
            }
            "position" => {
                self.handle_position(&parts[1..]);
                None
            }
            "go" => Some(self.handle_go()),
            "quit" => None,
            "debug" => {
                if parts.len() > 1 {
                    self.debug = parts[1] == "on";
                }
                None
            }
            _ => None,
        }
    }

    fn handle_uci(&self) -> String {
        let mut response = String::new();
        response.push_str("id name swag chess V1.2.1\n");
        response.push_str("id author Frigge\n");
        response.push_str("uciok");
        response
    }

    fn handle_position(&mut self, args: &[&str]) {
        if args.is_empty() {
            return;
        }

        let mut move_index = 0;

        match args[0] {
            "startpos" => {
                self.board = Board::new();
                move_index = 1;
            }
            "fen" => {
                let mut fen_parts = Vec::new();
                let mut i = 1;
                while i < args.len() && args[i] != "moves" {
                    fen_parts.push(args[i]);
                    i += 1;
                }
                let fen = fen_parts.join(" ");
                if let Ok(board) = Board::new_from_fen(&fen) {
                    self.board = board;
                }
                move_index = i;
            }
            _ => return,
        }

        if move_index < args.len() && args[move_index] == "moves" {
            for &move_str in &args[move_index + 1..] {
                if let Some(mv) = self.parse_move(move_str) {
                    self.board.make_move_with_undo(&mv);
                }
            }
        }
    }

    fn parse_move(&self, move_str: &str) -> Option<Moves> {
        if move_str.len() < 4 {
            return None;
        }

        let from_str = &move_str[0..2];
        let to_str = &move_str[2..4];

        let from = algebraic_to_index(from_str)?;
        let to = algebraic_to_index(to_str)?;

        let mut promotion = 0;
        let mut is_promotion = false;

        if move_str.len() == 5 {
            is_promotion = true;
            promotion = match move_str.chars().nth(4)? {
                'q' => PROMOTE_QUEEN,
                'r' => PROMOTE_ROOK,
                'b' => PROMOTE_BISHOP,
                'n' => PROMOTE_KNIGHT,
                _ => return None,
            };
        }

        Some(Moves::new(from, to, promotion, 0, is_promotion))
    }

    fn handle_go(&mut self) -> String {
        let best_move = find_best_move(&self.board);
        format!("bestmove {}", best_move.to_string())
    }
}

fn algebraic_to_index(alg: &str) -> Option<u8> {
    if alg.len() != 2 {
        return None;
    }
    let chars: Vec<char> = alg.chars().collect();
    let file_char = chars[0];
    let rank_char = chars[1];

    let file = file_char.to_ascii_lowercase() as u8 - b'a';
    let rank = rank_char.to_digit(10)? as u8 - 1;

    if file > 7 || rank > 7 {
        return None;
    }

    Some(rank * 8 + file)
}
