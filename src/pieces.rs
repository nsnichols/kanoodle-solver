use crate::Layers;
use once_cell::sync::Lazy;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

define_layers!(ShapeLayers, bool, false,
    { layer: 0, rows: 5, cols: 5 },
    { layer: 1, rows: 4, cols: 4 },
    { layer: 2, rows: 3, cols: 3 },
    { layer: 3, rows: 2, cols: 2 },
    { layer: 4, rows: 1, cols: 1 }
);

enum ShiftInstruction {
    Up,
    Down,
    Left,
    // Right,
    Stop,
}

/// Defines a specific Kanoodle shape
///
/// # Example
/// L-shape (`A`)
/// ```
/// [[true, false, false, false, false],
/// [true, false, false, false, false],
/// [true, true, false, false, false],
/// [false, false, false, false, false],
/// [false, false, false, false, false]]
/// ```
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Shape {
    layers: ShapeLayers,
    pub is_3d: bool,
}

impl Shape {
    // If the cell at the specified layer, row, and column is set, that
    // cell is part of the shape.
    pub fn is_set(&self, layer: usize, row: usize, col: usize) -> bool {
        *self.layers.at(layer, row, col)
    }

    pub fn dimensions(&self, layer: usize) -> (usize, usize) {
        self.layers.dimensions(layer)
    }

    pub const fn layer_count(&self) -> usize {
        ShapeLayers::layer_count()
    }

    /// Creates a new shape that has been rotated 90 degrees clockwise
    pub fn rotate(&self) -> Shape {
        let mut rotated = self.layers.clone();
        transform_each_layer_2d(&self.layers, &mut rotated, |row, col, max_col| {
            (max_col - col, row)
        });
        Shape {
            layers: rotated,
            is_3d: self.is_3d,
        }
    }

    /// Creates a new shape that is a mirror image of the shape.
    pub fn reflect(&self) -> Shape {
        let mut reflected = self.layers.clone();
        transform_each_layer_2d(&self.layers, &mut reflected, |row, col, _| (col, row));
        Shape {
            layers: reflected,
            is_3d: self.is_3d,
        }
    }

    // Creates a new shape that has been shifted to the top left to remove whitespace.
    pub fn snap_to_top_left(&self) -> Shape {
        // Shift up until we find a non-falsy value in a row at any layer.
        let mut layers = shift(&self.layers, |ls| {
            if !ls.0[0][0]
                && !ls.0[0][1]
                && !ls.0[0][2]
                && !ls.0[0][3]
                && !ls.0[0][4]
                && !ls.1[0][0]
                && !ls.1[0][1]
                && !ls.1[0][2]
                && !ls.1[0][3]
                && !ls.2[0][0]
                && !ls.2[0][1]
                && !ls.2[0][2]
                && !ls.3[0][0]
                && !ls.3[0][1]
                && !ls.4[0][0]
            {
                ShiftInstruction::Up
            } else {
                ShiftInstruction::Stop
            }
        });

        // Shift left until we find a non-falsy value in a column at any layer.
        layers = shift(&layers, |ls| {
            if !ls.0[0][0]
                && !ls.0[1][0]
                && !ls.0[2][0]
                && !ls.0[3][0]
                && !ls.0[4][0]
                && !ls.1[0][0]
                && !ls.1[1][0]
                && !ls.1[2][0]
                && !ls.1[3][0]
                && !ls.2[0][0]
                && !ls.2[1][0]
                && !ls.2[2][0]
                && !ls.3[0][0]
                && !ls.3[1][0]
                && !ls.4[0][0]
            {
                ShiftInstruction::Left
            } else {
                ShiftInstruction::Stop
            }
        });

        return Shape {
            layers,
            is_3d: self.is_3d,
        };
    }

