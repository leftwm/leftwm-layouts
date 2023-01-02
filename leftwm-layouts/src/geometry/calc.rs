use crate::geometry::{Flipped, FloatRect, Rect, Rotation, SplitAxis};
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
        vec.push(val);
    }
    vec
}

/// Flip the given list of Rects according to the provided flipped parameter
pub fn flip(container: Rect, rects: &mut [Rect], flipped: &Flipped) {
    for rect in rects.iter_mut() {
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
    }
}

/// Rotates an array of `Rect`s within the smallest rectangle that contains them all
///
/// Provided that the array has no gaps (i.e. pixels within the outer rectangle that
/// belong to none of the `Rect`s in the array), the result after applying this function won't
/// have gaps either.
/// Similarly, if the array has no overlaps (i.e. pixels that are part of multiple `Rect`s
/// in the array), neither will the result.
pub fn rotate(rects: &mut [Rect], rotation: &Rotation) {
    let outer_rect = outer_rect(rects);

    // In this normalization, the outer Rect is a 1x1 rectangle at (0/0)
    let mut normalized_float_rects: Vec<FloatRect> = rects
        .iter()
        .map(|rect| {
            if outer_rect.w != 0 && outer_rect.h != 0 {
                FloatRect {
                    x: (rect.x - outer_rect.x) as f32 / outer_rect.w as f32,
                    y: (rect.y - outer_rect.y) as f32 / outer_rect.h as f32,
                    w: rect.w as f32 / outer_rect.w as f32,
                    h: rect.h as f32 / outer_rect.h as f32,
                }
            } else {
                // invisible rect might as well be at (0,0)
                FloatRect {
                    x: 0.0,
                    y: 0.0,
                    w: 0.0,
                    h: 0.0,
                }
            }
        })
        .collect();

    // Rotate normalized_float_rects
    for mut rect in &mut normalized_float_rects {
        let next_anchor = rotation.next_anchor(rect);
        match &rotation {
            Rotation::North => {}
            Rotation::East => {
                rect.x = 1.0 - next_anchor.1;
                rect.y = next_anchor.0;
                std::mem::swap(&mut rect.w, &mut rect.h);
            }
            Rotation::South => {
                rect.x = 1.0 - next_anchor.0;
                rect.y = 1.0 - next_anchor.1;
            }
            Rotation::West => {
                rect.x = next_anchor.1;
                rect.y = 1.0 - next_anchor.0;
                std::mem::swap(&mut rect.w, &mut rect.h);
            }
        }
    }

    // Revert the normalization and convert back to integer coordinates
    let new_rects: Vec<Rect> = normalized_float_rects
        .iter()
        .map(|rect| Rect {
            x: (rect.x * outer_rect.w as f32) as i32 + outer_rect.x,
            y: (rect.y * outer_rect.h as f32) as i32 + outer_rect.y,
            w: (rect.w * outer_rect.w as f32) as u32,
            h: (rect.h * outer_rect.h as f32) as u32,
        })
        .collect();

    // assign result to rects
    rects.copy_from_slice(&new_rects[..rects.len()]);

    // Fill missing pixels
    let n_rects = rects.len();
    for i in 0..n_rects {
        let mut wide_enough = true;
        let mut high_enough = true;

        // check whether rect "almost bounds" another rect
        for other in rects.iter() {
            if other != &rects[i]
                && !other.contains((rects[i].x + rects[i].w as i32, rects[i].y + 1))
                && other.contains((rects[i].x + rects[i].w as i32 + 1, rects[i].y + 1))
            {
                wide_enough = false;
            }
            if other != &rects[i]
                && !other.contains((rects[i].x + 1, rects[i].y + rects[i].h as i32))
                && other.contains((rects[i].x + 1, rects[i].y + rects[i].w as i32 + 1))
            {
                high_enough = false;
            }
        }

        // check whether rect "almost bounds" the outer rect
        if rects[i].x + rects[i].w as i32 + 1 == outer_rect.x + outer_rect.w as i32 {
            wide_enough = false;
        }

        // check whether rect "almost bounds" the outer rect
        if rects[i].y + rects[i].h as i32 + 1 == outer_rect.y + outer_rect.h as i32 {
            high_enough = false;
        }

        if !wide_enough && outer_rect.contains((rects[i].x + rects[i].w as i32 + 1, rects[i].y)) {
            rects[i].w += 1;
        }
        if !high_enough && outer_rect.contains((rects[i].x, rects[i].y + rects[i].h as i32 + 1)) {
            rects[i].h += 1;
        }
    }
}

