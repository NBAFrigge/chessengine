# UCI Protocol Module

This module handles the communication between the engine core and external applications (Chess GUIs) via the **Universal Chess Interface (UCI)** standard.

## Implementation (`uci.rs`)

The `UciEngine` struct runs the main input loop, reading standard input line-by-line.

### Key Responsibilities

1. **Command Parsing**: Interprets standard UCI commands (`uci`, `isready`, `position`, `go`).
2. **Move Parsing**: Converts algebraic notation (e.g., "e2e4") into the engine's internal move format, handling promotions and castling context.
3. **State Management**: Maintains the internal board state and history based on `position` commands.
4. **Time Management**: Interprets `wtime`, `btime`, and `movestogo` to allocate appropriate time for the search.
5. **Output**: Sends `bestmove` commands to standard output when the search concludes.
