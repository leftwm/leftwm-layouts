use std::cmp;

use crate::geometry::{Rect, Reserve, Size};

/// Calculate a two column layout (ie. layout with a main and stack part)
/// based on the provided parameters.
///
/// * `window_count` - Amount of windows to account for
/// * `container` - Container [`Rect`] in which the windows shall be displayed
/// * `main_window_count` - How many of the windows shall be in the main column
/// * `main_size` - Size of the main column
/// * `reserve_column_space` - How to handle unused column space
pub fn two_column(
    window_count: usize,
    container: &Rect,
    main_window_count: usize,
    main_size: Size,
    reserve_column_space: Reserve,
) -> (Option<Rect>, Option<Rect>) {
    let main_window_count = cmp::min(main_window_count, window_count);
    let stack_window_count = window_count.saturating_sub(main_window_count);

    let main_has_windows = main_window_count > 0;
    let stack_has_windows = stack_window_count > 0;

    let main_reserve = main_has_windows || reserve_column_space.is_reserved();
    let stack_reserve = stack_has_windows || reserve_column_space.is_reserved();

    let main_empty = !main_has_windows && reserve_column_space.is_reserved();
    let stack_empty = !stack_has_windows && reserve_column_space.is_reserved();

    let main_width = match (main_reserve, stack_reserve) {
        (true, true) => main_size.into_absolute(container.w) as usize,
        (true, false) => container.w as usize,
        _ => 0,
    };
    let stack_width = container.w as usize - main_width;

    let main_offset = match (reserve_column_space, stack_empty) {
        (Reserve::ReserveAndCenter, true) => stack_width / 2,
        _ => 0,
    };
    let stack_offset = match (reserve_column_space, main_empty) {
        (Reserve::ReserveAndCenter, true) => main_width / 2,
        _ => main_width,
    };

    let main = if main_has_windows {
        Some(Rect {
            x: main_offset as i32,
            y: container.y,
            w: main_width as u32,
            h: container.h,
        })
    } else {
        None
    };

    let stack = if stack_has_windows {
        Some(Rect {
            x: stack_offset as i32,
            y: container.y,
            w: stack_width as u32,
            h: container.h,
        })
    } else {
        None
    };

    (main, stack)
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Rect, Size};

    use super::two_column;

    const CONTAINER: Rect = Rect {
        x: 0,
        y: 0,
        w: 5120,
        h: 1440,
    };

    #[test]
    fn two_column_with_filled_columns() {
        let (main, stack) = two_column(
            3,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::None,
        );
        assert_eq!(
            main,
            Some(Rect {
                x: 0,
                y: 0,
                w: 3328,
                h: 1440
            })
        );
        assert_eq!(
            stack,
            Some(Rect {
                x: 3328,
                y: 0,
                w: 1792,
                h: 1440
            })
        );
    }

    #[test]
    fn two_column_with_filled_columns_reserved() {
        let (main, stack) = two_column(
            3,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::Reserve,
        );
        assert_eq!(
            main,
            Some(Rect {
                x: 0,
                y: 0,
                w: 3328,
                h: 1440
            })
        );
        assert_eq!(
            stack,
            Some(Rect {
                x: 3328,
                y: 0,
                w: 1792,
                h: 1440
            })
        );
    }

    #[test]
    fn two_column_with_filled_columns_reserved_and_centered() {
        let (main, stack) = two_column(
            3,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::ReserveAndCenter,
        );
        assert_eq!(
            main,
            Some(Rect {
                x: 0,
                y: 0,
                w: 3328,
                h: 1440
            })
        );
        assert_eq!(
            stack,
            Some(Rect {
                x: 3328,
                y: 0,
                w: 1792,
                h: 1440
            })
        );
    }

    #[test]
    fn two_column_with_no_stack_windows_unreserved() {
        let (main, stack) = two_column(
            1,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::None,
        );
        assert_eq!(
            main,
            Some(Rect {
                x: 0,
                y: 0,
                w: 5120,
                h: 1440
            })
        );
        assert_eq!(stack, None);
    }

    #[test]
    fn two_column_with_no_main_windows_unreserved() {
        let (main, stack) = two_column(
            1,
            &CONTAINER,
            0,
            Size::Ratio(0.65),
            crate::geometry::Reserve::None,
        );
        assert_eq!(main, None);
        assert_eq!(
            stack,
            Some(Rect {
                x: 0,
                y: 0,
                w: 5120,
                h: 1440
            })
        );
    }

    #[test]
    fn two_column_with_no_main_windows_reserved() {
        let (main, stack) = two_column(
            1,
            &CONTAINER,
            0,
            Size::Ratio(0.65),
            crate::geometry::Reserve::Reserve,
        );
        assert_eq!(main, None);
        assert_eq!(
            stack,
            Some(Rect {
                x: 3328,
                y: 0,
                w: 1792,
                h: 1440
            })
        );
    }

    #[test]
    fn two_column_with_no_stack_windows_reserved() {
        let (main, stack) = two_column(
            1,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::Reserve,
        );
        assert_eq!(
            main,
            Some(Rect {
                x: 0,
                y: 0,
                w: 3328,
                h: 1440
            })
        );
        assert_eq!(stack, None);
    }

    #[test]
    fn two_column_with_no_main_windows_reserved_and_centered() {
        let (main, stack) = two_column(
            1,
            &CONTAINER,
            0,
            Size::Ratio(0.65),
            crate::geometry::Reserve::ReserveAndCenter,
        );
        assert_eq!(main, None);
        assert_eq!(
            stack,
            Some(Rect {
                x: 1664,
                y: 0,
                w: 1792,
                h: 1440
            })
        );
    }

    #[test]
    fn two_column_with_no_stack_windows_reserved_and_centered() {
        let (main, stack) = two_column(
            1,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::ReserveAndCenter,
        );
        assert_eq!(
            main,
            Some(Rect {
                x: 896,
                y: 0,
                w: 3328,
                h: 1440
            })
        );
        assert_eq!(stack, None);
    }
}
