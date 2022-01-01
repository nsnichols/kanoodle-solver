use crate::pieces::{Shape, PIECES};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// A suggested piece to be placed or removed from the board.
/// There is no information about where on the board the piece
/// should be placed and a suggested piece is not guaranteed
/// to be placeable.
///
/// The wrapped `Shape` is in the orientation in which the piece
/// is expected to be placed.
#[derive(Clone)]
pub struct PieceSuggestion {
    pub name: char,
    pub shape: &'static Shape,
    orientation_index: usize,
}

pub struct PieceAfterSuccess {
    pub piece: PieceSuggestion,
}

pub struct PieceAfterFailure {
    pub piece: PieceSuggestion,
    /// If this vector contains pieces, they must be removed from the board
    /// before attempting to place the suggested piece.
    pub to_remove: Vec<PieceSuggestion>,
}

/// Keeps track of which pieces are currently placed, and provides suggestions
/// for the next piece to be placed, given the current state.
///
/// This struct ensures that once a suggestion has been made for a particular context
/// (all pieces placed previous to the suggestion) that suggestion will never be made
/// again.
///
/// Suggested pieces are returned in ascending order by piece name and orientation.
pub struct Placements {
    positions: Vec<PieceSuggestion>,
    used_letters: HashSet<char>,
}

#[derive(Debug)]
pub struct RequestedPiece {
    pub name: char,
    pub orientation_index: usize,
}

// Needed for StructOpt
impl FromStr for RequestedPiece {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = &mut s.chars();
        let name = chars.nth(0).unwrap();
        let orientation_index = chars.nth(1).unwrap().to_digit(10).unwrap() as usize;

        return Result::Ok(RequestedPiece {
            name,
            orientation_index,
        });
    }
}

impl Display for Placements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for position in &self.positions {
            write!(f, "{}[{}]; ", position.name, position.orientation_index)?;
        }
        Result::Ok(())
    }
}

impl Placements {
    pub fn new() -> Placements {
        Placements {
            positions: Vec::new(),
            used_letters: HashSet::new(),
        }
    }

    /// Initializes the state and provides the initial piece suggestion based on the
    /// `start_at` piece.
    pub fn initialize(&mut self, start_at: RequestedPiece) -> Option<PieceSuggestion> {
        Option::Some(PieceSuggestion {
            name: start_at.name,
            orientation_index: start_at.orientation_index,
            shape: get_piece_orientation(start_at.name, start_at.orientation_index).unwrap(),
        })
    }

    /// Removes the last placed piece (if there was one) and returns it.
    ///
    /// Normally, if all 12 pieces are in use, that means a particular solution has been found,
    /// but that doesn't mean that all possible solutions have been found.
    ///
    /// This allows the caller to remove the last piece in order to get a new suggestion for the
    /// same position. Thus the caller can start looking for the next solution immediately.
    pub fn remove_last_piece(&mut self) -> Option<PieceSuggestion> {
        match self.positions.pop() {
            Option::Some(previous) => {
                self.used_letters.remove(&previous.name);
                Option::Some(previous)
            }
            _ => Option::None,
        }
    }

    /// If a suggested piece was successfully placed, this method should be called when requesting
    /// the next piece to place.
    ///
    /// The passed in piece will be saved in the state so that it is not suggested again in the same
    /// position if it is later removed.
    pub fn get_next_piece_to_try_after_success(
        &mut self,
        success: PieceSuggestion,
    ) -> Option<PieceAfterSuccess> {
        self.used_letters.insert(success.name);
        self.positions.push(success);

        return self
            .get_next_piece_to_try((('A' as u8) - 1) as char)
            .map(|piece| PieceAfterSuccess { piece });
    }

