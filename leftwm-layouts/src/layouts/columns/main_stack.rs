use crate::{
    geometry::{self, Rect},
    LayoutModifiers,
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
pub fn main_stack(window_count: usize, container: Rect, modifiers: &LayoutModifiers) -> Vec<Rect> {
    let tiles: &mut Vec<Rect> = &mut Vec::new();
    if window_count == 0 {
        return tiles.to_vec();
    }
    let (main_tile, stack_tile) = two_column(
        window_count,
        container,
        modifiers.main_window_count,
        modifiers.main_size,
        modifiers.reserve_column_space,
    );

    if let Some(tile) = main_tile {
        // don't worry if there are no main windows, splitting by zero returns an empty vec :)
        tiles.append(&mut geometry::split(
            &tile,
            modifiers.main_window_count,
            &modifiers.main_split,
        ));
    }

    if let Some(tile) = stack_tile {
        // don't worry if there are no stack windows, splitting by zero returns an empty vec :)
        tiles.append(&mut geometry::split(
            &tile,
            window_count.saturating_sub(modifiers.main_window_count),
            &modifiers.first_stack_split,
        ));
    }

    tiles.to_vec()
}

#[cfg(test)]
mod tests {}
