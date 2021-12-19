/// Represents the four states an object can be in,
/// if it can be flipped horizontally and vertically.
///
/// ## Examples
/// ### None
/// ```txt
/// +--------+
/// |A      B|
/// |        |
/// |C      D|
/// +--------+
/// ```
///
/// ### Horizontal
/// ```txt
/// +--------+
/// |C      D|
/// |        |
/// |A      B|
/// +--------+
/// ```
///
/// ### Vertical
/// ```txt
/// +--------+
/// |B      A|
/// |        |
/// |D      C|
/// +--------+
/// ```
///
/// ### Both
/// ```txt
/// +--------+
/// |D      C|
/// |        |
/// |B      A|
/// +--------+
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flipped {
    None,
    Horizontal,
    Vertical,
    Both,
}

impl Flipped {
    pub fn is_flipped_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal | Self::Both)
    }

    pub fn is_flipped_vertical(&self) -> bool {
        matches!(self, Self::Vertical | Self::Both)
    }

    pub fn toggle_horizontal(&self) -> Flipped {
        match self {
            Self::None => Self::Horizontal,
            Self::Horizontal => Self::None,
            Self::Vertical => Self::Both,
            Self::Both => Self::Vertical,
        }
    }

    pub fn toggle_vertical(&self) -> Flipped {
        match self {
            Self::None => Self::Vertical,
            Self::Horizontal => Self::Both,
            Self::Vertical => Self::None,
            Self::Both => Self::Horizontal,
        }
    }
}

impl Default for Flipped {
    fn default() -> Self {
        Flipped::None
    }
}
