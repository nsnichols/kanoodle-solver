# Kanoodle Solver

A [Kanoodle](https://www.educationalinsights.com/kanoodle) solver implemented in [Rust](https://www.rust-lang.org/).

### Rectangle Board

There are 12 Kanoodle pieces (`A` - `L`) that may be placed in various orientations on a `5 x 11` board.

```
A   BB  C   D   E   FF  G    H    II  J  KK   L
A   BB  C   DD  EE  F   G    HH   I   J  KK  LLL
AA  B   C   D    E      GGG   HH  II  J       L
        CC  D    E                    J
        
8   8   8   8   8   4   4    4    4   2  1   1  (orientations)
```

This means there are a _lot_ of permutations to check...

When all pieces have been successfully placed (in any orientations) that is a solution. There are many solutions.

Example solution:
```
·······················
·H·H·J·J·J·J·L·I·I·K·K·
·F·H·H·B·B·L·L·L·I·K·K·
·F·F·H·B·B·B·L·I·I·C·G·
·A·A·A·D·E·E·C·C·C·C·G·
·A·D·D·D·D·E·E·E·G·G·G·
·······················
```

### Pyramid Board

The same 12 Kanoodle pieces (`A` - `L`) are arranged in such a way that they form a pyramid. The pieces may be placed in many more orientations.

The pyramid has `5` layers with the following dimensions: `5 x 5`, `4 x 4`, `3 x 3`, `2 x 2`, and `1 x 1`.

Example solution (each layer is viewed from above and sits on the layer below it):
```
    ···
    ·I·
   ·····
   ·I·H·
   ·H·I·
  ·······
  ·A·L·H·
  ·E·H·L·
  ·H·E·I·
 ·········
 ·A·G·G·G·
 ·E·A·L·G·
 ·C·E·I·G·
 ·C·C·C·C·
··········· 
·A·D·D·D·D·
·E·F·L·D·J·
·B·F·F·L·J·
·B·B·K·K·J·
·B·B·K·K·J·
···········
```



Thealgorithm implemented here a relatively naive depth-first search for solutions. Board configurations that cannot possibly lead to a solution are pruned early, if possible (this could be smarter).

## Running

Find all possible solutions. This is slow...
```shell
$ cargo run --release -- --help
```

* Finds all possible solutions starting with a specific board state.
```shell
$ cargo run --release -- < board-state.txt
```
*Note: The file should contain a valid board with pieces defined using their letter names. New lines indicate new rows. Any non piece name character can be used (including a space). Empty rows and columns do not need to be included in the state, unless they provide spacing between actual pieces.*
  ```text
  AAABBB.....
  A...BB.....
  ...........
  ..........F
  .........FF
  ```

* Finds solutions for the pyramid board
```shell
$ cargo run --release -- --board-type pyramid
```
*Note: When initializing a pyramid board, new lines still separate rows, but empty lines indicate layer breaks. Additionally, layers are specified in biggest to smallest (backwards from how the pyramid actually sits).*

```text
ABB..
BBB..
.....
.....
.....

A...
.A..
....
....

A..
...
...
```



## Notes

This is not an example of good Rust code. It's a beginner flailing around trying to figure things out.