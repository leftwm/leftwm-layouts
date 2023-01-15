use serde::{Deserialize, Serialize};

/// Determines whether the space of a layouts' column should be reserved
/// when there is no window inside the column. A value of [`Reserve::Reserve`] or
/// [`Reserve::ReserveAndCenter`] will reserve the column space and make other
/// column(s) avoid it entirely. While a value of [`Reserve::None`]
/// makes other columns overtake the empty column space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reserve {
    /// No space will be reserved. Instead, the populated space
    /// will take over the empty space. This is the default variant.
    ///
    /// ```txt
    /// +--------------+
    /// |              |
    /// |     MAIN     |
    /// |              |
    /// +--------------+
    /// ```
    None,

    /// Empty space is reserved in-place
    /// and won't be populated with other elements
    ///
    /// ```txt
    /// +--------+-----+
    /// |        |     |
    /// |  MAIN  |     |
    /// |        |     |
    /// +--------+-----+
    ///             ^
    ///    reserved empty space
    /// ```
    Reserve,

    /// Empty space is reserved in terms of amount of space,
    /// but not in terms of its position. Instead the populated
    /// space will be centered, while the empty space is accounted
    /// for on each side.
    ///
    /// ```txt
    /// +--+--------+--+
    /// |  |        |  |
    /// |  |  MAIN  |  |
    /// |  |        |  |
    /// +--+--------+--+
    ///  ^            ^
    /// reserved empty space
    /// ```
    ReserveAndCenter,
}

impl Reserve {
    pub fn is_reserved(&self) -> bool {
        match self {
            Reserve::None => false,
            Reserve::Reserve | Reserve::ReserveAndCenter => true,
        }
    }
}

impl Default for Reserve {
    fn default() -> Self {
        Reserve::None
    }
}

#[cfg(test)]
mod tests {}
