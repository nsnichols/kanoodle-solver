use crate::{layer::Position, pieces::Shape, Layers};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const EMPTY_SLOT: char = '·';

define_layers!(Rectangle, char, EMPTY_SLOT, { layer: 0, rows: 5, cols: 11 });

define_layers!(Pyramid, char, EMPTY_SLOT,
    { layer: 0, rows: 5, cols: 5 },
    { layer: 1, rows: 4, cols: 4 },
    { layer: 2, rows: 3, cols: 3 },
    { layer: 3, rows: 2, cols: 2 },
    { layer: 4, rows: 1, cols: 1 }
);

/// Defines a Kanoodle board that can be a flat rectangle or a 3d pyramid.
///
/// The board keeps track of the currently placed pieces and also the next
/// possible position a piece may be added.
pub struct Board<T: Layers<char>> {
    layers: T,
    next_pos: Position,
}

impl<T: Layers<char>> Board<T> {
    /// Determines if the board is currently in the solved state.
    /// It is solved if none of the cells in any layer are holding
    /// the empty slot char.
    pub fn solved(&self) -> bool {
        self.layers.find(&EMPTY_SLOT).is_none()
    }

    /// Determines if the board can hold the requested shape positioned
    /// at whatever the board's next_pos is. The shape must fit in its
    /// entirety on the board and cannot overlap any existing pieces.
    pub fn does_shape_fit(&self, shape: &Shape) -> bool {
        // Shapes may not be aligned such that their (0, 0) cell is set, but we must always add
        // the shape to the board in a way that fills the next_pos' position. Therefore, we need
        // to know how far we'll need to shift the shape when placing it on the board.
        let (shape_row_offset, shape_col_offset) = calculate_shape_offsets(shape);

        let board_layer_count = self.layers.layer_count();
        let Position(layer, next_row, next_col) = self.next_pos;

        // The shape and board layers are not necessarily equal. If we are trying to place a piece
        // where the next_pos layer is 2, the 0-layer of the shape will be matched against the 2-layer
        // of the board. 1 -> 3, 2 -> 4, etc. If there are any parts of the shape in its 3-layer, the
        // shape will not fit. We'll loop over the shape layers because those are what we need to
        // add to the board layers.
        for shape_layer_index in 0..shape.layer_count() {
            // As shape layers increase, we must also increase the board layer. Remember, they layers
            // must be kept in sync, but are not necessarily equal.
            let board_layer = layer + shape_layer_index;
            let (shape_size, _) = shape.dimensions(shape_layer_index);

            // Check if we've run out of layers on the board.
            if board_layer >= board_layer_count {
                // We've run out of layers, we'll know the shape doesn't fit if there is any part of
                // it in this next shape layer. Below is an example of attempting to place a 3d shape
                // starting on layer 2 of the pyramid board. At the last board layer (4) there are
                // still two shape layers (3 & 4) which have parts of the shape.
                //
                //                        C     <-< 4 shape layers
                //                         C    <-< 3 <-- The layer being checked in this if block
                // board layers 4 >->     C     <-< 2
                //              3 >->    C      <-< 1
                //              2 >->   A       <-< 0
                //              1 >->  A A K K
                //              0 >-> A B B B J
                for shape_row in 0..shape_size {
                    for shape_col in 0..shape_size {
                        if shape.is_set(shape_layer_index, shape_row, shape_col) {
                            // We found a piece of the shape in the layer, therefore
                            // this shape cannot fit on the board.
                            return false;
                        }
                    }
                }
                // Shapes are contiguous, and they always start at the lowest layer.
                // If we get this point, there was no part of the shape in this layer
                // and there cannot be any part of the shape on a higher layer either.
                // Therefore, we can say the shape fits.
                return true;
            }

            // We have at least one board layer in which we can see if the shape fits.

            let (board_row_count, board_col_count) = self.layers.dimensions(board_layer);

            let mut no_parts_found_in_layer = true;

            // Starting with the shape, for every cell that is part of the shape, we must find a
            // corresponding empty slot in the layer. If any part of the shape in this layer does
            // not have a corresponding empty slot in the layer, the shape does not fit.
            for shape_row in 0..shape_size {
                for shape_col in 0..shape_size {
                    if shape.is_set(shape_layer_index, shape_row, shape_col) {
                        // If the numbers go negative they'll wrap around and be much greater than the board
                        // or column count. This takes the place of checking for < 0.
                        let board_row = (next_row + shape_row).wrapping_sub(shape_row_offset);
                        let board_col = (next_col + shape_col).wrapping_sub(shape_col_offset);

                        if board_row >= board_row_count || board_col >= board_col_count {
                            // Our shape does not fit in the available space on this layer. There is at least
                            // one part of it that would be extend off the edge of the board if we tried to
                            // place it.
                            return false;
                        }
                        if *self.layers.at(board_layer, board_row, board_col) != EMPTY_SLOT {
                            // Our shape does not fit here. There is at least on part of it that would overlap
                            // with an existing shape already on the board.
                            return false;
                        }

                        no_parts_found_in_layer = false;
                    }
                }
            }

            if !shape.is_3d || no_parts_found_in_layer {
                // If the shape isn't 3d or the shape does not have any parts in the layer, that
                // means we can skip the remaining layers. We know the shape must fit.
                return true;
            }
        }

        // All our checks passed successfully! The shape fits.
        true
    }

