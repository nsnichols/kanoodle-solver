use crate::placements::RequestedPiece;
use crate::{placements, Board, PieceSuggestion, Placements};

/// Finds solutions between the starting_at piece (inclusive) and the ending_at path (exclusive)
///
/// The algorithm used here is a naive depth-first search for solutions. Starting at the
/// initial piece and orientation, all combinations of other pieces and orientations are
/// tried. The pieces and orientations are always tried in lexical order. If a piece cannot
/// be placed on the board, that tree will be abandoned.
///
/// Pieces will always be placed in the top-most, left-most available space on the board.
///
/// A single iteration of placing a piece looks like the following:
/// 1. Attempt to place piece 'X' in orientation [n] into next board position
/// 2. If the piece fits:
///   1. If the board is solved:
///     1. If the solution falls outside the ending_at path, we're done. exit.
///     2. Otherwise, Print out the solution
///     2. Remove the last piece and pretend as if the last piece failed to be placed
///   2. If the board is not solved:
///     1. Get the next piece to try to place and goto 1.
/// 3. If the piece does not fit:
///   1. Ask for a new piece to try (this new piece and orientation will always be 'after'
///      the failed piece).
///   2. If all possible pieces have already been tried at this position, one or more pieces
///      will be removed until we find another piece that can be tried.
///     2. Goto with the new piece to try.
///   3. Otherwise, goto 1 with the new piece to try.
///
/// # Arguments
/// * `starting_at` - if not passed, `A[0]` will be used.
/// * `ending_at` - if not passed, a fake path will be created using a piece `Z`. This effectively
///   allows all solutions after starting_at to be returned.
pub fn find_solutions(starting_at: Option<RequestedPiece>, ending_at: Option<Vec<RequestedPiece>>) {
    println!("Finding solutions...");

    // Placements keeps track of the available and used pieces at any given time. Pieces are always
    // ordered lexically (by name and then orientation) to ensure that once we've attempted to place
    // a piece in a specific context (previous pieces/orientation and positions), we will never try
    // that same permutation again. That means that as we ask for pieces, we'll eventually run out
    // of permutations to try and the algorithm will halt.
    let mut placements = Placements::new();

    // While placements track which pieces and orientations have been tried, the board tracks
    // where the pieces are placed, whether a piece will fit, and whether the board is in the
    // solved state.
    let mut board = Board::new();

    let ending_at_placement = convert_ending_at_to_placement_string(ending_at);

    let mut solutions = 0u32;

    // We always start with an empty board, but the caller can specify which should be the first piece
    // and orientation they want to try to place. All pieces that come before this piece lexically will
    // be skipped and never considered for placement in the first slot. For example: If someone requests
    // to start with K[0], the only solutions that can possibly be found will container either K[0] or
    // L[0] in the board's first position. (Note: given L[0]'s shape, no solutions with it in the first
    // position are possible).
    let mut next_piece = placements.initialize(starting_at.unwrap_or(RequestedPiece {
        name: 'A',
        orientation_index: 0,
    }));

    // Now we start the solution loop. We will continue asking for the next piece to place until we
    // find a solution that exceeds the ending_at path, or we run out of pieces to try in every
    // possible position.
    while next_piece.is_some() {
        let p = next_piece.unwrap();

        if board.does_shape_fit(p.shape) {
            board.add_shape(p.shape, p.name);
            next_piece = placements
                .get_next_piece_to_try_after_success(p)
                .map(|success| success.piece);
        } else {
            next_piece = get_next_piece_to_try_after_failure(&mut placements, &mut board, p);
        }

        // If we have no pieces remaining to try then we can also check if the board is solved
        // We avoid checking that if we still have pieces since it cannot be solved in that case,
        // and it's just doing extra work.
        if next_piece.is_none() && board.solved() {
            if !placements.to_string().le(&ending_at_placement) {
                // Additionally, we want to make sure that if we have a solution we only report it
                // and continue finding more solutions, if our placement path is lexically less than
                // the specified ending path. Note: this check being placed here means that we will
                // always find one extra solution (and not report it) before exiting, but that is
                // more convenient than making the caller define an exact path to stop at. (If they
                // knew the exact path, they'd already know all the solutions and wouldn't need to
                // call the function to begin with).
                break;
            }

            solutions += 1;
            println!("{}", board);
            println!("{}", placements);

            // We found a solution, but there could be more. We'll remove the last piece we placed,
            // pretend like it failed placement in the board, and try the next piece. If there are
            // more solutions, this is enough to kick off the process again while ensuring we don't
            // re-find any solutions we've already discovered.
            next_piece = match placements.remove_last_piece() {
                Option::Some(last) => {
                    board.remove_shape(last.name);
                    get_next_piece_to_try_after_failure(&mut placements, &mut board, last)
                }
                _ => Option::None,
            }
        }
    }

    println!("found {} solutions", solutions);
}

/// Wraps `Placements`' `get_next_piece_to_try_after_failure` method to add logic to remove
/// any necessary pieces from the `Board`.
fn get_next_piece_to_try_after_failure(
    placements: &mut Placements,
    board: &mut Board,
    failed_piece: PieceSuggestion,
) -> Option<PieceSuggestion> {
    match placements.get_next_piece_to_try_after_failure(failed_piece) {
        Option::Some(placements::PieceAfterFailure { piece, to_remove }) => {
            // There was a failure. Make sure the board reverts any necessary piece placements
            for piece_to_remove in to_remove {
                board.remove_shape(piece_to_remove.name);
            }
            Option::Some(piece)
        }
        _ => Option::None,
    }
}

/// Transforms a possible vector of `RequestedPiece`s into  a `String` that can be compared with a
/// `Placements` path string.
fn convert_ending_at_to_placement_string(ending_at: Option<Vec<RequestedPiece>>) -> String {
    ending_at
        .or(Option::Some(Vec::new()))
        .map(|pieces| {
            if pieces.is_empty() {
                // A placement string looks something like "A[0]; B[0]; etc..."
                // If the vector is empty, that means all solutions greater than starting_at,
                // are desired. The last piece is 'L' currently. That means that a string
                // starting with 'Z' (or even M) will always come lexically after any valid
                // placement path.
                "Z-NO-LIMIT".to_string()
            } else {
                let mut string = Vec::new();
                for piece in pieces {
                    string.push(piece.name.to_string());
                    string.push('['.to_string());
                    string.push(piece.orientation_index.to_string());
                    string.push(']'.to_string());
                    string.push("; ".to_string());
                }
                string.join("")
            }
        })
        .unwrap()
}
