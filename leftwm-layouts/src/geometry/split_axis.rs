use serde::{Deserialize, Serialize};

/// Describes different ways a [`geometry::Rect`] can be split.
///
/// *Disclaimer: As it may be confusing - The terms vertical/horizontal are referring to the "splits"
/// not the orientation of the resulting stack. For example, a [`SplitAxis::Horizontal`] SplitAxis splits a rect by horizontal cuts, resulting
/// in a "vertically stacked" list of rects. See the variants' documentation for clarification.*
#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum SplitAxis {
    /// Rectangle is split by `horizontal` cuts.
    ///
    /// ```txt
    /// +--------+      +--------+
    /// |        |      |        |
    /// |        |      +--------+
    /// |        |  =>  |        |
    /// |        |      +--------+
    /// |        |      |        |
    /// +--------+      +--------+
    /// ```
    Horizontal,

    /// Rectangle is split by `vertical` cuts.
    ///
    /// ```txt
    /// +--------+      +--+--+--+
    /// |        |      |  |  |  |
    /// |        |      |  |  |  |
    /// |        |  =>  |  |  |  |
    /// |        |      |  |  |  |
    /// |        |      |  |  |  |
    /// +--------+      +--+--+--+
    /// ```
    Vertical,

    /// Rectangle is split in a "Grid" pattern while still accounting for
    /// all of the available space, resulting in some rectangles being larger.
    ///
    /// ```txt
    /// +-------+      +---+---+
    /// |       |      |   |   |
    /// |       |      |   |   |
    /// |       |  =>  |   +---+
    /// |       |      |   |   |
    /// |       |      |   |   |
    /// +-------+      +---+---+
    /// ```
    Grid,

    /// Rectangle is split in a "Fibonacci" pattern.
    ///
    /// ```txt
    /// +-------+      +---+---+
    /// |       |      |   |   |
    /// |       |      |   |   |
    /// |       |  =>  |   +-+-+
    /// |       |      |   |_| |
    /// |       |      |   | | |
    /// +-------+      +---+---+
    /// ```
    Fibonacci,

    /// Rectangle is split in a "Fibonacci"-like pattern.
    /// But instead of spiraling into the middle, it spirals into the bottom right.
    ///
    /// ```txt
    /// +-------+      +---+---+
    /// |       |      |   |   |
    /// |       |      |   |   |
    /// |       |  =>  |   +-+-+
    /// |       |      |   | |_|
    /// |       |      |   | |||
    /// +-------+      +---+---+
    /// ```
    Dwindle,

    /// Rectangle will not be split at all,
    /// resulting in simply returning the Rect as is.
    None,
}
