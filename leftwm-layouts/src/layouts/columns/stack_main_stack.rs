use std::cmp;

use crate::{
    geometry::{self, Rect},
    LayoutModifiers,
};

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
    modifiers: &LayoutModifiers,
) -> Vec<Rect> {
    let tiles: &mut Vec<Rect> = &mut Vec::new();
    if window_count == 0 {
        return tiles.to_vec();
    }

    let main_window_count = cmp::min(modifiers.main_window_count, window_count);
    let stack_window_count = window_count.saturating_sub(main_window_count);
    let (left_window_count, right_window_cound) = if modifiers.balance_stacks {
        let counts = geometry::remainderless_division(stack_window_count, 2);
        (counts[0], counts[1])
    } else {
        (1, stack_window_count - 1)
    };

    let (left_column, main_column, right_column) = stack_main_stack_columns(
        window_count,
        container,
        main_window_count,
        modifiers.main_size_percentage,
        modifiers.reserve_empty_space,
    );

    if let Some(tile) = main_column {
        tiles.append(&mut geometry::split(
            &tile,
            main_window_count,
            &modifiers.main_split,
        ));
    }

    if let Some(tile) = left_column {
        tiles.append(&mut geometry::split(
            &tile,
            left_window_count,
            &modifiers.first_stack_split,
        ));
    }

    if let Some(tile) = right_column {
        // don't worry if there are no stack windows, splitting by zero returns an empty vec :)
        tiles.append(&mut geometry::split(
            &tile,
            right_window_cound,
            &modifiers.second_stack_split,
        ));
    }
    tiles.to_vec()
}

fn stack_main_stack_columns(
    window_count: usize,
    container: Rect,
    main_window_count: usize,
    main_size_percentage: f32,
    reserve_empty_space: bool,
) -> (Option<Rect>, Option<Rect>, Option<Rect>) {
    let main_window_count = cmp::min(main_window_count, window_count);
    let stack_window_count = window_count.saturating_sub(main_window_count);

    let main_exists = main_window_count > 0 || reserve_empty_space;
    let left_stack_exists = stack_window_count > 0 || reserve_empty_space;
    let right_stack_exists = left_stack_exists && stack_window_count > 1 || reserve_empty_space;

    let main_width = (container.w as f32 / 100.0 * main_size_percentage) as u32;
    let main_column = match (main_exists, left_stack_exists, right_stack_exists) {
        (false, _, _) => None,
        (true, false, false) => Some(Rect { ..container }),
        (true, true, false) => Some(Rect {
            x: (container.w - main_width) as i32,
            w: main_width,
            ..container
        }),
        (true, true, true) => {
            let stack_widths =
                geometry::remainderless_division(container.w as usize - main_width as usize, 2);
            Some(Rect {
                x: stack_widths[0] as i32,
                w: main_width,
                ..container
            })
        }
        (true, false, true) => panic!(
            "right stack cannot exist when left stack does not exist, this is a programming error"
        ),
    };

    let rest_width = container.w - main_column.map_or_else(|| 0, |c| c.w);
    let (left_column, right_column) = match (left_stack_exists, right_stack_exists) {
        (false, false) => (None, None),
        (true, false) => {
            let left = Rect {
                w: rest_width,
                ..container
            };
            (Some(left), None)
        }
        (true, true) => {
            let stack_widths = geometry::remainderless_division(rest_width as usize, 2);
            let left = Rect {
                w: stack_widths[0] as u32,
                ..container
            };
            let right = Rect {
                x: (container.w - stack_widths[1] as u32) as i32,
                w: stack_widths[1] as u32,
                ..container
            };
            (Some(left), Some(right))
        }
        (false, true) => panic!(
            "right stack cannot exist when left stack does not exist, this is a programming error"
        ),
    };

    (left_column, main_column, right_column)
}
