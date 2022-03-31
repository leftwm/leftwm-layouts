/// Represents the four states an object can be in,
/// if it can be flipped horizontally and vertically.
///
/// ## Examples
/// ### None
/// ```txt
///     +---------+
///     |A       B|
///     |         |
///     |C       D|
///     +---------+
/// ```
///
/// ### Horizontal
/// ```txt
///     +---------+
///     |C       D|
/// >- -|- - - - -|- -< flipped on horizontal axis
///     |A       B|
///     +---------+
/// ```
///
/// ### Vertical
/// ```txt
///          v
///          |    
///     +---------+
///     |B   |   A|
///     |    |    |
///     |D   |   C|
///     +---------+
///          |
///          ^
///      flipped on
///     verical axis
/// ```
///
/// ### Both
/// ```txt
///          v
///          |
///     +---------+
///     |D   |   C|
/// >- -|- - + - -|- -< flipped on both axis
///     |B   |   A|
///     +---------+
///          |
///          ^
///      flipped on
///      both axis
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
