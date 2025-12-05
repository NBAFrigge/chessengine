# Evaluation Module

This module acts as the engine's "intuition," assigning a numerical score to a board position.

## Evaluation Terms (`evaluate.rs`)

The final score is a linear combination of several factors:

1.  **Material**: Base value of pieces (Pawn=100, Knight=320, Bishop=330, Rook=500, Queen=900).
2.  **Piece-Square Tables (`pst.rs`)**: Encourages pieces to move to advantageous squares (e.g., Knights to the center, Kings to safety) depending on the game phase.
3.  **Mobility (`mobility.rs`)**: Rewards positions where pieces have more legal moves available.
4.  **King Safety (`king_safety.rs`)**: Penalizes open files near the king and rewards pawn shields.
5.  **Pawn Structure (`pawn_evaluation.rs`)**: Penalizes doubled/isolated pawns and rewards passed pawns.
6.  **Endgame Knowledge (`endgame.rs`)**: Specific logic for endgame phases, such as pushing the enemy king to the edge or incentivizing king activity.
7.  **Bishop Pair (`bishop_pair.rs`)**: Applies a bonus for retaining both bishops.

## Tapered Evaluation
The engine calculates a "Game Phase" value based on the remaining material. The evaluation smoothly interpolates between Middlegame and Endgame scores (particularly for King PSTs) based on this phase.
