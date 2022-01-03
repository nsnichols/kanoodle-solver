use structopt::StructOpt;

use crate::board::BoardType;
use crate::layer::Layers;
use crate::pieces::PIECES;
use crate::placements::{PieceSuggestion, Placements, RequestedPiece};

#[macro_use]
mod layer;
mod board;
mod pieces;
mod placements;
mod solver;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "kanoodle-solver",
    about = "Finds Kanoodle (https://www.educationalinsights.com/kanoodle) solutions"
)]
struct CliOptions {
    /// Indicates the initial pieces, the solver should start with. If the pieces cannot be
    /// successfully added to the board in the order they are specified, the solver will
    /// panic.
    ///
    /// Orientations must have leading zeros if they are single digits.
    ///
    /// Defaults to A[00].
    ///
    /// Example -s "A[00]" "J[00]" "K[00]"
    #[structopt(short, long)]
    starting_at: Option<Vec<RequestedPiece>>,

    /// Indicates the ending pieces at which the solver should sto finding solutions.
    /// Only solutions that are found between the starting pieces and orientation and
    /// this path will be found.
    ///
    /// Orientations must have leading zeros if they are single digits.
    ///
    /// Defaults to no limit.
    ///
    /// Example: -c "B[00]" "C[01]"
    #[structopt(short, long)]
    ending_at: Option<Vec<RequestedPiece>>,

    /// One or more pieces and orientations to display. When this option is present,
    /// other options passed will be ignored.
    #[structopt(short, long)]
    display_pieces: Option<Vec<RequestedPiece>>,

    /// Indicates which type of board (Rectangle or Pyramid) should be used
    /// when finding solutions.
    ///
    /// Defaults to "rectangular"
    #[structopt(short, long)]
    board_type: Option<BoardType>,
}

fn main() {
    let options = CliOptions::from_args();

    let display_pieces: Option<Vec<RequestedPiece>> = options.display_pieces;
    if display_pieces.is_some() {
        let requested_pieces = display_pieces.unwrap();
        for requested_piece in requested_pieces {
            match (*PIECES).get(&requested_piece.name.to_ascii_uppercase()) {
                Option::Some(piece) => {
                    println!("{}", piece);
                    println!(
                        "[{}] => \n{}",
                        requested_piece.orientation_index,
                        piece
                            .orientations
                            .get(requested_piece.orientation_index)
                            .unwrap()
                    );
                }
                Option::None => {
                    println!("Piece {} not found!", &requested_piece.name);
                }
            }
        }

        return;
    }

    solver::find_solutions(options.starting_at, options.ending_at, options.board_type);
}
