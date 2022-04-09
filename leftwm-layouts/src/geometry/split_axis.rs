/// The `SplitAxis` describes different ways something can be split.
/// - `Horizontal` Splits in horizontal cuts
/// - `Vertical` Splits in vertical cuts
/// - `Both` Splits in a grid pattern
#[derive(PartialEq, Clone, Copy)]
pub enum SplitAxis {
    Horizontal,
    Vertical,
    Grid,
    Fibonacci,
    Fakebonacci,
}

impl SplitAxis {
    /*pub fn split_horizontally(&self) -> bool {
        self == &Self::Horizontal || self == &Self::Both
    }

    pub fn split_vertically(&self) -> bool {
        self == &Self::Vertical || self == &Self::Both
    }*/
}

impl Default for SplitAxis {
    fn default() -> Self {
        Self::Vertical
    }
}
