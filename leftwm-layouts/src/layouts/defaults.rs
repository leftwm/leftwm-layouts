use crate::{
    geometry::{Reserve, Rotation, Split},
    Layout,
};

use super::layout::{Columns, Main, SecondStack, Stack};

const EVEN_HORIZONTAL: &str = "EvenHorizontal";
const EVEN_VERTICAL: &str = "EvenVertical";
const MONOCLE: &str = "Monocle";
const GRID: &str = "Grid";

const MAIN_AND_VERT_STACK: &str = "MainAndVertStack";
const MAIN_AND_HORIZONTAL_STACK: &str = "MainAndHorizontalStack";
const RIGHT_MAIN_AND_VERT_STACK: &str = "RightMainAndVertStack";
const FIBONACCI: &str = "Fibonacci";
const DWINDLE: &str = "Dwindle";
const MAIN_AND_DECK: &str = "MainAndDeck";

const CENTER_MAIN: &str = "CenterMain";
const CENTER_MAIN_BALANCED: &str = "CenterMainBalanced";
const CENTER_MAIN_FLUID: &str = "CenterMainFluid";

/// Layout which gives each window full height, but splits the workspace width among them all.
/// This layout has only one stack and no main column.
/// The stack is split in a [`Split::Vertical`] pattern (resulting in a horizontal stack).
///
/// ```txt
/// +--+--+--+--+
/// |  |  |  |  |
/// |  |  |  |  |
/// |  |  |  |  |
/// +--+--+--+--+
/// ```
pub fn even_horizontal() -> Layout {
    Layout {
        name: EVEN_HORIZONTAL.to_string(),
        columns: Columns {
            main: None,
            stack: Stack {
                split: Some(Split::Vertical),
                ..Default::default()
            },
            ..Columns::default()
        },
        ..Default::default()
    }
}