    pub fn erect(&self) -> Shape {
        if self.is_3d {
            return self.clone();
        }

        // Ensure we're starting at:
        // C C C C ·
        // C · · · ·
        // · · · · ·
        // · · · · ·
        let aligned = self.snap_to_top_left();

        // Shift the shape until no part of it is to the right of the tl-br diagonal:
        // · · · · ·
        // · · · · ·
        // · · · · ·
        // C C C C ·
        // C · · · ·

        // The shape isn't 3d right now so we only have to worry about layer 0 when
        // shifting it.
        let layer0 = &mut shift(&aligned.layers, |ls| {
            if ls.0[0][1] || ls.0[1][2] || ls.0[2][3] || ls.0[3][4] {
                ShiftInstruction::Down
            } else {
                ShiftInstruction::Stop
            }
        })
        .0;

        // Erect the shape to the north-east keeping all pieces on the tl-br diagonal as
        // the pivots.
        // Side view:
        //     C
        //    C ·
        //   · C ·
        //  · · C ·
        // · · · C ·

        // Pieces on the center diagonal remain in layer 0
        // The next diagonal to the left is layer 1, the next is layer 2, etc.
        // Start at layer4 and work backwards.

        let layer4 = [[layer0[4][0]]];

        let layer3 = [[layer0[3][0], false], [false, layer0[4][1]]];

        let layer2 = [
            [layer0[2][0], false, false],
            [false, layer0[3][1], false],
            [false, false, layer0[4][2]],
        ];

        let layer1 = [
            [layer0[1][0], false, false, false],
            [false, layer0[2][1], false, false],
            [false, false, layer0[3][2], false],
            [false, false, false, layer0[4][3]],
        ];

        // Now that we've copied the proper values into the layers above 0, we can
        // update layer0 to clear out any values that moved to a different layer.
        // This means that all bools other than then ones in the tl-br diagonal should
        // be set to false.
        for i in 0..5 {
            for j in 0..5 {
                if i != j {
                    layer0[i][j] = false;
                }
            }
        }

        return Shape {
            layers: ShapeLayers(*layer0, layer1, layer2, layer3, layer4),
            is_3d: true,
        };
    }

    /// Parses a vector of strings into a shape. This vector may contain
    /// multiple shapes. The specific shape that will be parsed is the one
    /// specified by the letter.
    ///
    /// Each string in the vector represents a shape layer (to support
    /// parsing 3d shapes). Rows in a layer are separated by new lines.
    ///
    /// # Examples
    ///
    /// Given the vector:
    /// ```
    /// ["AAABB\nA.BBB"]
    /// ```
    /// Shapes `A` and `B` can be parsed from it.
    ///
    /// Given the vector:
    /// ```
    /// ["A..\n...\n...\n",  "A.\n.A\n", "A"]
    /// ```
    /// 3D shape `A` can be parsed from it.
    /// ```
    ///   A
    ///  A A
    /// A . .
    /// ```
    pub fn parse(strings: &Vec<String>, letter: char) -> Option<Shape> {
        // First we need to figure out if the shape is offset.
        // Shapes are 5x5 at the most. If we're parsing a string
        // that is larger than that, we need to make sure we don't
        // overflow the shape dimensions.
        let mut row_offset = usize::MAX;
        let mut col_offset = usize::MAX;

        strings.iter().for_each(|string| {
            let mut row = 0usize;
            let mut col = 0usize;
            for ch in string.chars() {
                match ch {
                    '\n' => {
                        row += 1;
                        col = 0;
                    }
                    c if c == letter => {
                        row_offset = row_offset.min(row);
                        col_offset = col_offset.min(col);
                        col += 1;
                    }
                    _ => col += 1,
                }
            }
        });

        // Now that we have our offsets, we can actually create the shape.
        let mut layers = ShapeLayers::default();
        let layer_count = strings.len();
        let mut layers_with_shape = HashSet::new();

        for layer in 0..layer_count {
            let string: &String = strings.get(layer).unwrap();
            let mut row = 0usize;
            let mut col = 0usize;
            for ch in string.chars() {
                match ch {
                    '\n' => {
                        row += 1;
                        col = 0;
                    }
                    c if c == letter => {
                        // These offset subtractions should never overflow since they are always the
                        // minimum possible values. They should always be <= row and col everywhere.
                        layers.update(layer, row - row_offset, col - col_offset, true);
                        layers_with_shape.insert(layer);
                        col += 1;
                    }
                    _ => col += 1,
                }
            }
        }

        if layers.find(&true).is_some() {
            Option::Some(
                Shape {
                    layers,
                    is_3d: layers_with_shape.len() > 1,
                }
                .snap_to_top_left(),
            )
        } else {
            Option::None
        }
    }
}

