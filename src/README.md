# Source Directory

This directory contains the main entry point of the application and the high-level module definitions.

## Files

* **`main.rs`**: The application entry point. It handles command-line argument parsing to switch between UCI mode, Perft testing, and internal Search testing. It initializes global static resources (such as Magic Bitboards) before execution.
* **`util.txt`**: Contains helper data, ASCII representations of bitboards, and magic number constants used for debugging and development reference.

## Modules

* **`bitboard`**: Defines the fundamental `Bitboard` data type used throughout the engine.
* **`chess`**: Encapsulates game rules, board state, and move generation logic.
* **`engine`**: Contains the AI logic, including search algorithms and evaluation functions.
* **`uci`**: Handles input/output for the Universal Chess Interface.
