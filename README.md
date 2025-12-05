# Rust Chess Engine

A high-performance chess engine written in Rust, leveraging bitboards and magic bitboards for efficient move generation and search. The engine implements the Universal Chess Interface (UCI) protocol and includes comprehensive testing tools.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [UCI Mode](#uci-mode)
  - [Command-Line Tools](#command-line-tools)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Performance](#performance)
- [License](#license)

---

## Features

- **Bitboard representation** for fast board state manipulation
- **Magic bitboards** for O(1) sliding piece attack generation
- **UCI protocol** compliance for integration with chess GUIs
- **Negamax search** with alpha-beta pruning and iterative deepening
- **Transposition table** using Zobrist hashing
- **Sophisticated evaluation** including material, piece-square tables, mobility, and king safety
- **Perft testing suite** for move generator validation

---

## Installation

### Prerequisites

- Rust toolchain (latest stable version recommended)

### Building

Clone the repository and build with optimizations:

```bash
cargo build --release
```

The compiled binary will be located at `./target/release/chessengine`.

---

## Usage

### UCI Mode

Run the engine without arguments to start UCI mode for use with chess GUIs (Arena, CuteChess, BanksiaGUI, etc.):

```bash
./target/release/chessengine
```

**Supported UCI Commands:**

- `uci` — Engine identification handshake
- `isready` — Ready status check
- `ucinewgame` — Reset board and transposition tables
- `position [fen <fenstring> | startpos] moves <move1> ... <moveN>` — Set position
- `go [wtime <ms>] [btime <ms>] [depth <d>] ...` — Start search

### Command-Line Tools

The engine provides additional CLI commands for testing and debugging.

#### Perft (Move Generation Testing)

Validate move generation by counting all positions at a given depth:

```bash
# Standard perft
./chessengine perft <depth>

# Divide mode (shows move-by-move breakdown)
./chessengine perft v <depth>

# Plus mode (includes capture/castle/en-passant statistics)
./chessengine perft p <depth>

# Custom FEN position
./chessengine perft f "<FEN>" <depth>
```

#### Search Benchmarking

Run tactical position tests (mate detection, stalemate):

```bash
./chessengine search
```

---

## Architecture

The engine follows a modular design:

- **Bitboard Representation**: Uses `u64` bitboards for parallel bitwise operations
- **Magic Bitboards**: Hash-based O(1) lookup for rook and bishop attacks
- **Search Algorithm**: Negamax with alpha-beta pruning and iterative deepening
- **Transposition Table**: Zobrist hashing for position caching and transposition detection
- **Evaluation Function**: Considers material balance, piece-square tables, mobility, pawn structure, and king safety

---

## Project Structure

```
src/
├── main.rs              # Entry point
├── bitboard/            # Core bitboard data structures
├── chess/               # Board representation and game logic
│   └── moves_gen/       # Move generation (piece-specific)
├── engine/              # Search and AI
│   └── evaluate/        # Position evaluation
└── uci/                 # UCI protocol implementation
```

For detailed module documentation, see the README files in each subdirectory.

---

## Performance

**Benchmark System**: Intel Core i5-9400F @ 2.9 GHz

**Perft Performance**: ~25 million nodes/second

---

## License

This project is open-source and available under the MIT License.
