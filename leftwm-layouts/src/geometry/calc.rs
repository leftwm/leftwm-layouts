use crate::geometry::{Flipped, Rect, Rotation, SplitAxis};
use std::ops::Rem;

/// Divide the provided `a` by `b` and return the
/// result of the integer division as well as the remainder.
///
/// ## Example
/// ```rust
/// let result = leftwm_layouts::geometry::divrem(11, 3);
/// assert_eq!((3, 2), result);
/// ```
pub fn divrem(a: usize, b: usize) -> (usize, usize) {
    let division = a / b;
    let remainder = a.rem(b);
    (division, remainder)
}

/// Divide the provided `a` by `b` and prevent
/// remainders by distributing the remainder count
/// evenly across the results.
///
/// ## Example
/// ```rust
/// let result = leftwm_layouts::geometry::remainderless_division(11, 3);
/// assert_eq!(vec![4,4,3], result);
/// ```
pub fn remainderless_division(a: usize, b: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = vec![];
    let (div, mut rem) = divrem(a, b);
    for _ in 0..b {
        let val = if rem > 0 {
            rem -= 1;
            div + 1
        } else {
            div
        };
        vec.push(val)
    }
    vec
}

/// Flip the given list of Rects according to the provided flipped parameter
pub fn flip(container: Rect, rects: &mut [Rect], flipped: &Flipped) {
    rects.iter_mut().for_each(|rect| {
        if flipped.is_flipped_horizontal() {
            // from left edge as far away as right side is from right edge before being flipped
            let right_window_edge = rect.x + rect.w as i32;
            let right_container_edge = container.x + container.w as i32;
            rect.x = right_container_edge - right_window_edge;
        }
        if flipped.is_flipped_vertical() {
            // from top edge as far away as bottom side was from bottom edge before being flipped
            let bottom_window_edge = rect.y + rect.h as i32;
            let bottom_container_edge = container.y + container.h as i32;
            rect.y = bottom_container_edge - bottom_window_edge;
        }
    });
}

pub fn translate_rotation(container: Rect, rect: &mut Rect, rotation: &Rotation) {
    match &rotation {
        Rotation::North => {}
        Rotation::East => {
            let next_anchor = rotation.anchor(rect);
            let new_x = container.h - next_anchor.1;
            let new_y = next_anchor.0;
            rect.x = new_x as i32;
            rect.y = new_y as i32;
            std::mem::swap(&mut rect.w, &mut rect.h);
        }
        Rotation::South => {
            let next_anchor = rotation.anchor(rect);
            let new_x = container.w - next_anchor.0;
            let new_y = container.h - next_anchor.1;
            rect.x = new_x as i32;
            rect.y = new_y as i32;
        }
        Rotation::West => {
            let next_anchor = rotation.anchor(rect);
            let new_x = next_anchor.1;
            let new_y = container.w - next_anchor.0;
            rect.x = new_x as i32;
            rect.y = new_y as i32;
            std::mem::swap(&mut rect.w, &mut rect.h)
        }
    }
}

