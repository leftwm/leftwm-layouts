use serde::{Deserialize, Serialize};

/// Determines whether the space of a column should be reserved
/// when there is no window inside the column. A value of [`ReserveColumnSpace::Reserve`] or
/// [`ReserveColumnSpace::ReserveAndCenter`] will reserve the column space and make other
/// column(s) avoid it entirely. While a value of [`ReserveColumnSpace::None`]
/// makes other columns overtake the empty column space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ReserveColumnSpace {
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

impl ReserveColumnSpace {
    pub fn is_reserved(&self) -> bool {
        match self {
            ReserveColumnSpace::None => false,
            ReserveColumnSpace::Reserve | ReserveColumnSpace::ReserveAndCenter => true,
        }
    }
}

impl Default for ReserveColumnSpace {
    fn default() -> Self {
        ReserveColumnSpace::None
    }
}

#[cfg(test)]
mod tests {}
