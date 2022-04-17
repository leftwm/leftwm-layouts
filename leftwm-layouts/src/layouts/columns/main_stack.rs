use std::cmp;

use crate::{
    geometry::{self, remainderless_division, Rect, ReserveColumnSpace},
    LayoutModifiers,
};

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

fn column_widths(
    container_width: usize,
    window_count: usize,
    main_window_count: usize,
    main_size_percentage: f32,
    two_stacks: bool,
    reserve_column_space: ReserveColumnSpace,
) -> (Option<usize>, Option<usize>, Option<usize>) {
    let sanitised_main_count = cmp::min(window_count, main_window_count);
    let sanitised_stack_count = window_count.saturating_sub(sanitised_main_count);

    let reserve_main_space = sanitised_main_count > 0 || reserve_column_space.is_reserved();
    let reserve_first_stack_space = sanitised_stack_count > 0 || reserve_column_space.is_reserved();
    let reserve_second_stack_space =
        two_stacks && (sanitised_stack_count > 1 || reserve_column_space.is_reserved());

    let main_width = match reserve_main_space {
        true => Some((container_width as f32 / 100.0 * main_size_percentage) as usize),
        false => None,
    };

    let rest = container_width.saturating_sub(main_width.unwrap_or(0));

    let (first, second) = match (reserve_first_stack_space, reserve_second_stack_space) {
        (true, true) => {
            let split = remainderless_division(rest, 2);
            (Some(split[0]), Some(split[1]))
        }
        (true, false) => (Some(rest), None),
        (false, false) => (None, None),
        _ => panic!("impossible"), // fixme
    };

    if !reserve_column_space.is_reserved() && first.is_none() && second.is_none() {
        
    }

    (main_width, first, second)
}

fn main_stack_columns(
    window_count: usize,
    container: Rect,
    main_window_count: usize,
    main_size_percentage: f32,
    reserve_column_space: ReserveColumnSpace,
) -> (Option<Rect>, Option<Rect>) {
    let main_window_count = cmp::min(main_window_count, window_count);
    let stack_window_count = window_count.saturating_sub(main_window_count);

    let main_exists = main_window_count > 0 || reserve_column_space.is_reserved();
    let stack_exists = stack_window_count > 0 || reserve_column_space.is_reserved();

    let main_column = match (main_exists, stack_exists) {
        (false, _) => None,
        (true, true) => Some(Rect {
            w: (container.w as f32 / 100.0 * main_size_percentage) as u32,
            ..container
        }),
        (true, false) => Some(Rect { ..container }),
    };

    let stack_column: Option<Rect> = match (main_column, stack_exists) {
        (_, false) => None,
        (None, true) => Some(Rect { ..container }),
        (Some(m), true) => Some(Rect {
            x: m.w as i32,
            w: container.w - m.w,
            ..container
        }),
    };

    // center the columns
    /*if reserve_column_space.eq(&ReserveColumnSpace::ReserveAndCenter) {
        let main_invisible = main_column.is_some() && main_window_count == 0;
        let stack_invisible = stack_column.is_some() && stack_window_count == 0;

        if main_invisible && !stack_invisible {
            stack_column.unwrap().x = main_column.unwrap().w as i32;
        }

        if main_invisible ^ stack_invisible {
            main_column.filter(|_| stack_invisible).andthen(|c| c.w);
            if let Some(col) = main_column {

            }
        }
    }*/

    (main_column, stack_column)
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::{Rect, ReserveColumnSpace},
        layouts::columns::main_stack::main_stack_columns,
    };

    use super::column_widths;

    const CONTAINER: Rect = Rect {
        x: 0,
        y: 0,
        w: 400,
        h: 200,
    };

    const RESERVE_VARIANTS: &[ReserveColumnSpace] = &[
        ReserveColumnSpace::None,
        ReserveColumnSpace::Reserve,
        ReserveColumnSpace::ReserveAndCenter,
    ];

    /*#[test]
    fn zero_windows() {
        let rects = main_stack_columns(0, CONTAINER, 1, 60.0, false);
        assert_eq!(rects.0, None);
        assert_eq!(rects.1, None);
    }*/

    #[test]
    fn column_widths_without_windows_unreserved_all_none() {
        let widths = column_widths(1920, 0, 1, 60.0, true, ReserveColumnSpace::None);
        assert!(widths.0.is_none());
        assert!(widths.1.is_none());
        assert!(widths.2.is_none());
    }

    #[test]
    fn column_widths_without_windows_reserved_all_some() {
        let widths = column_widths(1920, 0, 1, 60.0, true, ReserveColumnSpace::Reserve);
        assert!(widths.0.is_some());
        assert!(widths.1.is_some());
        assert!(widths.2.is_some());

        let widths = column_widths(1920, 0, 1, 60.0, true, ReserveColumnSpace::ReserveAndCenter);
        assert!(widths.0.is_some());
        assert!(widths.1.is_some());
        assert!(widths.2.is_some());
    }

    #[test]
    fn column_widths_one_window_one_main_unreserved_has_full_width() {
        let widths = column_widths(1920, 1, 1, 60.0, true, ReserveColumnSpace::None);
        assert_eq!(widths.0, Some(1920));
        assert!(widths.1.is_none());
        assert!(widths.2.is_none());
    }
}
