use crate::geometry::{Flip, Rect, Rotation, Split};
use std::{ops::Rem, vec};

use super::split::{dwindle, fibonacci, grid, horizontal, vertical};

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

/// Flip an array of [`Rect`] inside the container, according to the provided `flip` parameter
pub fn flip(rects: &mut [Rect], flip: Flip, container: &Rect) {
    if flip == Flip::None {
        return;
    }

    for rect in rects.iter_mut() {
        if flip.is_flipped_horizontal() {
            // from top edge as far away as bottom side was from bottom edge before being flipped
            let bottom_window_edge = rect.y + rect.h as i32;
            let bottom_container_edge = container.y + container.h as i32;
            rect.y = bottom_container_edge - bottom_window_edge;
        }
        if flip.is_flipped_vertical() {
            // from left edge as far away as right side is from right edge before being flipped
            let right_window_edge = rect.x + rect.w as i32;
            let right_container_edge = container.x + container.w as i32;
            rect.x = right_container_edge - right_window_edge;
        }
    }
}

/// Rotates an array of [`Rect`] inside the container, according to the provided `rotation` parameter.
///
/// Provided that the array has no gaps (i.e. pixels within the container that
/// belong to none of the [`Rect`] in the array), the result after applying this function won't
/// have gaps either. Similarly, if the array has no overlaps (i.e. pixels that are part of multiple [`Rect`]s
/// in the array), neither will the result.
pub fn rotate(rects: &mut [Rect], rotation: Rotation, container: &Rect) {
    if rotation == Rotation::North {
        return;
    }

    for rect in rects.iter_mut() {
        rotate_single_rect(rect, rotation, container);
    }

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

        // check whether rect "almost bounds" the container
        if rects[i].x + rects[i].w as i32 + 1 == container.x + container.w as i32 {
            wide_enough = false;
        }

        // check whether rect "almost bounds" the container
        if rects[i].y + rects[i].h as i32 + 1 == container.y + container.h as i32 {
            high_enough = false;
        }

        if !wide_enough && container.contains((rects[i].x + rects[i].w as i32 + 1, rects[i].y)) {
            rects[i].w += 1;
        }
        if !high_enough && container.contains((rects[i].x, rects[i].y + rects[i].h as i32 + 1)) {
            rects[i].h += 1;
        }
    }
}

fn rotate_single_rect(rect: &mut Rect, rotation: Rotation, container: &Rect) {
    // normalize so that Rect is at position (0/0)
    rect.x -= container.x;
    rect.y -= container.y;

    // rotate
    let next_anchor = rotation.next_anchor(rect);
    match rotation {
        Rotation::North => {}
        Rotation::East => {
            rect.x = container.h as i32 - next_anchor.1;
            rect.y = next_anchor.0;
            std::mem::swap(&mut rect.w, &mut rect.h);
        }
        Rotation::South => {
            let next_anchor = rotation.next_anchor(rect);
            rect.x = container.w as i32 - next_anchor.0;
            rect.y = container.h as i32 - next_anchor.1;
        }
        Rotation::West => {
            let next_anchor = rotation.next_anchor(rect);
            rect.x = next_anchor.1;
            rect.y = container.w as i32 - next_anchor.0;
            std::mem::swap(&mut rect.w, &mut rect.h);
        }
    }

    // new aspect ratio
    match rotation {
        Rotation::North | Rotation::South => {}
        Rotation::East | Rotation::West => {
            rect.x *= container.w as i32;
            rect.x /= container.h as i32;
            rect.y *= container.h as i32;
            rect.y /= container.w as i32;
            rect.w *= container.w;
            rect.w /= container.h;
            rect.h *= container.h;
            rect.h /= container.w;
        }
    }

    // revert normalization
    rect.x += container.x;
    rect.y += container.y;
}

