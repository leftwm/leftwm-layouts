/// The `SplitAxis` describes different ways something can be split.
/// - `Horizontal` Splits in horizontal cuts
/// - `Vertical` Splits in vertical cuts
/// - `Both` Splits in a grid pattern
#[derive(PartialEq)]
pub enum SplitAxis {
    Horizontal,
    Vertical,
    Both,
}

impl SplitAxis {
    pub fn split_horizontally(&self) -> bool {
        self == &Self::Horizontal || self == &Self::Both
    }

    pub fn split_vertically(&self) -> bool {
        self == &Self::Vertical || self == &Self::Both
    }
}
