use std::fmt;
use std::fmt::Formatter;

use crate::pieces::Shape;

const EMPTY_SLOT: char = ' ';

/// Defines a Kanoodle board which is 5 x 11
///
/// The board keeps track of the currently placed pieces (by char) and also
/// the next possible position a piece may be added.
pub struct Board {
    cells: [[char; 11]; 5],
    next_pos: (usize, usize),
}

impl Board {
    pub const fn new() -> Board {
        Board {
            cells: [[EMPTY_SLOT; 11]; 5],
            next_pos: (0, 0),
        }
    }

    /// Determines if the board is currently in the solved state.
    /// It is solved if all of the cells are holding part of a piece.
    pub fn solved(&self) -> bool {
        for i in 0..5 {
            for j in 0..11 {
                if self.cells[i][j] == EMPTY_SLOT {
                    return false;
                }
            }
        }
        true
    }

    /// Determines if the board can hold the requested shape positioned
    /// at whatever the board's next_pos is. The shape must fit in it's
    /// entirety on the board and cannot overlap any existing pieces.
    pub fn does_shape_fit(&self, shape: &Shape) -> bool {
        let mut offset: i32 = 0;
        for i in 0..4 {
            if !shape.is_set(0, i) {
                offset -= 1;
            } else {
                break;
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                if shape.is_set(i, j) {
                    let (r, c): (usize, i32) =
                        (i + self.next_pos.0, (j + self.next_pos.1) as i32 + offset);
                    if r >= 5 || c < 0 || c >= 11 {
                        return false;
                    }
                    if self.cells[r][c as usize] != EMPTY_SLOT {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Adds the specified shape to the board.
    /// This method does not check if the shape can be added. Do not call this method
    /// unless you know the shape already fits on the board.
    pub fn add_shape(&mut self, shape: &Shape, letter: char) {
        let mut offset: i32 = 0;
        for i in 0..4 {
            if !shape.is_set(0, i) {
                offset -= 1;
            } else {
                break;
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                if shape.is_set(i, j) {
                    let (r, c) = (i + self.next_pos.0, ((j + self.next_pos.1) as i32 + offset));
                    self.cells[r][c as usize] = letter;
                }
            }
        }

        'outer: for i in 0..5 {
            for j in 0..11 {
                if self.cells[i][j] == EMPTY_SLOT {
                    self.next_pos = (i, j);
                    break 'outer;
                }
            }
        }
    }

    /// Removes the space with the specified name from the board (if it is present).
    /// This method will do nothing if the shape is not present on the board.
    pub fn remove_shape(&mut self, name: char) {
        let mut updated_next_pos = false;
        for i in 0..5 {
            for j in 0..11 {
                if self.cells[i][j] == name {
                    self.cells[i][j] = EMPTY_SLOT;
                    if !updated_next_pos {
                        updated_next_pos = true;
                        self.next_pos = (i, j);
                    }
                }
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, ".............")?;
        for i in 0..5 {
            write!(f, ".")?;
            for j in 0..11 {
                write!(f, "{}", self.cells[i][j])?;
            }
            writeln!(f, ".")?;
        }
        write!(f, ".............")
    }
}