fn outer_rect(rects: &[Rect]) -> Rect {
    Rect {
        x: min_x(rects),
        y: min_y(rects),
        w: (max_x(rects) - min_x(rects)) as u32,
        h: (max_y(rects) - min_y(rects)) as u32,
    }
}

fn min_x(rects: &[Rect]) -> i32 {
    rects.iter().map(|rect| rect.x).min().unwrap_or(0)
}
fn min_y(rects: &[Rect]) -> i32 {
    rects.iter().map(|rect| rect.y).min().unwrap_or(0)
}
fn max_x(rects: &[Rect]) -> i32 {
    rects
        .iter()
        .map(|rect| rect.x + rect.w as i32)
        .max()
        .unwrap_or(0)
}
fn max_y(rects: &[Rect]) -> i32 {
    rects
        .iter()
        .map(|rect| rect.y + rect.h as i32)
        .max()
        .unwrap_or(0)
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
            tiles.clone()
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
            tiles.clone()
        }
        &SplitAxis::None => {
            vec![*rect]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::calc::{divrem, remainderless_division, split},
        geometry::{Rect, SplitAxis},
    };

    use super::rotate;

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

    #[test]
    fn split_none_two_windows() {
        let rects = split(&CONTAINER, 2, &SplitAxis::None);
        assert_eq!(rects.len(), 1);
        assert!(rects[0].eq(&CONTAINER));
    }

    #[test]
    fn split_none_five_windows() {
        let rects = split(&CONTAINER, 2, &SplitAxis::None);
        assert_eq!(rects.len(), 1);
        assert!(rects[0].eq(&CONTAINER));
    }

    #[test]
    fn rotate_0_degrees() {
        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect {
                x: 0,
                y: 0,
                w: 400,
                h: 100,
            },
            Rect {
                x: 200,
                y: 100,
                w: 200,
                h: 100,
            },
            Rect {
                x: 0,
                y: 150,
                w: 200,
                h: 50,
            },
            Rect {
                x: 0,
                y: 100,
                w: 200,
                h: 50,
            },
        ];

        rotate(&mut rects, &crate::geometry::Rotation::North);

        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        assert_eq!(
            rects,
            vec![
                Rect {
                    x: 0,
                    y: 0,
                    w: 400,
                    h: 100
                },
                Rect {
                    x: 200,
                    y: 100,
                    w: 200,
                    h: 100
                },
                Rect {
                    x: 0,
                    y: 150,
                    w: 200,
                    h: 50
                },
                Rect {
                    x: 0,
                    y: 100,
                    w: 200,
                    h: 50
                },
            ]
        );
    }

    #[test]
    fn rotate_90_degrees() {
        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect {
                x: 0,
                y: 0,
                w: 400,
                h: 100,
            },
            Rect {
                x: 200,
                y: 100,
                w: 200,
                h: 100,
            },
            Rect {
                x: 0,
                y: 150,
                w: 200,
                h: 50,
            },
            Rect {
                x: 0,
                y: 100,
                w: 200,
                h: 50,
            },
        ];

        rotate(&mut rects, &crate::geometry::Rotation::East);

        // +---+---+-------+
        // |   |   |       |
        // +---+---+       |  90°
        // |       |       |
        // +-------+-------+
        assert_eq!(
            rects,
            vec![
                Rect {
                    x: 200,
                    y: 0,
                    w: 200,
                    h: 200
                },
                Rect {
                    x: 0,
                    y: 100,
                    w: 200,
                    h: 100
                },
                Rect {
                    x: 0,
                    y: 0,
                    w: 100,
                    h: 100
                },
                Rect {
                    x: 100,
                    y: 0,
                    w: 100,
                    h: 100
                },
            ]
        );
    }

    #[test]
    fn rotate_180_degrees() {
        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect {
                x: 0,
                y: 0,
                w: 400,
                h: 100,
            },
            Rect {
                x: 200,
                y: 100,
                w: 200,
                h: 100,
            },
            Rect {
                x: 0,
                y: 150,
                w: 200,
                h: 50,
            },
            Rect {
                x: 0,
                y: 100,
                w: 200,
                h: 50,
            },
        ];

        rotate(&mut rects, &crate::geometry::Rotation::South);

        // +-------+-------+
        // |       +-------+
        // +-------+-------+  180°
        // |               |
        // +---------------+
        assert_eq!(
            rects,
            vec![
                Rect {
                    x: 0,
                    y: 100,
                    w: 400,
                    h: 100
                },
                Rect {
                    x: 0,
                    y: 0,
                    w: 200,
                    h: 100
                },
                Rect {
                    x: 200,
                    y: 0,
                    w: 200,
                    h: 50
                },
                Rect {
                    x: 200,
                    y: 50,
                    w: 200,
                    h: 50
                },
            ]
        );
    }

    #[test]
    fn rotate_270_degrees() {
        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect {
                x: 0,
                y: 0,
                w: 400,
                h: 100,
            },
            Rect {
                x: 200,
                y: 100,
                w: 200,
                h: 100,
            },
            Rect {
                x: 0,
                y: 150,
                w: 200,
                h: 50,
            },
            Rect {
                x: 0,
                y: 100,
                w: 200,
                h: 50,
            },
        ];

        rotate(&mut rects, &crate::geometry::Rotation::West);

        // +-------+-------+
        // |       |       |
        // |       +---+---+  270°
        // |       |   |   |
        // +-------+---+---+
        assert_eq!(
            rects,
            vec![
                Rect {
                    x: 0,
                    y: 0,
                    w: 200,
                    h: 200
                },
                Rect {
                    x: 200,
                    y: 0,
                    w: 200,
                    h: 100
                },
                Rect {
                    x: 300,
                    y: 100,
                    w: 100,
                    h: 100
                },
                Rect {
                    x: 200,
                    y: 100,
                    w: 100,
                    h: 100
                },
            ]
        );
    }

    #[test]
    fn rotate_0_degrees_with_offset() {
        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---------------+
        // xxxxxxxx |               |
        // xxxxxxxx +-------+-------+  0°
        // xxxxxxxx +-------+       |
        // xxxxxxxx +-------+-------+
        let mut rects = vec![
            Rect {
                x: 200,
                y: 50,
                w: 400,
                h: 100,
            },
            Rect {
                x: 400,
                y: 150,
                w: 200,
                h: 100,
            },
            Rect {
                x: 200,
                y: 200,
                w: 200,
                h: 50,
            },
            Rect {
                x: 200,
                y: 150,
                w: 200,
                h: 50,
            },
        ];

        rotate(&mut rects, &crate::geometry::Rotation::North);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---------------+
        // xxxxxxxx |               |
        // xxxxxxxx +-------+-------+  0°
        // xxxxxxxx +-------+       |
        // xxxxxxxx +-------+-------+
        assert_eq!(
            rects,
            vec![
                Rect {
                    x: 200,
                    y: 50,
                    w: 400,
                    h: 100
                },
                Rect {
                    x: 400,
                    y: 150,
                    w: 200,
                    h: 100
                },
                Rect {
                    x: 200,
                    y: 200,
                    w: 200,
                    h: 50
                },
                Rect {
                    x: 200,
                    y: 150,
                    w: 200,
                    h: 50
                },
            ]
        );
    }

    #[test]
    fn rotate_90_degrees_with_offset() {
        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---------------+
        // xxxxxxxx |               |
        // xxxxxxxx +-------+-------+  0°
        // xxxxxxxx +-------+       |
        // xxxxxxxx +-------+-------+
        let mut rects = vec![
            Rect {
                x: 200,
                y: 50,
                w: 400,
                h: 100,
            },
            Rect {
                x: 400,
                y: 150,
                w: 200,
                h: 100,
            },
            Rect {
                x: 200,
                y: 200,
                w: 200,
                h: 50,
            },
            Rect {
                x: 200,
                y: 150,
                w: 200,
                h: 50,
            },
        ];

        rotate(&mut rects, &crate::geometry::Rotation::East);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---+---+-------+
        // xxxxxxxx |   |   |       |
        // xxxxxxxx +---+---+       |  90°
        // xxxxxxxx |       |       |
        // xxxxxxxx +-------+-------+
        assert_eq!(
            rects,
            vec![
                Rect {
                    x: 400,
                    y: 50,
                    w: 200,
                    h: 200
                },
                Rect {
                    x: 200,
                    y: 150,
                    w: 200,
                    h: 100
                },
                Rect {
                    x: 200,
                    y: 50,
                    w: 100,
                    h: 100
                },
                Rect {
                    x: 300,
                    y: 50,
                    w: 100,
                    h: 100
                },
            ]
        );
    }

    #[test]
    fn rotate_180_degrees_with_offset() {
        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---------------+
        // xxxxxxxx |               |
        // xxxxxxxx +-------+-------+  0°
        // xxxxxxxx +-------+       |
        // xxxxxxxx +-------+-------+
        let mut rects = vec![
            Rect {
                x: 200,
                y: 50,
                w: 400,
                h: 100,
            },
            Rect {
                x: 400,
                y: 150,
                w: 200,
                h: 100,
            },
            Rect {
                x: 200,
                y: 200,
                w: 200,
                h: 50,
            },
            Rect {
                x: 200,
                y: 150,
                w: 200,
                h: 50,
            },
        ];

        rotate(&mut rects, &crate::geometry::Rotation::South);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +-------+-------+
        // xxxxxxxx |       +-------+
        // xxxxxxxx +-------+-------+  180°
        // xxxxxxxx |               |
        // xxxxxxxx +---------------+
        assert_eq!(
            rects,
            vec![
                Rect {
                    x: 200,
                    y: 150,
                    w: 400,
                    h: 100
                },
                Rect {
                    x: 200,
                    y: 50,
                    w: 200,
                    h: 100
                },
                Rect {
                    x: 400,
                    y: 50,
                    w: 200,
                    h: 50
                },
                Rect {
                    x: 400,
                    y: 100,
                    w: 200,
                    h: 50
                },
            ]
        );
    }

    #[test]
    fn rotate_270_degrees_with_offset() {
        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---------------+
        // xxxxxxxx |               |
        // xxxxxxxx +-------+-------+  0°
        // xxxxxxxx +-------+       |
        // xxxxxxxx +-------+-------+
        let mut rects = vec![
            Rect {
                x: 200,
                y: 50,
                w: 400,
                h: 100,
            },
            Rect {
                x: 400,
                y: 150,
                w: 200,
                h: 100,
            },
            Rect {
                x: 200,
                y: 200,
                w: 200,
                h: 50,
            },
            Rect {
                x: 200,
                y: 150,
                w: 200,
                h: 50,
            },
        ];

        rotate(&mut rects, &crate::geometry::Rotation::West);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +-------+-------+
        // xxxxxxxx |       |       |
        // xxxxxxxx |       +---+---+  270°
        // xxxxxxxx |       |   |   |
        // xxxxxxxx +-------+---+---+
        assert_eq!(
            rects,
            vec![
                Rect {
                    x: 200,
                    y: 50,
                    w: 200,
                    h: 200
                },
                Rect {
                    x: 400,
                    y: 50,
                    w: 200,
                    h: 100
                },
                Rect {
                    x: 500,
                    y: 150,
                    w: 100,
                    h: 100
                },
                Rect {
                    x: 400,
                    y: 150,
                    w: 100,
                    h: 100
                },
            ]
        );
    }

    #[test]
    fn rotate_90_degrees_non_divisible() {
        // +---------------+
        // |         |     |
        // +         |     +  0°
        // +         |     |
        // +---------+-----+
        let mut rects = vec![
            Rect {
                x: 0,
                y: 0,
                w: 201,
                h: 100,
            },
            Rect {
                x: 201,
                y: 0,
                w: 200,
                h: 100,
            },
        ];

        rotate(&mut rects, &crate::geometry::Rotation::East);

        // +---------------+
        // |               |
        // +---------------|  90°
        // |               |
        // +---------------+
        assert_eq!(
            rects,
            vec![
                Rect {
                    x: 0,
                    y: 0,
                    w: 401,
                    h: 50,
                },
                Rect {
                    x: 0,
                    y: 50,
                    w: 401,
                    h: 50,
                },
            ]
        );
    }

    // todo: test with negative offset
}