fn shift(
    layers: &ShapeLayers,
    next_instruction: fn(to_be_shifted: &ShapeLayers) -> ShiftInstruction,
) -> ShapeLayers {
    let mut source = layers.clone();
    let mut dest = layers.clone();

    let mut max_size = 0usize;
    for layer in 0..layers.layer_count() {
        let (size, _) = layers.dimensions(layer);
        max_size = max(max_size, size);
    }

    // Allow full shifts in every direction before giving up.
    let mut remaining_iterations = max_size * 4;

    while remaining_iterations > 0 {
        match next_instruction(&dest) {
            ShiftInstruction::Left => {
                transform_each_layer_2d(&source, &mut dest, |row, col, size| {
                    (row, if col == size { 0usize } else { col + 1 })
                });
            }
            ShiftInstruction::Up => {
                transform_each_layer_2d(&source, &mut dest, |row, col, size| {
                    (if row == size { 0usize } else { row + 1 }, col)
                });
            }
            // ShiftInstruction::Right => {
            //     transform_each_layer_2d(&source, &mut dest, |row, col, size| {
            //         (row, if col == 0 { size } else { col - 1 })
            //     });
            // }
            ShiftInstruction::Down => {
                transform_each_layer_2d(&source, &mut dest, |row, col, size| {
                    (if row == 0 { size } else { row - 1 }, col)
                });
            }
            ShiftInstruction::Stop => {
                return dest;
            }
        }

        // Copy values back into source so we can updated dest again in the next iteration
        // of the loop
        std::mem::swap(&mut source, &mut dest);

        remaining_iterations -= 1;
    }

    panic!("Detected infinite loop in shift logic");
}

fn transform_each_layer_2d(
    layers: &ShapeLayers,
    to_be_transformed: &mut ShapeLayers,
    transformer: fn(usize, usize, usize) -> (usize, usize),
) {
    let layer_count = layers.layer_count();

    for layer in 0..layer_count {
        let (row_count, col_count) = layers.dimensions(layer);
        assert_eq!(
            row_count, col_count,
            "A non-square layer cannot be transformed"
        );
        let size = row_count - 1;
        for row in 0..row_count {
            for col in 0..col_count {
                let (transformed_row, transformed_col) = transformer(row, col, size);
                let val = layers.at(layer, transformed_row, transformed_col);
                to_be_transformed.update(layer, row, col, *val);
            }
        }
    }
}

const EMPTY_CELL: &str = "·";
const FILLED_CELL: &str = "●";

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut vec = Vec::new();
        if self.is_3d {
            append_layer(&mut vec, &self.layers.4);
            append_layer(&mut vec, &self.layers.3);
            append_layer(&mut vec, &self.layers.2);
            append_layer(&mut vec, &self.layers.1);
        }

        for row in self.layers.0 {
            for col in row {
                vec.push(if col { FILLED_CELL } else { EMPTY_CELL })
            }
            vec.push("\n")
        }
        writeln!(f, "{}", vec.join(""))?;

        Result::Ok(())
    }
}

fn append_layer<Level: AsRef<[Row]>, Row: AsRef<[bool]>>(vec: &mut Vec<&str>, bools: Level) {
    for row in bools.as_ref() {
        for col in row.as_ref() {
            vec.push(if *col { FILLED_CELL } else { EMPTY_CELL });
        }
        vec.push("\n");
    }
    vec.push("\n");
}

/// Defines a Kanoodle piece.
///
/// A piece consists of a name (letter) A - L, and a vector that contains
/// all possible orientations it may be legally by placed in.
pub struct Piece {
    pub letter: String,
    pub orientations: Vec<Shape>,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({} orientations)",
            self.letter,
            self.orientations.len()
        )
    }
}

impl Piece {
    /// Parses a string that defines a `Piece`'s shape in its
    /// default orientation.
    ///
    /// The remaining orientations are automatically derived from the
    /// default orientation.
    pub fn parse(value: &str, letter: char) -> Piece {
        let to_parse = vec![value.to_string()];
        let shape = Shape::parse(&to_parse, letter).unwrap();

        let orientations = generate_orientations(shape.layers.0)
            .into_iter()
            .map(|s| s)
            .collect();

        Piece {
            letter: letter.to_string(),
            orientations,
        }
    }
}

