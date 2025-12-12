use crate::{
    chess::{
        moves_gen::moves_struct::{
            FLAG_CAPTURE, FLAG_CASTLE, FLAG_EN_PASSANT, FLAG_NORMAL, Moves, PROMOTE_BISHOP,
            PROMOTE_KNIGHT, PROMOTE_QUEEN, PROMOTE_ROOK,
        },
        table::Board,
    },
    engine::find_best_move::Engine,
};
use std::io::{self, BufRead, Write};

pub struct UciEngine {
    board: Board,
    engine: Engine,
    debug: bool,
    history: Vec<u64>,
}

impl UciEngine {
    pub fn new() -> Self {
        let board = Board::new();
        Self {
            board,
            engine: Engine::new(),
            debug: false,
            history: vec![board.get_hash()],
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
                self.engine.tt.clear();
                self.history = vec![self.board.get_hash()];
                None
            }
            "position" => {
                self.handle_position(&parts[1..]);
                None
            }
            "go" => Some(self.handle_go(&parts[1..])),
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
        response.push_str("id name swag chess V1.3.5\n");
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
                self.history = vec![self.board.get_hash()];
                self.engine.tt.clear();
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
                    self.history = vec![self.board.get_hash()];
                    self.engine.tt.clear();
                }
                move_index = i;
            }
            _ => return,
        }

        if move_index < args.len() && args[move_index] == "moves" {
            for &move_str in &args[move_index + 1..] {
                if let Some(mv) = self.parse_move(move_str) {
                    self.board.make_move_with_undo(&mv);
                    self.history.push(self.board.get_hash());
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
        let mut flags = FLAG_NORMAL;

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

        let is_king_move = self.board.king.get_value() & (1u64 << from) != 0;
        if is_king_move {
            let from_to_diff = (to as i8 - from as i8).abs();
            if from_to_diff == 2 {
                flags = FLAG_CASTLE;
            }
        }

        if flags != FLAG_CASTLE {
            let target_white = self.board.white.get_value() & (1u64 << to) != 0;
            let target_black = self.board.black.get_value() & (1u64 << to) != 0;

            if (self.board.is_white_turn && target_black)
                || (!self.board.is_white_turn && target_white)
            {
                flags = FLAG_CAPTURE;
            }

            if self.board.enpassant.get_value() == (1u64 << to) {
                let is_pawn = self.board.pawn.get_value() & (1u64 << from) != 0;
                if is_pawn {
                    flags = FLAG_EN_PASSANT;
                }
            }
        }

        Some(Moves::new(from, to, promotion, flags, is_promotion))
    }

    fn handle_go(&mut self, args: &[&str]) -> String {
        let mut depth = 64;
        let mut wtime: Option<u64> = None;
        let mut btime: Option<u64> = None;
        let mut winc: Option<u64> = None;
        let mut binc: Option<u64> = None;
        let mut infinite = false;

        let mut i = 0;
        while i < args.len() {
            match args[i] {
                "depth" => {
                    if i + 1 < args.len() {
                        if let Ok(d) = args[i + 1].parse::<u8>() {
                            depth = d;
                        }
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "wtime" => {
                    if i + 1 < args.len() {
                        if let Ok(t) = args[i + 1].parse::<u64>() {
                            wtime = Some(t);
                        }
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "btime" => {
                    if i + 1 < args.len() {
                        if let Ok(t) = args[i + 1].parse::<u64>() {
                            btime = Some(t);
                        }
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "winc" => {
                    if i + 1 < args.len() {
                        if let Ok(inc) = args[i + 1].parse::<u64>() {
                            winc = Some(inc);
                        }
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "binc" => {
                    if i + 1 < args.len() {
                        if let Ok(inc) = args[i + 1].parse::<u64>() {
                            binc = Some(inc);
                        }
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "infinite" => {
                    infinite = true;
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }

        let time_limit = if infinite {
            None
        } else {
            let (my_time, my_inc) = if self.board.is_white_turn {
                (wtime, winc.unwrap_or(0))
            } else {
                (btime, binc.unwrap_or(0))
            };

            match my_time {
                Some(t) => {
                    let time_slot = (t as f64 / 20.0) + (my_inc as f64 / 2.0);

                    let max_alloc = t.saturating_sub(100).max(50);

                    Some((time_slot as u64).clamp(50, max_alloc))
                }
                None => None,
            }
        };

        let best_move = self
            .engine
            .find_best_move(&self.board, depth, &self.history, time_limit);
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