/// Splits the provided rectangle (`Rect`) into smaller rectangles
/// using the provided `SplitAxis`.
///
/// ## Remainders
/// If a rectangle can not be split into even sizes that fill the whole original rectangle,
/// some of the resulting rectangles might be slightly bigger than others to account for the remaining space.
///
/// ie. When horizontally splitting a rectangle of 100px height into 3 pieces,
/// the resulting rectangles will be of the heights: 34px, 33px, and 33px.
/// The first rectangle being slightly taller to account for the remaining space that must be filled out.
///
/// The rectangles will differ by 1px at maximum. The remaining space of the division is
/// distributed evenly and by order accross the resulting rectangles, until no remaining space is left.
pub fn split(rect: &Rect, amount: usize, axis: &SplitAxis) -> Vec<Rect> {
    if amount == 0 {
        return vec![];
    }
    match axis {
        SplitAxis::Vertical => {
            let mut from_left = rect.x;
            remainderless_division(rect.w as usize, amount)
                .iter()
                .map(|width| {
                    let rect = Rect::new(from_left, rect.y, *width as u32, rect.h);
                    from_left += *width as i32;
                    rect
                })
                .collect()
        }
        SplitAxis::Horizontal => {
            let mut from_top = rect.y;
            remainderless_division(rect.h as usize, amount)
                .iter()
                .map(|height| {
                    let rect = Rect::new(rect.x, from_top, rect.w, *height as u32);
                    from_top += *height as i32;
                    rect
                })
                .collect()
        }
        SplitAxis::Grid => {
            let cols = (amount as f64).sqrt().ceil() as usize;
            let col_tiles = split(rect, cols, &SplitAxis::Vertical);
            // the minimum amount of rows per column
            let min_rows = (amount as f64 / cols as f64).floor() as usize;
            // the amount of columns in which there are only the minimum amount of rows
            let min_row_amount = col_tiles.len() - divrem(amount, cols).1;

            col_tiles
                .iter()
                .enumerate()
                .flat_map(|(i, col_tile)| {
                    let rows = if i < min_row_amount {
                        min_rows
                    } else {
                        min_rows + 1
                    };
                    split(col_tile, rows, &SplitAxis::Horizontal)
                })
                .collect()
        }
        SplitAxis::Fibonacci => {
            let tiles: &mut Vec<Rect> = &mut Vec::new();
            let mut remaining_tile = *rect;
            let mut direction = Rotation::East;
            for i in 0..amount {
                let has_next = i < amount - 1;
                direction = direction.clockwise();
                if has_next {
                    let split_axis = match direction {
                        Rotation::North | Rotation::South => SplitAxis::Horizontal,
                        Rotation::East | Rotation::West => SplitAxis::Vertical,
                    };
                    let backwards = match direction {
                        Rotation::East | Rotation::South => false,
                        Rotation::West | Rotation::North => true,
                    };
                    let splitted_tiles = split(&remaining_tile, 2, &split_axis);
                    if backwards {
                        tiles.push(splitted_tiles[1]);
                        remaining_tile = splitted_tiles[0];
                    } else {
                        tiles.push(splitted_tiles[0]);
                        remaining_tile = splitted_tiles[1];
                    }
                } else {
                    tiles.push(remaining_tile);
                }
            }
            tiles.to_vec()
        }
        SplitAxis::Dwindle => {
            let tiles: &mut Vec<Rect> = &mut Vec::new();
            let mut remaining_tile = *rect;
            let mut last_axis = SplitAxis::Vertical;
            for i in 0..amount {
                let has_next = i < amount - 1;
                last_axis = if last_axis == SplitAxis::Vertical {
                    SplitAxis::Horizontal
                } else {
                    SplitAxis::Vertical
                };
                if has_next {
                    let splitted_tiles = split(&remaining_tile, 2, &last_axis);
                    tiles.push(splitted_tiles[0]);
                    remaining_tile = splitted_tiles[1];
                } else {
                    tiles.push(remaining_tile);
                }
            }
            tiles.to_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::calc::{divrem, remainderless_division, split},
        geometry::{Rect, SplitAxis},
    };

    #[test]
    fn divrem_100_by_3_gives_33_1() {
        let result = divrem(100, 3);
        assert_eq!(result, (33, 1));
    }

    #[test]
    fn divrem_500_by_3_gives_166_2() {
        let result = divrem(500, 3);
        assert_eq!(result, (166, 2));
    }

    #[test]
    fn remainderless_division_works_without_remainder() {
        let result = remainderless_division(9, 3);
        assert_eq!(vec![3, 3, 3], result);
    }

    #[test]
    fn remainderless_division_works_with_remainders() {
        let result = remainderless_division(5, 3);
        assert_eq!(vec![2, 2, 1], result);

        let result = remainderless_division(10, 3);
        assert_eq!(vec![4, 3, 3], result);

        let result = remainderless_division(29, 8);
        assert_eq!(vec![4, 4, 4, 4, 4, 3, 3, 3], result);
    }

    const CONTAINER: Rect = Rect {
        x: 0,
        y: 0,
        w: 400,
        h: 200,
    };

    #[test]
    fn split_by_zero() {
        let rects = split(&CONTAINER, 0, &SplitAxis::Vertical);
        assert_eq!(rects.len(), 0);
    }

    #[test]
    fn split_single_window() {
        let rects = split(&CONTAINER, 1, &SplitAxis::Vertical);
        assert_eq!(rects.len(), 1);
        assert!(rects[0].eq(&CONTAINER));
    }

    #[test]
    fn split_vertical_two_windows() {
        let rects = split(&CONTAINER, 2, &SplitAxis::Vertical);
        assert_eq!(rects.len(), 2);
        let expected_first = Rect {
            x: 0,
            y: 0,
            w: 200,
            h: 200,
        };
        let expected_second = Rect {
            x: 200,
            y: 0,
            w: 200,
            h: 200,
        };
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
    }

    #[test]
    fn split_vertical_three_windows() {
        let rects = split(&CONTAINER, 3, &SplitAxis::Vertical);
        assert_eq!(rects.len(), 3);
        // first window must be larger because of the remainderless division
        let expected_first = Rect {
            x: 0,
            y: 0,
            w: 134,
            h: 200,
        };
        let expected_second = Rect {
            x: 134,
            y: 0,
            w: 133,
            h: 200,
        };
        let expected_third = Rect {
            x: 267,
            y: 0,
            w: 133,
            h: 200,
        };
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
    }

    #[test]
    fn split_horizontal_two_windows() {
        let rects = split(&CONTAINER, 2, &SplitAxis::Horizontal);
        assert_eq!(rects.len(), 2);
        let expected_first = Rect {
            x: 0,
            y: 0,
            w: 400,
            h: 100,
        };
        let expected_second = Rect {
            x: 0,
            y: 100,
            w: 400,
            h: 100,
        };
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
    }

    #[test]
    fn split_horizontal_three_windows() {
        let rects = split(&CONTAINER, 3, &SplitAxis::Horizontal);
        assert_eq!(rects.len(), 3);
        // first two windows must be taller because of remainderless division
        let expected_first = Rect {
            x: 0,
            y: 0,
            w: 400,
            h: 67,
        };
        let expected_second = Rect {
            x: 0,
            y: 67,
            w: 400,
            h: 67,
        };
        let expected_third = Rect {
            x: 0,
            y: 134,
            w: 400,
            h: 66,
        };
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
    }

    #[test]
    fn split_grid_three_windows() {
        let rects = split(&CONTAINER, 3, &SplitAxis::Grid);
        assert_eq!(rects.len(), 3);
        let expected_first = Rect {
            x: 0,
            y: 0,
            w: 200,
            h: 200,
        };
        let expected_second = Rect {
            x: 200,
            y: 0,
            w: 200,
            h: 100,
        };
        let expected_third = Rect {
            x: 200,
            y: 100,
            w: 200,
            h: 100,
        };
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
    }

    #[test]
    fn split_grid_four_windows() {
        let rects = split(&CONTAINER, 4, &SplitAxis::Grid);
        assert_eq!(rects.len(), 4);
        let expected_first = Rect {
            x: 0,
            y: 0,
            w: 200,
            h: 100,
        };
        let expected_second = Rect {
            x: 0,
            y: 100,
            w: 200,
            h: 100,
        };
        let expected_third = Rect {
            x: 200,
            y: 0,
            w: 200,
            h: 100,
        };
        let expected_fourth = Rect {
            x: 200,
            y: 100,
            w: 200,
            h: 100,
        };
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
        assert!(rects[3].eq(&expected_fourth));
    }

    #[test]
    fn split_fibonacci_four_windows() {
        let rects = split(&CONTAINER, 4, &SplitAxis::Fibonacci);
        assert_eq!(rects.len(), 4);
        let expected_first = Rect {
            x: 0,
            y: 0,
            w: 400,
            h: 100,
        };
        let expected_second = Rect {
            x: 200,
            y: 100,
            w: 200,
            h: 100,
        };
        let expected_third = Rect {
            x: 0,
            y: 150,
            w: 200,
            h: 50,
        };
        let expected_fourth = Rect {
            x: 0,
            y: 100,
            w: 200,
            h: 50,
        };
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
        assert!(rects[3].eq(&expected_fourth));
    }

    #[test]
    fn split_fibonacci_five_windows() {
        let rects = split(&CONTAINER, 5, &SplitAxis::Fibonacci);
        assert_eq!(rects.len(), 5);
        let expected_first = Rect {
            x: 0,
            y: 0,
            w: 400,
            h: 100,
        };
        let expected_second = Rect {
            x: 200,
            y: 100,
            w: 200,
            h: 100,
        };
        let expected_third = Rect {
            x: 0,
            y: 150,
            w: 200,
            h: 50,
        };
        let expected_fourth = Rect {
            x: 0,
            y: 100,
            w: 100,
            h: 50,
        };
        let expected_fifth = Rect {
            x: 100,
            y: 100,
            w: 100,
            h: 50,
        };
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
        assert!(rects[3].eq(&expected_fourth));
        assert!(rects[4].eq(&expected_fifth));
    }

    #[test]
    fn split_dwindle_four_windows() {
        let rects = split(&CONTAINER, 4, &SplitAxis::Dwindle);
        assert_eq!(rects.len(), 4);
        let expected_first = Rect {
            x: 0,
            y: 0,
            w: 400,
            h: 100,
        };
        let expected_second = Rect {
            x: 0,
            y: 100,
            w: 200,
            h: 100,
        };
        let expected_third = Rect {
            x: 200,
            y: 100,
            w: 200,
            h: 50,
        };
        let expected_fourth = Rect {
            x: 200,
            y: 150,
            w: 200,
            h: 50,
        };
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
        assert!(rects[3].eq(&expected_fourth));
    }

    #[test]
    fn split_dwindle_five_windows() {
        let rects = split(&CONTAINER, 5, &SplitAxis::Dwindle);
        assert_eq!(rects.len(), 5);
        let expected_first = Rect {
            x: 0,
            y: 0,
            w: 400,
            h: 100,
        };
        let expected_second = Rect {
            x: 0,
            y: 100,
            w: 200,
            h: 100,
        };
        let expected_third = Rect {
            x: 200,
            y: 100,
            w: 200,
            h: 50,
        };
        let expected_fourth = Rect {
            x: 200,
            y: 150,
            w: 100,
            h: 50,
        };
        let expected_fifth = Rect {
            x: 300,
            y: 150,
            w: 100,
            h: 50,
        };
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
        assert!(rects[3].eq(&expected_fourth));
        assert!(rects[4].eq(&expected_fifth));
    }
}
