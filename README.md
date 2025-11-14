# Chess engine rust

## Command Line Interface (CLI)

The engine supports two main commands: `perft` and `search`.

### 1. Perft (Performance Test)

Tests move generation by counting legal positions up to a given depth.

| Command | Description | Example |
| :--- | :--- | :--- |
| **`perft <depth>`** | Standard Perft (Initial FEN) | `./chessengine perft 5` |
| **`perft p <depth>`** | Perft Plus (Detailed statistics) | `./chessengine perft p 4` |
| **`perft v <depth>`** | Perft Divide (Move-by-move count) | `./chessengine perft v 3` |
| **`perft f "<FEN>" <depth>`** | Perft from specific FEN | `./chessengine perft f "r3k2r/.../1" 4` |

### 2. Search

Runs the main search algorithm against internal test positions to find the best move.

| Command | Description | Example |
| :--- | :--- | :--- |
| **`search`** | Execute search tests | `./chessengine search` |

## Performance

### Perft

Current speed 25 Mnode/sec on a i5-9400f @2.9 GHz
