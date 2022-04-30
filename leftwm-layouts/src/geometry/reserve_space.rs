/// Determines whether the space of a column should be reserved
/// when there is no window inside the column. A value of 'Reserve' or
/// 'ReserveAndCenter' will "reserve" the column space and make other
/// column(s) avoid it entirely.
///
/// ## Demonstration
/// When there is only one main window and
/// no stack windows, the modifier has the following effects.
///
/// When set to `None` (default)
/// ```txt
/// +--------------+
/// |              |
/// |     MAIN     |
/// |              |
/// +--------------+
/// ```
///
/// When set to `Reserve`
/// ```txt
/// +--------+-----+
/// |        |     |
/// |  MAIN  |     |
/// |        |     |
/// +--------+-----+
///             ^
///    reserved empty space
/// ```
///
/// When set to `ReserveAndCenter`
/// ```txt
/// +--+--------+--+
/// |  |        |  |
/// |  |  MAIN  |  |
/// |  |        |  |
/// +--+--------+--+
///  ^            ^
/// reserved empty space
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReserveColumnSpace {
    None,
    Reserve,
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
