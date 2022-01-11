use std::io::Read;
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
    about = "Finds Kanoodle (https://www.educationalinsights.com/kanoodle) solutions. An initial board state may be sent in stdin."
)]
struct CliOptions {
    /// Indicates the ending pieces at which the solver should sto finding solutions.
    /// Only solutions that are found between the starting pieces and orientation and
    /// this path will be found.
    ///
    /// Orientations must have leading zeros if they are single digits.
    ///
    /// Defaults to no limit (or all permutations beyond the initial state if
    /// backtracking is not allowed).
    ///
    /// Example: --ending-at "B[00]" "C[01]"
    #[structopt(short, long)]
    ending_at: Option<Vec<RequestedPiece>>,

    /// One or more pieces and orientations to display. When this option is present,
    /// other options passed will be ignored.
    ///
    /// Example: --display-pieces "A[00]" "B[02]" "C[10]"
    #[structopt(short = "p", long)]
    display_pieces: Option<Vec<RequestedPiece>>,

    /// Indicates which type of board (Rectangle or Pyramid) should be used
    /// when finding solutions.
    ///
    /// Defaults to "rectangular"
    #[structopt(short = "t", long)]
    board_type: Option<BoardType>,

    /// If an initial state is specified, enabling this flag lets
    /// the solver remove pieces from the initial state once it has
    /// exhausted all possible solutions given the initial state.
    ///
    /// This flag has no effect when no initial state is passed.
    ///
    /// Defaults to false
    #[structopt(short, long)]
    allow_backtracking: Option<bool>,
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

    solver::find_solutions(
        options.board_type,
        read_in_initial_state(),
        options.allow_backtracking,
        options.ending_at,
    );
}

fn read_in_initial_state() -> Option<Vec<String>> {
    if atty::is(atty::Stream::Stdin) {
        return Option::None;
    }

    let mut stdin = std::io::stdin();
    let mut input = String::new();
    match stdin.read_to_string(&mut input) {
        _ => (),
    }

    let lines: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();
    Option::Some(lines)
}
