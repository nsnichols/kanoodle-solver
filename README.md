# Kanoodle Solver

A [Kanoodle](https://www.educationalinsights.com/kanoodle) solver implemented in [Rust](https://www.rust-lang.org/).

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
.......................
.H.H.J.J.J.J.L.I.I.K.K.
.F.H.H.B.B.L.L.L.I.K.K.
.F.F.H.B.B.B.L.I.I.C.G.
.A.A.A.D.E.E.C.C.C.C.G.
.A.D.D.D.D.E.E.E.G.G.G.
.......................
```

The algorithm implemented here a relatively naive depth-first search for solutions. Board configurations that cannot possibly lead to a solution are pruned early, if possible (this could be smarter).

## Running

Find all possible solutions. This is slow...
```shell
$ cargo run --release -- --help
```

Finds all possible solutions starting with the specific piece and orientation and ending at the specific piece and orientation.
```shell
$ cargo run --release -- --starting-at "A[0]" --ending-at "A[1]" 
```

## Notes

This is not an example of good Rust code. It's a project I created to try out Rust.