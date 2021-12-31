use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use once_cell::sync::Lazy;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Shape {
    cells: [[bool; 4]; 4]
}

impl Shape {
    pub fn is_set(&self, row: usize, col: usize) -> bool {
        if row < 4 && col < 4 {
            self.cells[row][col]
        } else {
            false
        }
    }

    pub fn rotate(&self) -> Shape {
        Shape {
            cells: [
                [self.cells[3][0], self.cells[2][0], self.cells[1][0], self.cells[0][0]],
                [self.cells[3][1], self.cells[2][1], self.cells[1][1], self.cells[0][1]],
                [self.cells[3][2], self.cells[2][2], self.cells[1][2], self.cells[0][2]],
                [self.cells[3][3], self.cells[2][3], self.cells[1][3], self.cells[0][3]],
            ]

        }
    }

    pub fn mirror(&self) -> Shape {
        Shape {
            cells: [
                [self.cells[0][0], self.cells[1][0], self.cells[2][0], self.cells[3][0]],
                [self.cells[0][1], self.cells[1][1], self.cells[2][1], self.cells[3][1]],
                [self.cells[0][2], self.cells[1][2], self.cells[2][2], self.cells[3][2]],
                [self.cells[0][3], self.cells[1][3], self.cells[2][3], self.cells[3][3]],
            ]
        }
    }

    pub fn snap_to_top_left(&self) -> Shape {
        let mut cells = [[false; 4]; 4];
        cells.copy_from_slice(&self.cells);

        // Shift up until we find a non-falsy value in a row.
        while !cells[0][0] && !cells[0][1] && !cells[0][2] && !cells[0][3] {
            cells[0] = cells[1];
            cells[1] = cells[2];
            cells[2] = cells[3];
            cells[3] = [false; 4];
        }

        // Shift left until we find a non-falsy value in a column.
        while !cells[0][0] && !cells[1][0] && !cells[2][0] && !cells[3][0] {
            cells[0] = [cells[0][1], cells[0][2], cells[0][3], false];
            cells[1] = [cells[1][1], cells[1][2], cells[1][3], false];
            cells[2] = [cells[2][1], cells[2][2], cells[2][3], false];
            cells[3] = [cells[3][1], cells[3][2], cells[3][3], false];
        }

        Shape {
            cells
        }
    }

    pub fn render(&self, letter: &String) -> String {
        let mut rendered_string = Vec::new();
        let empty = String::from(" ");
        let new_line = String::from("\n");

        for row in self.cells {
            for cell in row {
                rendered_string.push(if cell { letter.to_string() } else { empty.to_string() })
            }
            rendered_string.push(new_line.to_string())
        }

        rendered_string.join("")
    }
}

pub struct Piece {
    pub letter: String,
    pub orientations: Vec<Shape>
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [+{}]", self.letter, self.orientations.len())
    }
}

impl Piece {
    pub fn parse(value: &str, letter: char) -> Piece {
        let mut cells = [[false; 4]; 4];
        let mut row: usize = 0;
        let mut col: usize = 0;

        for v in value.chars() {
            if v == '\n' {
                row += 1;
                col = 0;
                continue
            }
            if v == letter {
                cells[row][col] = true;
            }
            col += 1;
        }

        let orientations = capture_orientations(Shape { cells }).into_iter().map(|s| s).collect();

        Piece {
            letter: letter.to_string(),
            orientations
        }
    }
}

fn capture_orientations(shape: Shape) -> Vec<Shape> {
    let mut set = HashSet::new();

    set.insert(shape.clone().snap_to_top_left());
    let next = shape.rotate();
    set.insert(next.snap_to_top_left());

    let next = next.rotate();
    set.insert(next.snap_to_top_left());

    let next = next.rotate();
    set.insert(next.snap_to_top_left());

    let next = shape.mirror();
    set.insert(next.snap_to_top_left());

    let next = next.rotate();
    set.insert(next.snap_to_top_left());

    let next = next.rotate();
    set.insert(next.snap_to_top_left());

    let next = next.rotate();
    set.insert(next.snap_to_top_left());

    let mut vec: Vec<Shape> = set.into_iter().collect();
    vec.sort_by(|s1, s2| to_int(s1).cmp(&to_int(s2)));

    vec
}

fn to_int(shape: &Shape) -> i32 {
    let mut v: i32 = 0;
    for i in 0..4 {
        let row_bonus = 10i32.pow(i as u32);
        for j in 0..4 {
            if shape.cells[i][j] {
                v += j as i32 * row_bonus;
            }
        }
    }
    return v;
}

pub static PIECES: Lazy<HashMap<char, Piece>> = Lazy::new(|| {
    let mut pieces = HashMap::new();

    pieces.insert('A', Piece::parse(
        "A\n\
               AAA",
        'A'
    ));

    pieces.insert('B', Piece::parse(
        "BB\n\
               BBB",
        'B'
    ));

    pieces.insert('C', Piece::parse(
        ".C\n\
               .C\n\
               .C\n\
               CC",
        'C'
    ));

    pieces.insert('D', Piece::parse(
        "DDDD\n\
               ..D",
        'D'
    ));

    pieces.insert('E', Piece::parse(
        "EE\n\
               .EEE",
        'E'
    ));

    pieces.insert('F', Piece::parse(
        "F\n\
               FF",
        'F'
    ));

    pieces.insert('G', Piece::parse(
        "GGG\n\
               ..G\n\
               ..G",
        'G'
    ));

    pieces.insert('H', Piece::parse(
        "HH\n\
               .HH\n\
               ..H",
        'H'
    ));

    pieces.insert('I', Piece::parse(
        "II\n\
               .I\n\
               II",
        'I'
    ));

    pieces.insert('J', Piece::parse(
        "JJJJ",
        'J'
    ));

    pieces.insert('K', Piece::parse(
        "KK\n\
               KK",
        'K'
    ));

    pieces.insert('L', Piece::parse(
        ".L\n\
               LLL\n\
               .L",
        'L'
    ));

    pieces
});
