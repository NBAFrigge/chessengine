# Engine Core

This module contains the artificial intelligence algorithms used to determine the best move.

## Search Strategy (`search.rs` & `find_best_move.rs`)

* **Negamax Algorithm**: A simplified minimax implementation suitable for zero-sum games.
* **Alpha-Beta Pruning**: Reduces the search space by pruning irrelevant branches.
* **Iterative Deepening**: Performs searches at depth 1, then 2, etc., allowing for effective time management and better move ordering.
* **Time Management**: Checks elapsed time during search to strictly adhere to time controls.

## Quiescence Search (`quiescence.rs`)
A specialized search executed at leaf nodes (horizon) to avoid the "horizon effect." It continues searching capturing moves until a "quiet" position is reached to ensure the static evaluation is accurate.

## Transposition Table (`transposition_table.rs`)
A fixed-size hash map (`TT`) that stores the results of previously searched positions to avoid redundant work.
* **Entries**: Stores Evaluation score, Best move, Depth, and Bound type (Exact, Lower, Upper).
* **Replacement Strategy**: Prefers storing results from deeper searches.

## Performance Testing (`perft.rs`)
Contains functions to validate the move generator and measure raw node throughput (Perft, Perft Divide).

## Submodules
* **`evaluate/`**: Contains the static evaluation logic. See [Evaluation README](evaluate/README.md).
