use serde::{Deserialize, Serialize};

/// Represents the four states an object can be in,
/// if it can be flipped horizontally and vertically.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Flipped {
    /// Nothing is flipped at all
    ///
    /// ```txt
    ///     +---------+
    ///     |A       B|
    ///     |         |
    ///     |C       D|
    ///     +---------+
    /// ```
    None,

    /// Flipped on the horizontal axis
    ///
    /// ```txt
    ///     +---------+
    ///     |C       D|
    /// >- -|- - - - -|- -<
    ///     |A       B|
    ///     +---------+
    /// ```
    Horizontal,

    /// Flipped on the vertical axis
    ///
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
    /// ```
    Vertical,

    /// Flipped on horizontal and vertical axis
    ///
    /// ```txt
    ///          v
    ///          |
    ///     +---------+
    ///     |D   |   C|
    /// >- -|- - + - -|- -<
    ///     |B   |   A|
    ///     +---------+
    ///          |
    ///          ^
    /// ```
    Both,
}

impl Flipped {
    /// Indicates whether the variant is flipped horizontally
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
