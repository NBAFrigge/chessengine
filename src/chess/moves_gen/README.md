# Move Generation Module

This module is responsible for generating pseudo-legal moves for all piece types.

## Data Structures

* **`moves_struct.rs`**: Defines the `Moves` struct, a compact bit-packed `u32` representation of a move. It stores:
    * Source Square (6 bits)
    * Destination Square (6 bits)
    * Promotion Type (2 bits)
    * Flags (Capture, Castle, En Passant).

## Piece Logic

* **`pawn.rs`**: Handles single pushes, double pushes, captures, and en passant logic using bitwise shifts and masks (rank/file filters).
* **`knight.rs`**: Lookups based on bitwise shifts representing L-shapes.
* **`king.rs`**: Lookups for king steps and castling safety checks.
* **`bishop.rs`, `rook.rs`, `queen.rs`**: Interfaces for the Magic Bitboard lookups.

## Magic Bitboards (`magic_bitboards.rs`)
This file contains the pre-calculated magic numbers and lookup tables for sliding pieces (Rooks and Bishops). It allows the engine to query legal moves in **O(1)** time, accounting for blocking pieces (occupancy), by hashing the relevant occupancy bits into an index.

### `magic_gen/`
A standalone binary crate used to find and verify the magic numbers used in the main engine.
