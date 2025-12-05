# Bitboard Module

This module defines the foundational data structure for the engine: the `Bitboard`.

## Structure

* **`bitboard.rs`**: Defines the `Bitboard` struct, which is a transparent wrapper around a native `u64`.

## Features

The `Bitboard` struct implements optimized bitwise operations crucial for performance:
* **Accessors**: `set_bit`, `count_ones`, `lsb` (Least Significant Bit), and `pop_lsb` for efficient iteration.
* **Logic**: `and`, `or`, `xor`, and `not` operations for masking and combining board states.
* **Iteration**: Implements custom iterators to loop over occupied squares efficiently.
* **Formatting**: Includes helper methods to print bitboards as 8x8 grids for debugging purposes.
