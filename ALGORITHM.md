# Algorithm

There are 12 shapes (`A` - `L`), each of which have at least 1 (often many more) orientations.

Example: `A`

Flat Orientations (viewed from above):
```
AAA | AAA | A   | AAA
A   |   A | AAA | A
---------------------
AA  | AA  | A   |  A
A   |  A  | A   |  A
A   |  A  | AA  | AA
```
3D Orientations (viewed from the side):
```
A    |   A  |  A   |  A  
 A   |  A   |   A  | A   
  A  | A    |  A   |  A  
 A   |  A   | A    |   A 
-------------------------
A    |    A |   A  |  A 
 A A | A A  |  A A | A A  
  A  |  A   | A    |    A
```

Example: `K`

Flat Orientations (viewed from above):
```
KK
KK
```
3D Orientations (viewed from the side):
```
 K
K K
 K
```

Note: there are twice as many 3D orientations as displayed above since on the pyramid board each orientation works at two different angles. This is hard to represent in text.

## Version 1

All possible shapes (piece + orientation permutations) are tried in order until the board is in an invalid state or valid solution.

When a solution is found (all the pieces are on the board), the solution is recorded, the last piece to have been placed is removed, and the algorithm is started again, using that state. From there it looks for another solution.

When a shape does not fit on the board, we try the next orientation for the shape's piece or try the next piece. If all possible unplaced pieces have been tried in all their orientations and the board is not solved, the last successfully placed piece is removed, and we start searching for solutions using the next orientation of the removed piece or the next piece if there are no remaining untried orientations.

### Implementation details

The iteration order for shapes is predictable. Once a shape has been tried in a specific position in the iteration, it will never be tried again.

The order looks like this:

`A[0], A[1], ..., A[n], B[0], B[1] ... B[n], ..., K[0], L[0]`

The iterator tracks which shapes have been successfully placed and when asking for the next shape, you must always provide the previous shape. In this way, the iterator can be sure to not keep retrying the same pieces even though it does not store every permutation it has tried.

#### Example

*Starting State*

```
B[2]; D[3]; H[0]; A[4]; C[1]; F[0]; G[0]; K[0]

E[0] was suggested but has not been committed
```

`E[0]` is successfully placed and the next piece is requested from the iterator. The iterator is notified that `E[0]` fit on the board.

```
B[2]; D[3]; H[0]; A[4]; C[1]; F[0]; G[0]; K[0]; E[0]

I[0] is suggested
```

(`I[0]` is the suggestion because it is the next available piece and orientation that is not already on the board/saved in the iterator.)

`I[0]` does not fit. The next piece is requested, and it is notified that `I[0]` does not fit.

```
B[2]; D[3]; H[0]; A[4]; C[1]; F[0]; G[0]; K[0]; E[0]

I[1] is suggested
```

(`I[1]` is the suggestion because it is the next available orientation for `I` and `I` is the next piece not already on the board/saved in the iterator.)

`I[1]` fits on the board. The next piece is requested, and it is notified that `I[1]` fits.

```
B[2]; D[3]; H[0]; A[4]; C[1]; F[0]; G[0]; K[0]; E[0]; I[1]

J[0] is suggested
```
Assuming the board is now in an unsolvable state, As we request pieces from the iterator, it will advance through all the `J` orientations and then try the `K` orientations as well. Those are the only pieces available to try.

Once all the pieces have been exhausted, the iterator will pop the last successful piece off and return a different piece to try in that spot.

```
B[2]; D[3]; H[0]; A[4]; C[1]; F[0]; G[0]; K[0]; E[0];

I[2] is now suggested
```

Assuming the board is still in an unsolvable state, as we request pieces from the iterator, it will now advance through the remaining `I` orientations and then `J` and `K` again.

*Note: We retry `J` and `K` pieces here because even though we tried them previously, we tried them in a _different_ position.*

In this way, the iterator can advance and backtrack, but it only tries pieces and orientations that it has not already tried in the position.

Suppose we have the following iterator state:

```
K[0]
```

If we've run out of options for the next piece and `K[0]` is popped, the only possible next suggestion is `L[0]` (`K` has a single  non-3D orientation). Pieces `A` - `J` will not be suggested because the iterator knows they've already been tried at the 0th position. (The only way we could have gotten to `K` is by already trying them).

## Possible Improvements

* Run the solver in parallel. The `Placements` iterator and the `Board` support being initialized in a specific state and the solver can run just to a specific state, so this would be a quick win. We could run a separate solver for each shape in the 0th position. 
* Right now all permutations are checked separately. However, when a single solution is found, we've actually found multiple solutions.
  * On the Rectangular board, a single solution is actually possibly 8 different solutions:
    * The discovered solution
    * That solution mirrored vertically,
    * That vertically mirrored solution mirrored horizontally,
    * That horizontally mirrored solution mirrored vertically again,
    * The solution mirrored diagonally
    * The same set of horizontal and diagonal mirroring
    So for the 0th piece we can find all possible solutions for every piece and just do all the transformations to get the full set of solutions.
  * On the Pyramid board, we can simply find solutions for one set of orientations and then rotate the solution to find the solutions for 3 other orientations. I'm not sure if mirroring will work.
* Many solutions contain smaller shapes that could be used to reduce the number of solutions that need to be searched for.
  * Example: `K` and `G` can be combined into a 3x3 square. Any solutions that contain that square can be mirrored and derived automatically once we find them for one orientation.