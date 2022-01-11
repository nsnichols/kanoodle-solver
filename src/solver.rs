use crate::board::{create_board, BoardType, Variation};
use crate::layer::Position;
use crate::pieces::{Piece, Shape};
use crate::placements::RequestedPiece;
use crate::{placements, PieceSuggestion, Placements, PIECES};

/// Finds solutions between the initial_state and the ending_at path (exclusive)
///
/// The algorithm used here is a naive depth-first search for solutions. Starting at the
/// initial piece and orientation, all combinations of other pieces and orientations are
/// tried. The pieces and orientations are always tried in lexical order. If a piece cannot
/// be placed on the board, that tree will be abandoned.
///
/// Pieces will always be placed in the top-most, left-most available space on the board
/// (except when added as part of the initial state).
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
pub fn find_solutions(
    board_type: Option<BoardType>,
    initial_state: Option<Vec<String>>,
    allow_backtracking: Option<bool>,
    ending_at: Option<Vec<RequestedPiece>>,
) {
    let requested_board_type = board_type.unwrap_or(BoardType::Rectangle);

    println!("Finding solutions for {:?} board", requested_board_type);

    let (mut board, mut placements, mut next_piece) =
        initialize(initial_state, &requested_board_type, allow_backtracking);

    let mut solutions = 0u32;

    let ending_at_placement = convert_requested_pieces_to_path_string(ending_at);
    println!(
        "Ending at {}",
        if ending_at_placement.starts_with("Z") {
            "NO-LIMIT".to_string()
        } else {
            ending_at_placement.to_string()
        }
    );

    // Now we start the solution loop. We will continue asking for the next piece to place until we
    // find a solution that exceeds the ending_at path, or we run out of pieces to try in every
    // possible position.
    while next_piece.is_some() {
        let p = next_piece.unwrap();

        next_piece = match board.try_add_shape(p.shape, p.name) {
            Result::Ok(_) => placements
                .get_next_piece_to_try_after_success(p)
                .map(|success| success.piece),
            Result::Err(_) => get_next_piece_to_try_after_failure(&mut placements, &mut board, p),
        };

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
            println!("{}", placements);
            println!("{}", board);

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
    board: &mut Variation,
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
fn convert_requested_pieces_to_path_string(pieces: Option<Vec<RequestedPiece>>) -> String {
    pieces
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
                    if piece.orientation_index < 10 {
                        string.push('0'.to_string());
                    }
                    string.push(piece.orientation_index.to_string());
                    string.push(']'.to_string());
                    string.push("; ".to_string());
                }
                string.join("")
            }
        })
        .unwrap()
}

/// Initializes the board, the placement iterator and returns the first piece that needs
/// to be placed.
fn initialize(
    initial_state: Option<Vec<String>>,
    board_type: &BoardType,
    allow_backtracking: Option<bool>,
) -> (Variation, Placements, Option<PieceSuggestion>) {
    // Default to an empty board
    let board_state = initial_state.unwrap_or(Vec::new());

    // While placements track which pieces and orientations have been tried, the board tracks
    // where the pieces are placed, whether a piece will fit, and whether the board is in the
    // solved state.
    let mut board = create_board(board_type);

    // Placements keeps track of the available and used pieces at any given time. Pieces are always
    // ordered lexically (by name and then orientation) to ensure that once we've attempted to place
    // a piece in a specific context (previous pieces/orientation and positions), we will never try
    // that same permutation again. That means that as we ask for pieces, we'll eventually run out
    // of permutations to try and the algorithm will halt.
    let mut placements = Placements::new(*board_type == BoardType::Pyramid);

    let mut requested_pieces = Vec::new();

    // We're going to try to populate the board with shapes that match the initial board state.
    let piece_names = get_sorted_piece_names();
    for piece_name in piece_names {
        match Shape::parse(&board_state, *piece_name) {
            Option::Some(shape) => {
                let piece = get_piece(piece_name);
                // This builds a string when we might not need it, but it just happens on startup
                // for at most 12 pieces. So it really shouldn't matter.
                let expectation = format!(
                    "Unrecognized piece orientation for [{}]\n{}",
                    piece_name, shape
                );

                let orientation_index = piece
                    .orientations
                    .iter()
                    .position(|s| shape.eq(s))
                    .expect(&expectation);

                requested_pieces.push(RequestedPiece {
                    name: *piece_name,
                    orientation_index,
                });

                let shape_position = parse_shape_position(&board_state, piece_name);

                if let Result::Err(_) = board.try_add_shape_at(&shape, *piece_name, &shape_position)
                {
                    panic!("Unable to add initial piece {} to board at ({}, {}, {}). It does not fit. Initialization failed.",
                           piece.letter, shape_position.0, shape_position.1, shape_position.2);
                }
            }
            Option::None => {
                // This is fine. If the piece wasn't requested, we won't try to add it.
            }
        }
    }

    // Now that we've successfully added the shapes in the initial state to the board, we need to
    // initialize our placements iterator with those pieces.
    let suggestion = if requested_pieces.is_empty() {
        // The initial state was empty. Therefore we need to initialize with A[0].
        // There are no shapes on the board currently, so the piece that is returned
        // here can kick off our solution loop.
        placements
            .initialize(vec![RequestedPiece {
                name: 'A',
                orientation_index: 0,
            }])
            .pop()
    } else {
        // The initial state had some shapes, so push them all into the iterator.
        // We need the last initialized piece to get the _next_ piece from the iterator.
        let last_requested = placements.initialize(requested_pieces).pop().unwrap();

        // We know this piece fits (we added it to the board already). We need to get the _next_
        // piece and return it as the piece to kick off our solution loop. This gets us into an
        // equivalent state as the block above.
        placements
            .get_next_piece_to_try_after_success(last_requested)
            .map(|success| success.piece)
    };

    if !allow_backtracking.unwrap_or(false) {
        placements.prevent_backtracking_beyond_this_piece(Option::None);
    }

    println!("Initial board state");
    println!("{}", board);

    return (board, placements, suggestion);
}

fn parse_shape_position(board_state: &Vec<String>, letter: &char) -> Position {
    let layer_count = board_state.len();
    for layer_index in 0..layer_count {
        let chars = board_state.get(layer_index).unwrap().chars();
        let mut row = 0;
        let mut col = 0;

        for ch in chars {
            match ch {
                '\n' => {
                    row += 1;
                    col = 0;
                }
                c if c == *letter => {
                    return Position(layer_index, row, col);
                }
                _ => col += 1,
            }
        }
    }

    panic!(
        "Shape {} requested, but not found in initial board state",
        letter
    );
}

fn get_sorted_piece_names() -> Vec<&'static char> {
    let mut keys: Vec<&char> = (*PIECES).keys().collect();
    keys.sort();
    return keys;
}

fn get_piece(name: &char) -> &Piece {
    (*PIECES).get(name).unwrap()
}
