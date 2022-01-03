pub struct Position(pub usize, pub usize, pub usize);

pub trait Layers<T: Copy + PartialEq> {
    fn default_cell_value(&self) -> T;

    fn layer_count(&self) -> usize;

    fn dimensions(&self, layer_index: usize) -> (usize, usize);

    fn at(&self, layer_index: usize, row_index: usize, col_index: usize) -> &T;

    fn update(&mut self, layer_index: usize, row_index: usize, col_index: usize, val: T);

    fn find(&self, val: &T) -> Option<Position> {
        let layer_count = self.layer_count();
        for layer in 0..layer_count {
            let (row_count, col_count) = self.dimensions(layer);
            for row in 0..row_count {
                for col in 0..col_count {
                    if *self.at(layer, row, col) == *val {
                        return Option::Some(Position(layer, row, col));
                    }
                }
            }
        }
        Option::None
    }
}

#[macro_export]
macro_rules! count {
    ($t:tt) => { 1usize };
    ($t:tt, $($rest:tt),*) => {
        1usize + count!($({ $rest }),*)
    }
}

#[macro_export]
macro_rules! match_layer {
    ($self:ident, $index:ident, mut $layer:ident, $row:ident, $col:ident, $val:ident, $($i:tt),+ $block:block) => {
        match $index {
            $(
                $i => {
                    let $layer = &mut $self.$i;
                    $block
                },
            )+
            _ => panic!("layer not found!")
        }
    };
    ($self:ident, $index:ident, $layer:ident, $row:ident, $col:ident, $($i:tt),+ $block:block) => {
        match $index {
            $(
                $i => {
                    let $layer = &$self.$i;
                    $block
                },
            )+
            _ => panic!("layer not found!")
        }
    }
}

#[macro_export]
macro_rules! define_layers {
    ($struct:ident, $type:ty, $default:expr, $({ layer: $layer:tt, rows: $rows:literal, cols: $cols:literal }),+) => {
        #[derive(Clone, Debug, Hash, Eq, PartialEq)]
        pub struct $struct($([[$type; $cols]; $rows],)+);

        impl $struct {
            const fn layer_count() -> usize {
                count!($($rows),+)
            }

            const fn default_cell_value() -> $type {
                $default
            }
        }

        impl crate::layer::Layers<$type> for $struct {

            fn default_cell_value(&self) -> $type {
                $struct::default_cell_value()
            }

            fn layer_count(&self) -> usize {
                $struct::layer_count()
            }

            fn dimensions(&self, layer_index: usize) -> (usize, usize) {
                match_layer!(self, layer_index, layer, r, c, $($layer),+ {
                    // All rows in the layer have the same number of columns
                    // So we can get the column count by checking the first
                    // row's length.
                    return (layer.len(), layer[0].len());
                });
            }

             fn at(&self, layer_index: usize, row_index: usize, col_index: usize) -> &$type {
                match_layer!(self, layer_index, layer, row_index, col_index, $($layer),+ {
                    return &layer[row_index][col_index];
                });
            }

            fn update(&mut self, layer_index: usize, row_index: usize, col_index: usize, val: $type) {
                match_layer! (self, layer_index, mut layer, row_index, col_index, val, $($layer),+ {
                    layer[row_index][col_index] = val;
                });
            }
        }

        impl Default for $struct {
            fn default() -> Self {
                $struct($([[$default; $cols]; $rows]),+ )
            }
        }
    };
}
