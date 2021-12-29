use std::fmt;
use std::fmt::{Formatter, Write};

use colored::*;

#[derive(Clone)]
pub struct Shape {
    pub letter: ColoredString,
    pub bits: [[bool; 4]; 4],
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..4 {
            for j in 0..4 {
                if self.bits[i][j] {
                    write!(f, "{}", self.letter);
                } else {
                    f.write_char(' ');
                }
            }
            f.write_char('\n');
        }
        write!(f, "")
    }
}

impl Shape {
    pub fn rotate(&self) -> Shape {
        return Shape {
            letter: self.letter.clone(),
            bits: [
                [self.bits[3][0], self.bits[2][0], self.bits[1][0], self.bits[0][0]],
                [self.bits[3][1], self.bits[2][1], self.bits[1][1], self.bits[0][1]],
                [self.bits[3][2], self.bits[2][2], self.bits[1][2], self.bits[0][2]],
                [self.bits[3][3], self.bits[2][3], self.bits[1][3], self.bits[0][3]],
            ]

        }
    }

    pub fn mirror(&self) -> Shape {
        return Shape {
            letter: self.letter.clone(),
            bits: [
                [self.bits[0][0], self.bits[1][0], self.bits[2][0], self.bits[3][0]],
                [self.bits[0][1], self.bits[1][1], self.bits[2][1], self.bits[3][1]],
                [self.bits[0][2], self.bits[1][2], self.bits[2][2], self.bits[3][2]],
                [self.bits[0][3], self.bits[1][3], self.bits[2][3], self.bits[3][3]],
            ]
        }
    }

    pub fn snap_to_top_left(&self) -> Shape {
        let mut bits = [[false; 4]; 4];
        bits.copy_from_slice(&self.bits);

        // Shift up until we find a non-falsy value in a row.
        while !bits[0][0] && !bits[0][1] && !bits[0][2] && !bits[0][3] {
            bits[0] = bits[1];
            bits[1] = bits[2];
            bits[2] = bits[3];
            bits[3] = [false; 4];
        }

        // Shift left until we find a non-falsy value in a column.
        while !bits[0][0] && !bits[1][0] && !bits[2][0] && !bits[3][0] {
            bits[0] = [bits[0][1], bits[0][2], bits[0][3], false];
            bits[1] = [bits[1][1], bits[1][2], bits[1][3], false];
            bits[2] = [bits[2][1], bits[2][2], bits[2][3], false];
            bits[3] = [bits[3][1], bits[3][2], bits[3][3], false];
        }

        return Shape {
            letter: self.letter.clone(),
            bits
        }
    }

    pub fn equivalent(&self, other: &Shape) -> bool {
        if self.letter.eq(&other.letter) {
            return false
        }

        for i in 0..4 {
            for j in 0..4 {
                if self.bits[i][j] != other.bits[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

pub fn shape_e() -> Shape {
    Shape {
        letter: "E".green(),
        bits: [
            [true, true, false, false],
            [false, true, true, true],
            [false; 4],
            [false; 4]],
    }
}

pub fn shape_j() -> Shape {
    Shape {
        letter: "J".purple(),
        bits: [
            [true; 4],
            [false; 4],
            [false; 4],
            [false; 4]],
    }
}

pub fn shape_a() -> Shape {
    Shape {
        letter: "A".truecolor(255, 165, 0), // Orange
        bits: [
            [true, false, false, false],
            [true, true, true, false],
            [false; 4],
            [false; 4]],
    }
}

pub fn shape_b() -> Shape {
    Shape {
        letter: "B".red(),
        bits: [
            [true, true, false, false],
            [true, true, true, false],
            [false; 4],
            [false; 4]],
    }
}

pub fn shape_g() -> Shape {
    Shape {
        letter: "G".cyan(),
        bits: [
            [true, true, true, false],
            [false, false, true, false],
            [false, false, true, false],
            [false; 4]],
    }
}

pub fn shape_l() -> Shape {
    Shape {
        letter: "L".truecolor(211,211,211), // grey
        bits: [
            [false, true, false, false],
            [true, true, true, false],
            [false, true, false, false],
            [false; 4]],
    }
}

pub fn shape_c() -> Shape {
    Shape {
        letter: "C".blue(),
        bits: [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [true, true, false, false]],
    }
}

pub fn shape_d() -> Shape {
    Shape {
        letter: "D".truecolor(253, 185, 200), // Pink
        bits: [
            [true, true, true, true],
            [false, false, true, false],
            [false; 4],
            [false; 4]],
    }
}

pub fn shape_h() -> Shape {
    Shape {
        letter: "H".magenta(),
        bits: [
            [true, true, false, false],
            [false, true, true, false],
            [false, false, true, false],
            [false; 4]],
    }
}

pub fn shape_f() -> Shape {
    Shape {
        letter: "F".white(),
        bits: [
            [true, false, false, false],
            [true, true, false, false],
            [false;  4],
            [false; 4]],
    }
}

pub fn shape_i() -> Shape {
    Shape {
        letter: "I".yellow(),
        bits: [
            [true, true, false, false],
            [false, true, false, false],
            [true, true, false, false],
            [false; 4]],
    }
}

pub fn shape_k() -> Shape {
    Shape {
        letter: "K".bright_green(),
        bits: [
            [true, true, false, false],
            [true, true, false, false],
            [false; 4],
            [false; 4]],
    }
}

pub struct ShapeOrientations {
    pub letter: ColoredString,
    pub variations: Vec<Shape>
}

pub fn build_orientations(shape: &Shape) -> ShapeOrientations {
    let mut vec: Vec<Shape> = Vec::new();

    vec.push(shape.clone());
    let next = shape.rotate();
    vec.push(next.snap_to_top_left());

    let next = next.rotate();
    vec.push(next.snap_to_top_left());

    let next = next.rotate();
    vec.push(next.snap_to_top_left());

    let next = shape.mirror();
    vec.push(next.snap_to_top_left());

    let next = next.rotate();
    vec.push(next.snap_to_top_left());

    let next = next.rotate();
    vec.push(next.snap_to_top_left());

    let next = next.rotate();
    vec.push(next.snap_to_top_left());

    vec.sort_by(|s1, s2| hash(s1).cmp(&hash(s2)));
    vec.dedup_by(|s1, s2| s1.equivalent(s2));

    return ShapeOrientations {
        letter: shape.letter.clone(),
        variations: vec,
    }
}

fn hash(shape: &Shape) -> i32 {
    let mut h: i32 = 0;
    for i in 0..4 {
        let n = i32::pow(10, i);
        for j in 0..4 {
            if shape.bits[i as usize][j as usize] {
                h += i32::pow(2, j) * n;
            }
        }
    }
    h
}