/// Layout which gives each window full width, but splits the workspace height among them all.
/// This layout has only one stack and no main column.
/// The stack is split in a [`Split::Horizontal`] pattern (resulting in a vertical stack).
///
/// ```txt
/// +-----------+
/// |-----------|
/// |-----------|
/// |-----------|
/// +-----------+
/// ```
pub fn even_vertical() -> Layout {
    Layout {
        name: EVEN_VERTICAL.to_string(),
        columns: Columns {
            main: None,
            stack: Stack::default(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Layout which gives only one window with the full real estate. A monocle mode.
/// This layout has only one stack and no main column, with the stack not splitting at all.
///
/// ```txt
/// +-----------+
/// |           |
/// |           |
/// |           |
/// +-----------+
/// ```
pub fn monocle() -> Layout {
    Layout {
        name: MONOCLE.to_string(),
        columns: Columns {
            main: None,
            stack: Stack {
                split: None,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Layout which splits the workspace in a [`Split::Grid`] pattern.
/// It will prioritize creating a new column instead of a row.
/// This layout has only one stack and no main column.
///
/// ```txt
/// +-----+-----+   +---+---+---+
/// |     |     |   |   |   |   |
/// |     |     |   |   |   |   |
/// +-----+-----+   |   +---+---+
/// |     |     |   |   |   |   |
/// |     |     |   |   |   |   |
/// +-----+-----+   +---+---+---+
///   4 windows       5 windows
///
/// +---+---+---+   +---+---+---+
/// |   |   |   |   |   |   |   |
/// |   |   |   |   |   |   +---+
/// +---+---+---+   +---+---|   |
/// |   |   |   |   |   |   +---+
/// |   |   |   |   |   |   |   |
/// +---+---+---+   +---+---+---+
///   6 windows       7 windows
/// ```
pub fn grid() -> Layout {
    Layout {
        name: GRID.to_string(),
        columns: Columns {
            main: None,
            stack: Stack {
                split: Some(Split::Grid),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Layout which splits the workspace into two columns (main and stack).
/// The stack is split in a [`Split::Horizontal`] pattern (resulting in a vertical stack).
///
/// ```txt
/// +-------+-----+
/// |       |     |
/// |       +-----+
/// |       |     |
/// +-------+-----+
///   main   stack
/// ```
pub fn main_and_vert_stack() -> Layout {
    Layout {
        name: MAIN_AND_VERT_STACK.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack::default(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Layout which splits the workspace into two columns (main and stack).
/// The stack is split in a [`Split::Vertical`] pattern (resulting in a horizontal stack).
///
/// ```txt
/// +-------+--+--+
/// |       |  |  |
/// |       |  |  |
/// |       |  |  |
/// +-------+--+--+
///   main   stack
/// ```
pub fn main_and_horizontal_stack() -> Layout {
    Layout {
        name: MAIN_AND_HORIZONTAL_STACK.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack {
                split: Some(Split::Vertical),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Layout which splits the workspace into two columns (main and stack),
/// with the main column being on the right side.
/// The stack is split in a [`Split::Horizontal`] pattern (resulting in a vertical stack).
///
/// ```txt
/// +-----+-------+
/// |     |       |
/// +-----+       |
/// |     |       |
/// +-----+-------+
///  stack   main
/// ```
pub fn right_main_and_vert_stack() -> Layout {
    Layout {
        name: RIGHT_MAIN_AND_VERT_STACK.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack::default(),
            rotate: Rotation::South,
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Layout which splits the workspace into two columns (main and stack).
/// The stack is split in a [`Split::Fibonacci`] pattern.
///
/// ```txt
/// +-------+-----+
/// |       |     |
/// |       +--+--+
/// |       |--|  |
/// +-------+--+--+
///   main   stack
/// ```
pub fn fibonacci() -> Layout {
    Layout {
        name: FIBONACCI.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack {
                split: Some(Split::Fibonacci),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Layout which splits the workspace into two columns (main and stack).
/// The stack is split in a [`Split::Dwindle`] pattern.
///
/// ```txt
/// +-------+-----+
/// |       |     |
/// |       +--+--+
/// |       |  |--|
/// +-------+--+--+
///   main   stack
/// ```
pub fn dwindle() -> Layout {
    Layout {
        name: DWINDLE.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack {
                split: Some(Split::Dwindle),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Layout similar to monocle, but with a non-splitting main column.
/// Never displays more than two windows at once.
///
/// ```txt
/// +-------+-----+
/// |       |     |
/// |       |     |
/// |       |     |
/// +-------+-----+
///   main   stack
/// ```
pub fn main_and_deck() -> Layout {
    Layout {
        name: MAIN_AND_DECK.to_string(),
        columns: Columns {
            main: Some(Main {
                split: None,
                ..Default::default()
            }),
            stack: Stack {
                split: None,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Layout which splits the workspace into three columns (stack | main | second stack).
///
/// * Puts first N (`main_window_count`) windows into middle (main) column
/// * Puts second window into left (stack) column
/// * Puts rest of windows into right (second stack) column
///
/// *Note: The space of unoccupied columns will be taken over, see [`Reserve::None`] for details.*
///
/// ```text
///  1st               2nd
///  stack     main    stack
/// +-----+-----------+-----+
/// |     |           |  3  |
/// |     |           +-----+
/// |  2  |     1     |  4  |
/// |     |           +-----+
/// |     |           |  5  |
/// +-----+-----------+-----+
///
/// +-----------+-----------+
/// |           |           |
/// |           |           |  unoccupied
/// |     2     |     1     |  space is
/// |           |           |  taken over
/// |           |           |
/// +-----------+-----------+
///
/// +-----------------------+
/// |                       |
/// |                       |  unoccupied
/// |           1           |  space is
/// |                       |  taken over
/// |                       |
/// +-----------------------+
/// ```
pub fn center_main() -> Layout {
    Layout {
        name: CENTER_MAIN.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack {
                split: None,
                ..Default::default()
            },
            second_stack: Some(SecondStack::default()),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Layout which splits the workspace into three columns (stack | main | second stack).
///
/// * Puts first N (`main_window_count`) windows into middle (main) column
/// * Distributes rest of windows between left and right column and splits the stacks in a [`Split::Dwindle`] pattern
///
/// *Hint: When dynamically adding windows, the windows already present may change the stack they are on.
/// That is because leftwm-layouts will keep them in order of `main` -> `1st stack` -> `2nd stack` instead
/// of going back and forth between 1st and second stack when going down the list of windows*
///
/// ```text
///  1st               2nd
///  stack     main    stack
/// +-----+-----------+-----+
/// |     |           |     |
/// |     |           |     |
/// +-----+           +--+--+
/// |  |__|           |  |__|
/// |  |  |           |  |  |
/// +-----+-----------+--+--+
/// ```
pub fn center_main_balanced() -> Layout {
    Layout {
        name: CENTER_MAIN_BALANCED.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack {
                split: Some(Split::Dwindle),
                ..Default::default()
            },
            second_stack: Some(SecondStack {
                split: Split::Dwindle,
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Layout which splits the workspace into three columns (stack | main | second stack).
/// The layout is similar to `CenterMain`, but the space of unoccupied columns will be reserved,
/// see [`Reserve::Reserve`] for details.
///
/// * Puts first N (`main_window_count`) windows into middle (main) column
/// * Puts second window into left (stack) column
/// * Puts rest of windows into right (second stack) column
///
/// ```text
///  1st               2nd
///  stack     main    stack
/// +-----+-----------+-----+
/// |     |           |  3  |
/// |     |           +-----+
/// |  2  |     1     |  4  |
/// |     |           +-----+
/// |     |           |  5  |
/// +-----+-----------+-----+
///
/// +-----+-----------+-----+
/// |.....|           |.....|
/// |.....|           |.....|  unoccupied
/// |.....|     1     |.....|  space is
/// |.....|           |.....|  reserved
/// |.....|           |.....|
/// +-----+-----------+-----+
///
/// +-----+-----------+-----+
/// |     |           |.....|
/// |     |           |.....|  unoccupied
/// |  2  |     1     |.....|  space is
/// |     |           |.....|  reserved
/// |     |           |.....|
/// +-----+-----------+-----+
/// ```
pub fn center_main_fluid() -> Layout {
    Layout {
        name: CENTER_MAIN_FLUID.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack {
                split: None,
                ..Default::default()
            },
            second_stack: Some(SecondStack::default()),
            ..Default::default()
        },
        reserve: Reserve::Reserve,
        ..Default::default()
    }
}
