use std::collections::{HashSet};
use std::fmt::{Display, Formatter};
use crate::pieces::{PIECES, Shape};

#[derive(Clone)]
pub struct PlacedPiece {
    pub name: char,
    pub shape: &'static Shape,
    orientation_index: usize,
}

pub struct PieceAfterSuccess {
    pub piece: PlacedPiece,
}

pub struct PieceAfterFailure {
    pub piece: PlacedPiece,
    pub to_remove: Vec<PlacedPiece>,
}

pub struct Placements {
    positions: Vec<PlacedPiece>,
    used_letters: HashSet<char>,
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

    pub fn get_first_piece_to_try(&mut self) -> PlacedPiece {
        PlacedPiece {
            name: 'A',
            orientation_index: 0,
            shape: get_piece_orientation('A', 0).unwrap()
        }
    }

    pub fn remove_last_piece(&mut self) -> Option<PlacedPiece> {
        match self.positions.pop() {
            Option::Some(previous) => {
                self.used_letters.remove(&previous.name);
                Option::Some(previous)
            }
            _ => Option::None
        }
    }

    pub fn get_next_piece_to_try_after_success(&mut self, success: PlacedPiece) -> Option<PieceAfterSuccess> {
        self.used_letters.insert(success.name);
        self.positions.push(success);

        return self.get_next_piece_to_try((('A' as u8) - 1) as char).map(|piece| {
            PieceAfterSuccess {
                piece
            }
        });
    }

    pub fn get_next_piece_to_try_after_failure(&mut self, failure: PlacedPiece) -> Option<PieceAfterFailure> {
        // First we'll try a different orientation for the failed piece (if there is one).
        let next_index = failure.orientation_index + 1;
        let next_orientation = get_piece_orientation(failure.name, next_index);

        if next_orientation.is_some() {
            // We found another orientation for the piece, so let's return it to be tried.
            return Option::Some(PieceAfterFailure {
                piece: PlacedPiece {
                    name: failure.name,
                    orientation_index: next_index,
                    shape: next_orientation.unwrap(),
                },
                to_remove: Vec::new()
            });
        }

        // There were no remaining orientations for the piece, therefore, lets try the next
        // available piece.
        let next_piece = self.get_next_piece_to_try(failure.name);

        if next_piece.is_some() {
            // We found another piece so return it to be tried.
            return next_piece.map(|piece| {
                PieceAfterFailure {
                    piece,
                    to_remove: Vec::new()
                }
            });
        }

        // Ok, there are no remaining pieces to try. That means we need to
        // try popping off the previously successfully placed piece and
        let previous_success_opt = self.positions.pop();
        if previous_success_opt.is_none() {
            // We've popped everything off the queue and exhausted all shapes.
            return Option::None;
        }

        let previous_success = previous_success_opt.unwrap();
        self.used_letters.remove(&previous_success.name);

        match self.get_next_piece_to_try_after_failure(previous_success.clone()) {
            Option::Some(result) => {
                // We popped off the previous success, so we need to tell the caller to do the same.

                let mut to_remove = result.to_remove.clone();
                to_remove.push(previous_success);

                Option::Some(PieceAfterFailure {
                    piece: result.piece,
                    to_remove,
                })
            }
            _ => Option::None
        }
    }

    fn get_next_piece_to_try(&self, after_char: char) -> Option<PlacedPiece> {
        let mut next_name = (after_char as u8 + 1) as char;
        while next_name < 'M' && self.used_letters.contains(&next_name) {
            next_name = ((next_name as u8) + 1) as char;
        }

        return get_piece_orientation(next_name, 0)
            .map(|shape| {
                PlacedPiece {
                    name: next_name,
                    orientation_index: 0,
                    shape,
                }
            });
    }
}

fn get_piece_orientation(piece_name: char, orientation_index: usize) -> Option<&'static Shape> {
    match (*PIECES).get(&piece_name) {
        Option::None => {
            Option::None
        }
        Option::Some(piece) => {
            piece.orientations.get(orientation_index)
        }
    }
}