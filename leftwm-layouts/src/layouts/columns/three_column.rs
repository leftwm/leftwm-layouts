use std::cmp;

use crate::geometry::{remainderless_division, Rect, Reserve, Size};

pub fn three_column(
    window_count: usize,
    container: &Rect,
    main_window_count: usize,
    main_size: Size,
    reserve_column_space: Reserve,
    balance_stacks: bool,
) -> (Option<Rect>, Option<Rect>, Option<Rect>) {
    let main_window_count = cmp::min(main_window_count, window_count);
    let stack_window_count = window_count.saturating_sub(main_window_count);

    let (left_stack_window_count, right_stack_window_count) =
        match (stack_window_count, balance_stacks) {
            (1, _) => (1, 0),
            (2.., false) => (1, stack_window_count.saturating_sub(1)),
            (2.., true) => {
                let rems = remainderless_division(stack_window_count, 2);
                (rems[0], rems[1])
            }
            _ => (0, 0),
        };

    let main_has_windows = main_window_count > 0;
    let left_stack_has_windows = left_stack_window_count > 0;
    let right_stack_has_windows = right_stack_window_count > 0;

    // note: the right stack can't be reserved if the left stack isn't too
    let main_reserve = main_has_windows || reserve_column_space.is_reserved();
    let left_stack_reserve = left_stack_has_windows || reserve_column_space.is_reserved();
    let right_stack_reserve =
        left_stack_reserve && right_stack_has_windows || reserve_column_space.is_reserved();

    // note: the right stack must be empty too if the left stack is empty
    let main_empty = !main_has_windows && reserve_column_space.is_reserved();
    let left_stack_empty = !left_stack_has_windows && reserve_column_space.is_reserved();
    let right_stack_empty =
        left_stack_empty || !right_stack_has_windows && reserve_column_space.is_reserved();

    let main_width = match (main_reserve, left_stack_reserve) {
        (true, true) => main_size.into_absolute(container.w) as usize,
        (true, false) => container.w as usize,
        _ => 0,
    };
    let stack_width = container.w as usize - main_width;
    let left_stack_width = match (left_stack_reserve, right_stack_reserve) {
        (true, false) => stack_width,
        (true, true) => stack_width / 2,
        _ => 0,
    };
    let right_stack_width = if right_stack_reserve {
        stack_width - left_stack_width
    } else {
        0
    };

    let main_offset = match (reserve_column_space, left_stack_empty, right_stack_empty) {
        (Reserve::ReserveAndCenter, false, true) => left_stack_width + (right_stack_width / 2),
        (Reserve::ReserveAndCenter, true, _) => stack_width / 2,
        _ => left_stack_width,
    };
    let left_stack_offset = match (reserve_column_space, main_empty, right_stack_empty) {
        (Reserve::ReserveAndCenter, false, true) => right_stack_width / 2,
        (Reserve::ReserveAndCenter, true, false) => main_width / 2,
        (Reserve::ReserveAndCenter, true, true) => (main_width + right_stack_width) / 2,
        _ => 0,
    };
    let right_stack_offset = match (reserve_column_space, main_empty) {
        (Reserve::ReserveAndCenter, true) => (main_width / 2) + left_stack_width,
        _ => left_stack_width + main_width,
    };

    let main = if main_has_windows {
        Some(Rect {
            x: main_offset as i32,
            w: main_width as u32,
            ..*container
        })
    } else {
        None
    };

    let left_stack = if left_stack_has_windows {
        Some(Rect {
            x: left_stack_offset as i32,
            w: left_stack_width as u32,
            ..*container
        })
    } else {
        None
    };

    let right_stack = if right_stack_has_windows {
        Some(Rect {
            x: right_stack_offset as i32,
            w: right_stack_width as u32,
            ..*container
        })
    } else {
        None
    };

    (left_stack, main, right_stack)
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Rect, Size};

    use super::three_column;

    const CONTAINER: Rect = Rect {
        x: 0,
        y: 0,
        w: 5120,
        h: 1440,
    };

    #[test]
    fn three_column_with_filled_columns() {
        let (left_stack, main, right_stack) = three_column(
            3,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::None,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 0,
                y: 0,
                w: 896,
                h: 1440
            })
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
        assert_eq!(
            right_stack,
            Some(Rect {
                x: 4224,
                y: 0,
                w: 896,
                h: 1440
            })
        );
    }

    #[test]
    fn three_column_with_filled_columns_reserved() {
        let (left_stack, main, right_stack) = three_column(
            3,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::Reserve,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 0,
                y: 0,
                w: 896,
                h: 1440
            })
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
        assert_eq!(
            right_stack,
            Some(Rect {
                x: 4224,
                y: 0,
                w: 896,
                h: 1440
            })
        );
    }

    #[test]
    fn three_column_with_filled_columns_reserved_and_centered() {
        let (left_stack, main, right_stack) = three_column(
            3,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::ReserveAndCenter,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 0,
                y: 0,
                w: 896,
                h: 1440
            })
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
        assert_eq!(
            right_stack,
            Some(Rect {
                x: 4224,
                y: 0,
                w: 896,
                h: 1440
            })
        );
    }

    #[test]
    fn three_column_with_no_right_stack_unreserved() {
        let (left_stack, main, right_stack) = three_column(
            2,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::None,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 0,
                y: 0,
                w: 1792,
                h: 1440
            })
        );
        assert_eq!(
            main,
            Some(Rect {
                x: 1792,
                y: 0,
                w: 3328,
                h: 1440
            })
        );
        assert_eq!(right_stack, None);
    }

    #[test]
    fn three_column_with_no_right_stack_reserved() {
        let (left_stack, main, right_stack) = three_column(
            2,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::Reserve,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 0,
                y: 0,
                w: 896,
                h: 1440
            })
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
        assert_eq!(right_stack, None);
    }

    #[test]
    fn three_column_with_no_right_stack_reserved_and_centered() {
        let (left_stack, main, right_stack) = three_column(
            2,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::ReserveAndCenter,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 448,
                y: 0,
                w: 896,
                h: 1440
            })
        );
        assert_eq!(
            main,
            Some(Rect {
                x: 1344,
                y: 0,
                w: 3328,
                h: 1440
            })
        );
        assert_eq!(right_stack, None);
    }

    #[test]
    fn three_column_with_no_stack_unreserved() {
        let (left_stack, main, right_stack) = three_column(
            1,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::None,
            false,
        );
        assert_eq!(left_stack, None);
        assert_eq!(
            main,
            Some(Rect {
                x: 0,
                y: 0,
                w: 5120,
                h: 1440
            })
        );
        assert_eq!(right_stack, None);
    }

    #[test]
    fn three_column_with_no_stack_reserved() {
        let (left_stack, main, right_stack) = three_column(
            1,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::Reserve,
            false,
        );
        assert_eq!(left_stack, None);
        assert_eq!(
            main,
            Some(Rect {
                x: 896,
                y: 0,
                w: 3328,
                h: 1440
            })
        );
        assert_eq!(right_stack, None);
    }

    #[test]
    fn three_column_with_no_stack_reserved_and_centered() {
        let (left_stack, main, right_stack) = three_column(
            1,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::ReserveAndCenter,
            false,
        );
        assert_eq!(left_stack, None);
        assert_eq!(
            main,
            Some(Rect {
                x: 896,
                y: 0,
                w: 3328,
                h: 1440
            })
        );
        assert_eq!(right_stack, None);
    }

    #[test]
    fn three_column_with_no_main_two_stacks_unreserved() {
        let (left_stack, main, right_stack) = three_column(
            2,
            &CONTAINER,
            0,
            Size::Ratio(0.65),
            crate::geometry::Reserve::None,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 0,
                y: 0,
                w: 2560,
                h: 1440
            })
        );
        assert_eq!(main, None);
        assert_eq!(
            right_stack,
            Some(Rect {
                x: 2560,
                y: 0,
                w: 2560,
                h: 1440
            })
        );
    }

    #[test]
    fn three_column_with_no_main_two_stacks_reserved() {
        let (left_stack, main, right_stack) = three_column(
            2,
            &CONTAINER,
            0,
            Size::Ratio(0.65),
            crate::geometry::Reserve::Reserve,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 0,
                y: 0,
                w: 896,
                h: 1440
            })
        );
        assert_eq!(main, None);
        assert_eq!(
            right_stack,
            Some(Rect {
                x: 4224,
                y: 0,
                w: 896,
                h: 1440
            })
        );
    }

    #[test]
    fn three_column_with_no_main_two_stacks_reserved_and_centered() {
        let (left_stack, main, right_stack) = three_column(
            2,
            &CONTAINER,
            0,
            Size::Ratio(0.65),
            crate::geometry::Reserve::ReserveAndCenter,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 1664,
                y: 0,
                w: 896,
                h: 1440
            })
        );
        assert_eq!(main, None);
        assert_eq!(
            right_stack,
            Some(Rect {
                x: 2560,
                y: 0,
                w: 896,
                h: 1440
            })
        );
    }

    #[test]
    fn three_column_with_no_main_left_stacks_unreserved() {
        let (left_stack, main, right_stack) = three_column(
            1,
            &CONTAINER,
            0,
            Size::Ratio(0.65),
            crate::geometry::Reserve::None,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 0,
                y: 0,
                w: 5120,
                h: 1440
            })
        );
        assert_eq!(main, None);
        assert_eq!(right_stack, None);
    }

    #[test]
    fn three_column_with_no_main_left_stacks_reserved() {
        let (left_stack, main, right_stack) = three_column(
            1,
            &CONTAINER,
            0,
            Size::Ratio(0.65),
            crate::geometry::Reserve::Reserve,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 0,
                y: 0,
                w: 896,
                h: 1440
            })
        );
        assert_eq!(main, None);
        assert_eq!(right_stack, None);
    }

    #[test]
    fn three_column_with_no_main_left_stacks_reserved_and_centered() {
        let (left_stack, main, right_stack) = three_column(
            1,
            &CONTAINER,
            0,
            Size::Ratio(0.65),
            crate::geometry::Reserve::ReserveAndCenter,
            false,
        );
        assert_eq!(
            left_stack,
            Some(Rect {
                x: 2112,
                y: 0,
                w: 896,
                h: 1440
            })
        );
        assert_eq!(main, None);
        assert_eq!(right_stack, None);
    }

    #[test]
    fn three_column_with_no_windows() {
        let (left_stack, main, right_stack) = three_column(
            0,
            &CONTAINER,
            1,
            Size::Ratio(0.65),
            crate::geometry::Reserve::None,
            false,
        );
        assert_eq!(left_stack, None);
        assert_eq!(main, None);
        assert_eq!(right_stack, None);
    }
}
