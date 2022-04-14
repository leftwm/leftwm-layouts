use std::cmp;

use crate::{geometry::{self, Rect}, LayoutModifiers};

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
    let (main_tile, stack_tile) = main_stack_columns(
        window_count,
        container,
        modifiers.main_window_count,
        modifiers.main_size_percentage,
        modifiers.reserve_empty_space,
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

fn main_stack_columns(
    window_count: usize,
    container: Rect,
    main_window_count: usize,
    main_size_percentage: f32,
    reserve_empty_space: bool,
) -> (Option<Rect>, Option<Rect>) {
    let main_window_count = cmp::min(main_window_count, window_count);
    let stack_window_count = window_count.saturating_sub(main_window_count);

    let main_exists = main_window_count > 0 || reserve_empty_space;
    let stack_exists = stack_window_count > 0 || reserve_empty_space;

    let main_column = match (main_exists, stack_exists) {
        (false, _) => None,
        (true, true) => Some(Rect {
            w: (container.w as f32 / 100.0 * main_size_percentage) as u32,
            ..container
        }),
        (true, false) => Some(Rect { ..container }),
    };

    let stack_column = match (main_column, stack_exists) {
        (_, false) => None,
        (None, true) => Some(Rect { ..container }),
        (Some(m), true) => Some(Rect {
            x: m.w as i32,
            w: container.w - m.w,
            ..container
        }),
    };

    (main_column, stack_column)
}

#[cfg(test)]
mod tests {
    use crate::{geometry::Rect, layouts::columns::main_stack::main_stack_columns};

    const CONTAINER: Rect = Rect {
        x: 0,
        y: 0,
        w: 400,
        h: 200,
    };

    #[test]
    fn zero_windows() {
        let rects = main_stack_columns(0, CONTAINER, 1, 60.0, false);
        assert_eq!(rects.0, None);
        assert_eq!(rects.1, None);
    }
}
