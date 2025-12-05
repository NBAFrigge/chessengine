# Chess Logic Module

This module manages the state of the game, the board representation, and the rules of chess.

## Components

### `table.rs` (Board Representation)
Defines the `Board` struct, which aggregates:
* **Piece Bitboards**: Tracks locations of Pawns, Knights, Bishops, Rooks, Queens, and Kings.
* **Color Bitboards**: Tracks all White and Black pieces.
* **Game State**: Manages Castling rights, En Passant targets, side to move, and the Zobrist hash.
* **Core Methods**:
    * `make_move_with_undo` / `unmake_move`: Incremental state updates for search traversal.
    * `get_legal_moves`: Generates pseudo-legal moves and filters them against check constraints.
    * `new_from_fen`: Parses FEN strings to initialize board states.

### `zobrist.rs` (Hashing)
Implements **Zobrist Hashing** to generate a unique 64-bit signature for any board position.
* **Incremental Updates**: The hash is updated efficiently during move making/unmaking rather than recomputed from scratch.
* **Coverage**: Hashes include piece positions, castling rights, en passant files, and the side to move.

### `moves_gen/`
A submodule dedicated to the specific mechanics of generating moves for each piece type. See the [Move Generation README](moves_gen/README.md) for details.