/// Given a single shape, this function determines all possible orientations of that
/// shape and returns them in a vector.
///
/// The orientations in the vector returned in a consistent order.
///
/// `to_int` determines the sort order.
fn generate_orientations(cells: [[bool; 5]; 5]) -> Vec<Shape> {
    let mut set = HashSet::new();
    let mut layer0 = [[false; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            layer0[i][j] = cells[i][j];
        }
    }

    let shape = Shape {
        layers: ShapeLayers(
            layer0,
            [[false; 4]; 4],
            [[false; 3]; 3],
            [[false; 2]; 2],
            [[false; 1]; 1],
        ),
        is_3d: false,
    };

    add_rotated_orientations(&shape, &mut set);
    add_mirrored_orientations(&shape, &mut set);

    let mut vec: Vec<Shape> = set.into_iter().collect();
    vec.sort_by(|s1, s2| to_int(s1).cmp(&to_int(s2)));

    vec
}

fn add_rotated_orientations(shape: &Shape, set: &mut HashSet<Shape>) {
    let mut next = shape.clone();
    for _ in 0..4 {
        set.insert(next.snap_to_top_left());
        if !&next.is_3d {
            // Add rotated 3d orientations.
            add_rotated_orientations(&next.erect(), set);
        }
        // The last rotate in the loop brings us back to 0, so we can toss it.
        next = next.rotate();
    }
}

fn add_mirrored_orientations(shape: &Shape, set: &mut HashSet<Shape>) {
    add_rotated_orientations(&shape.reflect(), set);
}

pub fn to_int(shape: &Shape) -> u64 {
    let mut int_repr: u64 = 0;
    let mut layer_bonus: u64 = 1;

    for layer in 0..ShapeLayers::layer_count() {
        layer_bonus = match layer {
            0 => 1,
            1 => 10000,
            _ => layer_bonus * 100,
        };

        let (row_count, col_count) = shape.layers.dimensions(layer);

        for row in 0..row_count {
            let row_bonus: u64 = 10u64.pow(row as u32) * layer_bonus;
            for col in 0..col_count {
                if *shape.layers.at(layer, row, col) {
                    int_repr += (col as u32 + 1u32) as u64 * row_bonus;
                }
            }
        }
    }

    int_repr
}

/// Holds a static map of all possible Kanoodle pieces keyed by
/// their letter names.
pub static PIECES: Lazy<HashMap<char, Piece>> = Lazy::new(|| {
    let mut pieces = HashMap::new();

    pieces.insert(
        'A',
        Piece::parse(
            "A\n\
               AAA",
            'A',
        ),
    );

    pieces.insert(
        'B',
        Piece::parse(
            "BB\n\
               BBB",
            'B',
        ),
    );

    pieces.insert(
        'C',
        Piece::parse(
            ".C\n\
               .C\n\
               .C\n\
               CC",
            'C',
        ),
    );

    pieces.insert(
        'D',
        Piece::parse(
            "DDDD\n\
               ..D",
            'D',
        ),
    );

    pieces.insert(
        'E',
        Piece::parse(
            "EE\n\
               .EEE",
            'E',
        ),
    );

    pieces.insert(
        'F',
        Piece::parse(
            "F\n\
               FF",
            'F',
        ),
    );

    pieces.insert(
        'G',
        Piece::parse(
            "GGG\n\
               ..G\n\
               ..G",
            'G',
        ),
    );

    pieces.insert(
        'H',
        Piece::parse(
            "HH\n\
               .HH\n\
               ..H",
            'H',
        ),
    );

    pieces.insert(
        'I',
        Piece::parse(
            "II\n\
               .I\n\
               II",
            'I',
        ),
    );

    pieces.insert('J', Piece::parse("JJJJ", 'J'));

    pieces.insert(
        'K',
        Piece::parse(
            "KK\n\
               KK",
            'K',
        ),
    );

    pieces.insert(
        'L',
        Piece::parse(
            ".L\n\
               LLL\n\
               .L",
            'L',
        ),
    );

    pieces
});
