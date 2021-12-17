/// The `SplitAxis` describes different ways something can be split.
#[derive(PartialEq)]
pub enum SplitAxis {
    Horizontal,
    Vertical,
    Both, // splits in a grid
}

impl SplitAxis {
    pub fn split_horizontally(&self) -> bool {
        self == &Self::Horizontal || self == &Self::Both
    }

    pub fn split_vertically(&self) -> bool {
        self == &Self::Vertical || self == &Self::Both
    }
}
