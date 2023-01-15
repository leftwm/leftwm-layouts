use serde::{Deserialize, Serialize};

/// Represents the four states an object can be in,
/// if it can be flipped horizontally and vertically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Flip {
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

impl Flip {
    /// Indicates whether the variant is flipped horizontally, independent of vertical
    pub fn is_flipped_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal | Self::Both)
    }

    /// Indicates whether the variant is flipped vertically, independent of horizontal
    pub fn is_flipped_vertical(&self) -> bool {
        matches!(self, Self::Vertical | Self::Both)
    }

    /// Returns the resulting [`Flip`] state when flipped horizontally
    #[must_use]
    pub fn toggle_horizontal(&self) -> Flip {
        match self {
            Self::None => Self::Horizontal,
            Self::Horizontal => Self::None,
            Self::Vertical => Self::Both,
            Self::Both => Self::Vertical,
        }
    }

    /// Returns the resulting [`Flip`] state when flipped vertically
    #[must_use]
    pub fn toggle_vertical(&self) -> Flip {
        match self {
            Self::None => Self::Vertical,
            Self::Horizontal => Self::Both,
            Self::Vertical => Self::None,
            Self::Both => Self::Horizontal,
        }
    }
}

impl Default for Flip {
    fn default() -> Self {
        Flip::None
    }
}
