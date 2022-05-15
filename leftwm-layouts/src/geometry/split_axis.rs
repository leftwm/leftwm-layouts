use serde::{Deserialize, Serialize};

/// The `SplitAxis` describes the different ways a `Rect` can be split.
///
/// *Disclaimer: As it may be confusing - The terms vertical/horizontal are referring to the "splits"
/// not the orientation of the resulting stack. A "horizontal" SplitAxis splits a rect by horizontal cuts, resulting
/// in a "vertically stacked" list of rects. See the demonstration for clarification.*
///
/// ### Demonstration
/// Splitting a `Rect` into three smaller rectangles would look as follows.
///
/// ### Vertical
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
///
/// ### Horizontal
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
///
/// ### Grid
/// Rectangle is split in a "Grid" pattern while still accounting for
/// all of the available space, result in some rectangles being larger.
/// ```txt
/// +-------+      +---+---+
/// |       |      |   |   |
/// |       |      |   |   |
/// |       |  =>  |   +---+
/// |       |      |   |   |
/// |       |      |   |   |
/// +-------+      +---+---+
/// ```
///
/// ### Fibonacci
/// Rectangle is split in a "Fibonacci" pattern.
/// ```txt
/// +-------+      +---+---+
/// |       |      |   |   |
/// |       |      |   |   |
/// |       |  =>  |   +-+-+
/// |       |      |   |_| |
/// |       |      |   | | |
/// +-------+      +---+---+
/// ```
///
/// ### Dwindle
/// Rectangle is split in a "Fibonacci"-like pattern.
/// But instead of spiraling into the middle, it spirals into the bottom right.
/// ```txt
/// +-------+      +---+---+
/// |       |      |   |   |
/// |       |      |   |   |
/// |       |  =>  |   +-+-+
/// |       |      |   | |_|
/// |       |      |   | |||
/// +-------+      +---+---+
/// ```
///
/// ### None
/// Rectangle will not be split at all, resulting in simply returning
/// the Rect as is.
#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum SplitAxis {
    Horizontal,
    Vertical,
    Grid,
    Fibonacci,
    Dwindle,
    None,
}