    /// Adds the specified shape to the board.
    /// This method does not check if the shape can be added. Do not call this method
    /// unless you know the shape already fits on the board.
    pub fn add_shape(&mut self, shape: &Shape, letter: char) {
        let (shape_row_offset, shape_col_offset) = calculate_shape_offsets(shape);
        let Position(layer, next_row, next_col) = self.next_pos;

        for shape_layer_index in 0..shape.layer_count() {
            // Starting at the bottom layer of the shape, we place the shape
            // in the layer specified by next_pos. As the shape layer increases,
            // the board layer must as well.
            let board_layer = layer + shape_layer_index;
            let (shape_size, _) = shape.dimensions(shape_layer_index);
            let mut no_parts_found_in_layer = true;

            for shape_row in 0..shape_size {
                for shape_col in 0..shape_size {
                    if shape.is_set(shape_layer_index, shape_row, shape_col) {
                        let board_row = shape_row - shape_row_offset + next_row;
                        let board_col = shape_col - shape_col_offset + next_col;
                        self.layers
                            .update(board_layer, board_row, board_col, letter);
                        no_parts_found_in_layer = false;
                    }
                }
            }

            if !shape.is_3d || no_parts_found_in_layer {
                // If the shape isn't 3d or the shape does not have any parts in the layer, that
                // means we can skip the remaining layers. We've finished adding the shape.
                break;
            }
        }

        self.next_pos = self.layers.find(&EMPTY_SLOT).unwrap_or(Position(0, 0, 0));
    }

    /// Removes the space with the specified name from the board (if it is present).
    /// This method will do nothing if the shape is not present on the board.
    pub fn remove_shape(&mut self, name: char) {
        let mut updated_next_pos = false;
        let layer_count = self.layers.layer_count();
        for layer in 0..layer_count {
            let (row_count, col_count) = self.layers.dimensions(layer);
            for row in 0..row_count {
                for col in 0..col_count {
                    if *self.layers.at(layer, row, col) == name {
                        self.layers.update(layer, row, col, EMPTY_SLOT);
                        if !updated_next_pos {
                            updated_next_pos = true;
                            self.next_pos = Position(layer, row, col);
                        }
                    }
                }
            }
        }
    }
}

