use crate::{
    geometry::{self, Rect},
    LayoutDefinition,
};

use super::two_column;

/// The `main_stack` column layout consists of a main column
/// on the left, and a stack column on the right.
///
/// ```txt
/// +------------+-------+
/// |            |       |
/// |    MAIN    | STACK |
/// |            |       |
/// +------------+-------+
/// ```
///
/// *Hint: `main` being on the left and `stack` being on the right
/// is non configurable by design. To achieve a different outcome,
/// the generic Rotation and Flipped utilities can be used.*
pub fn main_stack(
    window_count: usize,
    container: &Rect,
    layout_definition: &LayoutDefinition,
) -> Vec<Rect> {
    let tiles: &mut Vec<Rect> = &mut Vec::new();
    if window_count == 0 {
        return tiles.clone();
    }
    let (main_tile, stack_tile) = two_column(
        window_count,
        container,
        layout_definition.main_window_count,
        layout_definition.main_size,
        layout_definition.reserve_column_space,
    );

    if let Some(tile) = main_tile {
        // don't worry if there are no main windows, splitting by zero returns an empty vec :)
        tiles.append(&mut geometry::split(
            &tile,
            layout_definition.main_window_count,
            &layout_definition.main_split,
        ));
    }

    if let Some(tile) = stack_tile {
        // don't worry if there are no stack windows, splitting by zero returns an empty vec :)
        tiles.append(&mut geometry::split(
            &tile,
            window_count.saturating_sub(layout_definition.main_window_count),
            &layout_definition.stack_split,
        ));
    }

    tiles.clone()
}

#[cfg(test)]
mod tests {}
