use crate::board::Board;
use crate::placements::{PlacedPiece, Placements};

mod board;
mod pieces;
mod placements;

fn main() {
    let mut placement = placements::Placements::new();
    let mut board = board::Board::new();
    let mut solutions = 0;
    let mut piece = Option::Some(placement.get_first_piece_to_try());

    while piece.is_some() {
        let p = piece.unwrap();
        if board.does_shape_fit(p.shape) {
            board.add_shape(p.shape, p.name);
            piece = placement.get_next_piece_to_try_after_success(p).map(|success| success.piece);
        } else {
            piece = get_next_piece_to_try_after_failure(&mut placement, &mut board, p);
        }

        if board.solved() {
            solutions += 1;
            println!("{}", board);
            println!("{}", placement);
            println!("solution: {}", solutions);
            // We got a solution. Remove the last piece and find the next one.
            piece = match placement.remove_last_piece() {
                Option::Some(last) => {
                    board.remove_shape(last.name);
                    get_next_piece_to_try_after_failure(&mut placement, &mut board, last)
                }
                _ => Option::None
            }
        }
    }
}

fn get_next_piece_to_try_after_failure(placement: &mut Placements, board: &mut Board, failed_piece: PlacedPiece) -> Option<PlacedPiece> {
    match placement.get_next_piece_to_try_after_failure(failed_piece) {
        Option::Some(placements::PieceAfterFailure { piece, to_remove }) => {
            // There was a failure. Make sure the board reverts any necessary piece placements
            for piece_to_remove in to_remove {
                board.remove_shape(piece_to_remove.name);
            }
            Option::Some(piece)
        }
        _ => Option::None
    }
}