impl<T: Layers<char>> fmt::Display for Board<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut layer = self.layers.layer_count() - 1;
        loop {
            let (row_count, col_count) = self.layers.dimensions(layer);
            for row in 0..row_count {
                write!(f, "{:width$}", "", width = layer)?;
                for col in 0..col_count {
                    write!(f, " {}", self.layers.at(layer, row, col))?;
                }
                writeln!(f, " ")?;
            }
            if layer == 0usize {
                break;
            }
            layer -= 1;
        }
        Result::Ok(())
    }
}

impl Board<Rectangle> {
    pub fn new() -> Self {
        Board {
            layers: Rectangle::default(),
            next_pos: Position(0, 0, 0),
        }
    }
}

impl Board<Pyramid> {
    pub fn new() -> Self {
        Board {
            layers: Pyramid::default(),
            next_pos: Position(0, 0, 0),
        }
    }
}

pub enum Variation {
    Rectangle(Board<Rectangle>),
    Pyramid(Board<Pyramid>),
}

/// This sucks... I haven't really figured out the right way to handle
/// generic structs. In a language like Java, I'd be able to use a type
/// like Board<?> to hold the board and then it would dispatch to the
/// right class at run time. That doesn't seem to be possible in Rust
/// given the way I've implemented the Rectangle and Pyramid boards.
/// They have different sizes so I can't have a type that can point to
/// either impl types. Seems like I've got to do some Boxing or something.
/// This enum is basically a hand-rolled box anyway...
impl Variation {
    pub fn solved(&self) -> bool {
        match self {
            Variation::Rectangle(r) => r.solved(),
            Variation::Pyramid(p) => p.solved(),
        }
    }

    pub fn does_shape_fit(&self, shape: &Shape) -> bool {
        match self {
            Variation::Rectangle(r) => r.does_shape_fit(shape),
            Variation::Pyramid(p) => p.does_shape_fit(shape),
        }
    }

    pub fn add_shape(&mut self, shape: &Shape, letter: char) {
        match self {
            Variation::Rectangle(r) => r.add_shape(shape, letter),
            Variation::Pyramid(p) => p.add_shape(shape, letter),
        }
    }

    pub fn remove_shape(&mut self, name: char) {
        match self {
            Variation::Rectangle(r) => r.remove_shape(name),
            Variation::Pyramid(p) => p.remove_shape(name),
        }
    }
}

impl Display for Variation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Variation::Rectangle(r) => write!(f, "{}", r),
            Variation::Pyramid(p) => write!(f, "{}", p),
        }
    }
}

// Creates a new board of the requested type.
pub fn create_board(board_type: BoardType) -> Variation {
    return if board_type == BoardType::Rectangle {
        Variation::Rectangle(<Board<Rectangle>>::new())
    } else {
        Variation::Pyramid(<Board<Pyramid>>::new())
    };
}

#[derive(Debug, PartialEq)]
pub enum BoardType {
    Rectangle,
    Pyramid,
}

impl FromStr for BoardType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("pyramid") {
            Result::Ok(BoardType::Pyramid)
        } else {
            Result::Ok(BoardType::Rectangle)
        }
    }
}

fn calculate_shape_offsets(shape: &Shape) -> (usize, usize) {
    // Shapes may not be aligned to the top left still (3d shapes
    // in certain orientations cannot be represented in the top left
    // position), so we need to figure out offset between the shape's
    // cells and the board cells.
    // Here is an example of a shape that has an offset of (0, 2)
    // ..A..
    // AAA..
    // .....
    // 3d shapes can have much larger offsets since they have very specific
    // orientations and often the base part of the shape cannot be placed
    // in the top-left of the layer, due to the shape tilting to the north
    // west.
    let mut shape_row_offset = 0usize;
    let mut shape_col_offset = 0usize;

    // The first part of the shape we find starting from the top left and going
    // right and down is equal to the offset.
    'offsets: for row in 0..5 {
        for col in 0..5 {
            if shape.is_set(0, row, col) {
                shape_row_offset = row;
                shape_col_offset = col;
                break 'offsets;
            }
        }
    }
    (shape_row_offset, shape_col_offset)
}
