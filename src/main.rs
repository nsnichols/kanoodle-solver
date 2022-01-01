use structopt::StructOpt;

use crate::board::Board;
use crate::placements::{PieceSuggestion, Placements, RequestedPiece};

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
    /// Indicates the piece and orientation the solver should start with.
    ///
    /// Defaults to A[0].
    ///
    /// Example
    #[structopt(short, long)]
    starting_at: Option<RequestedPiece>,

    /// Indicates ending placement path at which the solver should stop
    /// finding solutions. Only solutions that are found between the starting
    /// piece and orientation and this path will be found.
    ///
    /// Defaults to no restriction.
    ///
    /// Example: -c "B[0]" "C[1]"
    #[structopt(short, long)]
    ending_at: Option<Vec<RequestedPiece>>,
}

fn main() {
    let options = CliOptions::from_args();

    solver::find_solutions(options.starting_at, options.ending_at);
}
