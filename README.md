one-neighbor-binary-cellular-automata
=====================================

One Neighbor Binary Cellular Automata (1nCA) is a one-dimensional Cellular Au-tomata,
with two possible states per cell.

Each cell has two neighbors, left and right, defined to be the adjacent cells on either side,
but the update rule consider only one neighbor per step.

The neighborhood includes the cell itself and the left or the right adjacent cell and alternates
between these two situations at even and odd time steps. The size of the neighborhood is always 2,
so there are 4 possible patterns for the neighborhood and only 16 possible rules.
The number of possible rules is small com- pared to the 256 possible rules of the Elementary Cellular Automata,
so it is easier to exhaustively study the dynamic behavior of the all rules.
