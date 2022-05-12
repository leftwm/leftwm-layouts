use std::cmp;

use crate::{
    geometry::{self, Rect},
    LayoutDefinition,
};

use super::three_column;

/// The `stack_main_stack` column layout consists of three columns,
/// two stacks on the outer edges, and a main column in the center.
///
/// The stack on the left is considered the "first" stack, and
/// the stack on the right is the "second" stack.
///
/// ```txt
/// +------+------------+------+
/// |      |            |      |
/// |STACK1|    MAIN    |STACK2|
/// |      |            |      |
/// +------+------------+------+
/// ```
pub fn stack_main_stack(
    window_count: usize,
    container: Rect,
    definition: &LayoutDefinition,
) -> Vec<Rect> {
    let tiles: &mut Vec<Rect> = &mut Vec::new();
    if window_count == 0 {
        return tiles.to_vec();
    }

    let main_window_count = cmp::min(definition.main_window_count, window_count);
    let stack_window_count = window_count.saturating_sub(main_window_count);
    let (left_window_count, right_window_count) = if definition.balance_stacks {
        let counts = geometry::remainderless_division(stack_window_count, 2);
        (counts[0], counts[1])
    } else {
        (1, cmp::max(0, stack_window_count - 1))
    };

    let (left_column, main_column, right_column) = three_column(
        window_count,
        container,
        main_window_count,
        definition.main_size,
        definition.reserve_column_space,
        definition.balance_stacks,
    );

    if let Some(tile) = main_column {
        tiles.append(&mut geometry::split(
            &tile,
            main_window_count,
            &definition.main_split,
        ));
    }

    if let Some(tile) = left_column {
        tiles.append(&mut geometry::split(
            &tile,
            left_window_count,
            &definition.stack_split,
        ));
    }

    if let Some(tile) = right_column {
        // don't worry if there are no stack windows, splitting by zero returns an empty vec :)
        tiles.append(&mut geometry::split(
            &tile,
            right_window_count,
            &definition.stack_split,
        ));
    }
    tiles.to_vec()
}
