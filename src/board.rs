use colored::*;
use std::fmt;
use std::fmt::{Formatter};

use once_cell::sync::Lazy;

use crate::shapes::Shape;

static EMPTY_SLOT: Lazy<ColoredString> = Lazy::new(||" ".clear());

pub struct Board<'a> {
    bits: [[&'a ColoredString; 11]; 5],
    next_pos: (usize, usize),
}


impl<'a> Board<'a> {
    pub fn new() -> Self {
        Board {
            bits: [[&*EMPTY_SLOT; 11]; 5],
            next_pos: (0, 0),
        }
    }

    pub fn solved(&self) -> bool {
        for i in 0..5 {
            for j in 0..11 {
                if self.bits[i][j] == &*EMPTY_SLOT {
                    return false;
                }
            }
        }
        true
    }

    pub fn does_shape_fit(&self, shape: &Shape) -> bool {
        let mut offset = 0;
        for i in 0..4 {
            if !shape.bits[0][i] {
                offset -= 1;
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                if shape.bits[i][j] {
                    let (r, c) = (i + self.next_pos.0, j + self.next_pos.1 + offset);
                    if r >= 5 || c < 0 || c >= 11 {
                        return false;
                    }
                    if self.bits[r][c] != &*EMPTY_SLOT {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn add_shape(&self, shape: &'a Shape) -> Board<'a> {
        let mut bits = [[&*EMPTY_SLOT; 11]; 5];
        bits.copy_from_slice(&self.bits);

        let mut offset: i32 = 0;
        for i in 0..4 {
            println!("{}", shape.bits[0][i]);
            if !shape.bits[0][i] {
                offset -= 1;
            } else {
                break;
            }
        }

        println!("offset => {} for shape\n{}", offset, shape);

        for i in 0..4 {
            for j in 0..4 {
                if shape.bits[i][j] {
                    let (r, c) = (i + self.next_pos.0, ((j + self.next_pos.1) as i32 + offset) as usize);
                    bits[r][c] = &shape.letter;

                }
            }
        }

        for i in 0..5 {
            for j in 0..11 {
                if bits[i][j] == &*EMPTY_SLOT {
                    return Board {
                        next_pos: (i, j),
                        bits
                    }
                }
            }
        }

        Board {
            next_pos: (0, 0),
            bits
        }
    }
}

impl fmt::Display for Board<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "-------------");
        for i in 0..5 {
            write!(f, "|");
            for j in 0..11 {
                write!(f, "{}", self.bits[i][j]);
            }
            writeln!(f, "|");
        }
        writeln!(f, "-------------");
        writeln!(f, "next position({}, {})", self.next_pos.0, self.next_pos.1)
    }
}