    /// If a suggested piece could not be placed, this method should be called when requesting
    /// the next piece to place.
    ///
    /// The passed in piece will be used to ensure that the next suggested piece comes after
    /// the failed piece.
    ///
    /// It is possible that there are no more pieces that can be suggested given the current
    /// placement state. If that is the case, previously placed pieces will be removed until
    /// a new suggestion is found (or all possible suggestions have been exhausted).
    ///
    /// It is the responsibility of the caller to remove any specified pieces before attempting
    /// to place the new suggestion.
    pub fn get_next_piece_to_try_after_failure(
        &mut self,
        failure: PieceSuggestion,
    ) -> Option<PieceAfterFailure> {
        // First we'll try a different orientation for the failed piece (if there is one).
        let next_index = failure.orientation_index + 1;
        let next_orientation = get_piece_orientation(failure.name, next_index);

        if next_orientation.is_some() {
            // We found another orientation for the piece, so let's return it to be tried.
            return Option::Some(PieceAfterFailure {
                piece: PieceSuggestion {
                    name: failure.name,
                    orientation_index: next_index,
                    shape: next_orientation.unwrap(),
                },
                to_remove: Vec::new(),
            });
        }

        // There were no remaining orientations for the piece, therefore, lets try the next
        // available piece.
        let next_piece = self.get_next_piece_to_try(failure.name);

        if next_piece.is_some() {
            // We found another piece so return it to be tried.
            return next_piece.map(|piece| PieceAfterFailure {
                piece,
                // No need to remove anything
                to_remove: Vec::new(),
            });
        }

        // Ok, there are no remaining pieces to try. That means we need to try popping off the
        // previously successfully placed piece and then getting a suggestion.
        let previous_success_opt = self.positions.pop();
        if previous_success_opt.is_none() {
            // We've popped everything off the queue and exhausted all possible suggestions.
            return Option::None;
        }

        let previous_success = previous_success_opt.unwrap();
        self.used_letters.remove(&previous_success.name);

        // Our previous success was actually a failure, take the previous success and
        // ask for the next piece/orientation that fits in the position we just emptied.
        // This is a recursive call, and we can end up popping off multiple successful
        // pieces until we find a piece we can actually suggest. This can happen if we
        // are popping off pieces that lexically follow all available unused pieces.
        //
        // Here is a scenario where that is the case:
        // positions: A[0], L[0], K[0], J[1]
        //
        // If J[1] is popped off, there is no piece available that can be placed. We've already
        // tried everything before J[1] (J has only 2 orientations), and L and K are both already
        // placed.
        //
        // If K[0] is then popped off, there is still no piece that can be placed. We've already
        // tried everything before K[0] (K has only 1 orientation), and L is still placed.
        //
        // If L[0] is then popped off, there is still no piece that can be placed. Same as above.
        //
        // Therefore, we will finally get to the point of popping of A[0]. After A[0] is popped,
        // we can suggest A[1].
        match self.get_next_piece_to_try_after_failure(previous_success.clone()) {
            Option::Some(result) => {
                // We popped off the previous success, so we need to tell the caller to do the same.
                let mut to_remove = result.to_remove.clone();
                to_remove.insert(0, previous_success);

                Option::Some(PieceAfterFailure {
                    piece: result.piece,
                    to_remove,
                })
            }
            _ => Option::None,
        }
    }

    fn get_next_piece_to_try(&self, after_char: char) -> Option<PieceSuggestion> {
        let mut next_name = (after_char as u8 + 1) as char;
        while next_name < 'M' && self.used_letters.contains(&next_name) {
            next_name = ((next_name as u8) + 1) as char;
        }

        return get_piece_orientation(next_name, 0).map(|shape| PieceSuggestion {
            name: next_name,
            orientation_index: 0,
            shape,
        });
    }
}

fn get_piece_orientation(piece_name: char, orientation_index: usize) -> Option<&'static Shape> {
    match (*PIECES).get(&piece_name) {
        Option::None => Option::None,
        Option::Some(piece) => piece.orientations.get(orientation_index),
    }
}