/// Splits the provided [`Rect`] into smaller rectangles
/// according to the provided [`Split`].
///
/// ## Remainders
/// If a rectangle can not be split into even sizes that fill the whole original [`Rect`],
/// some of the resulting rectangles might be slightly bigger than others to account for the remaining space.
///
/// ie. When horizontally splitting a rectangle of 100px height into 3 pieces,
/// the resulting rectangles will be of the heights: 34px, 33px, and 33px.
/// The first rectangle being slightly taller to account for the remaining space that must be filled out.
///
/// The rectangles will differ by 1px at maximum. The remaining space of the division is
/// distributed evenly and by order accross the resulting rectangles, until no remaining space is left.
pub fn split(rect: &Rect, amount: usize, axis: Option<Split>) -> Vec<Rect> {
    match (amount, axis) {
        (0, _) => vec![],
        (_, None) => vec![*rect],
        (_, Some(a)) => match a {
            Split::Vertical => vertical(rect, amount),
            Split::Horizontal => horizontal(rect, amount),
            Split::Grid => grid(rect, amount),
            Split::Fibonacci => fibonacci(rect, amount),
            Split::Dwindle => dwindle(rect, amount),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::calc::{divrem, flip, remainderless_division, split},
        geometry::{Flip, Rect, Rotation, Split},
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
        let rects = split(&CONTAINER, 0, Some(Split::Vertical));
        assert_eq!(rects.len(), 0);
    }

    #[test]
    fn split_single_window() {
        let rects = split(&CONTAINER, 1, Some(Split::Vertical));
        assert_eq!(rects.len(), 1);
        assert!(rects[0].eq(&CONTAINER));
    }

    #[test]
    fn flip_none() {
        let container = Rect::new(0, 0, 400, 200);

        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect::new(0, 0, 400, 100),
            Rect::new(200, 100, 200, 100),
            Rect::new(0, 150, 200, 50),
            Rect::new(0, 100, 200, 50),
        ];

        flip(&mut rects, Flip::None, &container);

        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        assert_eq!(
            rects,
            vec![
                Rect::new(0, 0, 400, 100),
                Rect::new(200, 100, 200, 100),
                Rect::new(0, 150, 200, 50),
                Rect::new(0, 100, 200, 50),
            ]
        );
    }

    #[test]
    fn flip_horizontal() {
        let container = Rect::new(0, 0, 400, 200);

        // +---------------+
        // |               |
        // +-------+-------+
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect::new(0, 0, 400, 100),
            Rect::new(200, 100, 200, 100),
            Rect::new(0, 150, 200, 50),
            Rect::new(0, 100, 200, 50),
        ];

        flip(&mut rects, Flip::Horizontal, &container);

        // +-------+-------+
        // +-------+       |
        // +-------+-------+
        // |               |
        // +-------+-------+
        assert_eq!(
            rects,
            vec![
                Rect::new(0, 100, 400, 100),
                Rect::new(200, 0, 200, 100),
                Rect::new(0, 0, 200, 50),
                Rect::new(0, 50, 200, 50),
            ]
        );
    }

    #[test]
    fn flip_vertical() {
        let container = Rect::new(0, 0, 400, 200);

        // +---------------+
        // |               |
        // +-------+-------+
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect::new(0, 0, 400, 100),
            Rect::new(200, 100, 200, 100),
            Rect::new(0, 150, 200, 50),
            Rect::new(0, 100, 200, 50),
        ];

        flip(&mut rects, Flip::Vertical, &container);

        // +---------------+
        // |               |
        // +-------+-------+
        // |       +-------+
        // +-------+-------+
        assert_eq!(
            rects,
            vec![
                Rect::new(0, 0, 400, 100),
                Rect::new(0, 100, 200, 100),
                Rect::new(200, 150, 200, 50),
                Rect::new(200, 100, 200, 50),
            ]
        );
    }

    #[test]
    fn flip_both() {
        let container = Rect::new(0, 0, 400, 200);

        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect::new(0, 0, 400, 100),
            Rect::new(200, 100, 200, 100),
            Rect::new(0, 150, 200, 50),
            Rect::new(0, 100, 200, 50),
        ];

        flip(&mut rects, Flip::Both, &container);

        // +-------+-------+
        // |       +-------+
        // +-------+-------+  0°
        // |               |
        // +---------------+
        assert_eq!(
            rects,
            vec![
                Rect::new(0, 100, 400, 100),
                Rect::new(0, 0, 200, 100),
                Rect::new(200, 0, 200, 50),
                Rect::new(200, 50, 200, 50),
            ]
        );
    }

    #[test]
    fn rotate_0_degrees() {
        let container = Rect::new(0, 0, 400, 200);

        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect::new(0, 0, 400, 100),
            Rect::new(200, 100, 200, 100),
            Rect::new(0, 150, 200, 50),
            Rect::new(0, 100, 200, 50),
        ];

        rotate(&mut rects, Rotation::North, &container);

        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        assert_eq!(
            rects,
            vec![
                Rect::new(0, 0, 400, 100),
                Rect::new(200, 100, 200, 100),
                Rect::new(0, 150, 200, 50),
                Rect::new(0, 100, 200, 50),
            ]
        );
    }

    #[test]
    fn rotate_90_degrees() {
        let container = Rect::new(0, 0, 400, 200);

        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect::new(0, 0, 400, 100),
            Rect::new(200, 100, 200, 100),
            Rect::new(0, 150, 200, 50),
            Rect::new(0, 100, 200, 50),
        ];

        rotate(&mut rects, Rotation::East, &container);

        // +---+---+-------+
        // |   |   |       |
        // +---+---+       |  90°
        // |       |       |
        // +-------+-------+
        assert_eq!(
            rects,
            vec![
                Rect::new(200, 0, 200, 200),
                Rect::new(0, 100, 200, 100),
                Rect::new(0, 0, 100, 100),
                Rect::new(100, 0, 100, 100),
            ]
        );
    }

    #[test]
    fn rotate_180_degrees() {
        let container = Rect::new(0, 0, 400, 200);

        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect::new(0, 0, 400, 100),
            Rect::new(200, 100, 200, 100),
            Rect::new(0, 150, 200, 50),
            Rect::new(0, 100, 200, 50),
        ];

        rotate(&mut rects, Rotation::South, &container);

        // +-------+-------+
        // |       +-------+
        // +-------+-------+  180°
        // |               |
        // +---------------+
        assert_eq!(
            rects,
            vec![
                Rect::new(0, 100, 400, 100),
                Rect::new(0, 0, 200, 100),
                Rect::new(200, 0, 200, 50),
                Rect::new(200, 50, 200, 50),
            ]
        );
    }

    #[test]
    fn rotate_270_degrees() {
        let container = Rect::new(0, 0, 400, 200);

        // +---------------+
        // |               |
        // +-------+-------+  0°
        // +-------+       |
        // +-------+-------+
        let mut rects = vec![
            Rect::new(0, 0, 400, 100),
            Rect::new(200, 100, 200, 100),
            Rect::new(0, 150, 200, 50),
            Rect::new(0, 100, 200, 50),
        ];

        rotate(&mut rects, Rotation::West, &container);

        // +-------+-------+
        // |       |       |
        // |       +---+---+  270°
        // |       |   |   |
        // +-------+---+---+
        assert_eq!(
            rects,
            vec![
                Rect::new(0, 0, 200, 200),
                Rect::new(200, 0, 200, 100),
                Rect::new(300, 100, 100, 100),
                Rect::new(200, 100, 100, 100),
            ]
        );
    }

    #[test]
    fn rotate_0_degrees_with_offset() {
        let container = Rect::new(200, 50, 400, 200);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---------------+
        // xxxxxxxx |               |
        // xxxxxxxx +-------+-------+  0°
        // xxxxxxxx +-------+       |
        // xxxxxxxx +-------+-------+
        let mut rects = vec![
            Rect::new(200, 50, 400, 100),
            Rect::new(400, 150, 200, 100),
            Rect::new(200, 200, 200, 50),
            Rect::new(200, 150, 200, 50),
        ];

        rotate(&mut rects, Rotation::North, &container);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---------------+
        // xxxxxxxx |               |
        // xxxxxxxx +-------+-------+  0°
        // xxxxxxxx +-------+       |
        // xxxxxxxx +-------+-------+
        assert_eq!(
            rects,
            vec![
                Rect::new(200, 50, 400, 100),
                Rect::new(400, 150, 200, 100),
                Rect::new(200, 200, 200, 50),
                Rect::new(200, 150, 200, 50),
            ]
        );
    }

    #[test]
    fn rotate_90_degrees_with_offset() {
        let container = Rect::new(200, 50, 400, 200);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---------------+
        // xxxxxxxx |               |
        // xxxxxxxx +-------+-------+  0°
        // xxxxxxxx +-------+       |
        // xxxxxxxx +-------+-------+
        let mut rects = vec![
            Rect::new(200, 50, 400, 100),
            Rect::new(400, 150, 200, 100),
            Rect::new(200, 200, 200, 50),
            Rect::new(200, 150, 200, 50),
        ];

        rotate(&mut rects, Rotation::East, &container);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---+---+-------+
        // xxxxxxxx |   |   |       |
        // xxxxxxxx +---+---+       |  90°
        // xxxxxxxx |       |       |
        // xxxxxxxx +-------+-------+
        assert_eq!(
            rects,
            vec![
                Rect::new(400, 50, 200, 200),
                Rect::new(200, 150, 200, 100),
                Rect::new(200, 50, 100, 100),
                Rect::new(300, 50, 100, 100),
            ]
        );
    }

    #[test]
    fn rotate_180_degrees_with_offset() {
        let container = Rect::new(200, 50, 400, 200);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---------------+
        // xxxxxxxx |               |
        // xxxxxxxx +-------+-------+  0°
        // xxxxxxxx +-------+       |
        // xxxxxxxx +-------+-------+
        let mut rects = vec![
            Rect::new(200, 50, 400, 100),
            Rect::new(400, 150, 200, 100),
            Rect::new(200, 200, 200, 50),
            Rect::new(200, 150, 200, 50),
        ];

        rotate(&mut rects, Rotation::South, &container);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +-------+-------+
        // xxxxxxxx |       +-------+
        // xxxxxxxx +-------+-------+  180°
        // xxxxxxxx |               |
        // xxxxxxxx +---------------+
        assert_eq!(
            rects,
            vec![
                Rect::new(200, 150, 400, 100),
                Rect::new(200, 50, 200, 100),
                Rect::new(400, 50, 200, 50),
                Rect::new(400, 100, 200, 50),
            ]
        );
    }

    #[test]
    fn rotate_270_degrees_with_offset() {
        let container = Rect::new(200, 50, 400, 200);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +---------------+
        // xxxxxxxx |               |
        // xxxxxxxx +-------+-------+  0°
        // xxxxxxxx +-------+       |
        // xxxxxxxx +-------+-------+
        let mut rects = vec![
            Rect::new(200, 50, 400, 100),
            Rect::new(400, 150, 200, 100),
            Rect::new(200, 200, 200, 50),
            Rect::new(200, 150, 200, 50),
        ];

        rotate(&mut rects, Rotation::West, &container);

        // xxxxxxxxxxxxxxxxxxxxxxxxxx
        // xxxxxxxx +-------+-------+
        // xxxxxxxx |       |       |
        // xxxxxxxx |       +---+---+  270°
        // xxxxxxxx |       |   |   |
        // xxxxxxxx +-------+---+---+
        assert_eq!(
            rects,
            vec![
                Rect::new(200, 50, 200, 200),
                Rect::new(400, 50, 200, 100),
                Rect::new(500, 150, 100, 100),
                Rect::new(400, 150, 100, 100),
            ]
        );
    }

    #[test]
    fn rotate_90_degrees_non_divisible() {
        let container = Rect::new(0, 0, 401, 100);

        // +---------------+
        // |         |     |
        // +         |     +  0°
        // +         |     |
        // +---------+-----+
        let mut rects = vec![Rect::new(0, 0, 201, 100), Rect::new(201, 0, 200, 100)];

        rotate(&mut rects, Rotation::East, &container);

        // +---------------+
        // |               |
        // +---------------|  90°
        // |               |
        // +---------------+
        assert_eq!(
            rects,
            vec![Rect::new(0, 0, 401, 50), Rect::new(0, 50, 401, 50)]
        );
    }

    // todo: test with negative offset
}
