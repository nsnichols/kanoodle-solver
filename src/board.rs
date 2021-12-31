use std::fmt;
use std::fmt::{Formatter};

use crate::pieces::Shape;

const EMPTY_SLOT: char = ' ';

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
                    let (r, c): (usize, i32) = (i + self.next_pos.0, (j + self.next_pos.1) as i32 + offset);
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
        writeln!(f, "-------------")?;
        for i in 0..5 {
            write!(f, "|")?;
            for j in 0..11 {
                write!(f, "{}", self.cells[i][j])?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "-------------")?;
        writeln!(f, "next position({}, {})", self.next_pos.0, self.next_pos.1)
    }
}