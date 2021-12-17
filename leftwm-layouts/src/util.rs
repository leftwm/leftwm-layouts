use std::ops::Rem;

use crate::geometry::{Flipped, Rect, SplitAxis};

pub struct Util;
impl Util {
    pub fn divrem(a: usize, b: usize) -> (usize, usize) {
        let division = a / b;
        let remainder = a.rem(b);
        (division, remainder)
    }

    pub fn remainderless_division(a: usize, b: usize) -> Vec<usize> {
        let mut vec: Vec<usize> = vec![];
        let (div, mut rem) = Util::divrem(a, b);
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
    pub fn flip(container: Rect, rects: &mut Vec<Rect>, flipped: &Flipped) {
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

    /// ## Remainders
    /// After an object is cut, the resulting smaller object might not be all of the exact same size.
    /// If an object can not be split into even sizes while still "filling" the whole space of the original object,
    /// some object might be slightly bigger to account for the remaining space.
    /// 
    /// ie. When horizontally splitting a rectangle of 100px height into 3 pieces,
    /// the resulting rectangle will be of height 34px, 33px, and 33px.
    /// 
    /// ## Example
    /// Splitting a rectangle into three smaller rectangles would look as follows.
    /// 
    /// ### Vertical
    /// Rectangle is split by `vertical` cuts.
    /// 
    /// ```
    /// +--------+      +--+--+--+
    /// |        |      |  |  |  |
    /// |        |      |  |  |  |
    /// |        |  =>  |  |  |  |
    /// |        |      |  |  |  |
    /// |        |      |  |  |  |
    /// +--------+      +--+--+--+
    /// ```
    /// 
    /// ### Horizontal
    /// Rectangle is split by `horizontal` cuts.
    /// 
    /// ```
    /// +--------+      +--------+
    /// |        |      |        |
    /// |        |      +--------+
    /// |        |  =>  |        |
    /// |        |      +--------+
    /// |        |      |        |
    /// +--------+      +--------+
    /// ```
    /// 
    /// ### Both
    /// ```
    /// +-------+      +---+---+
    /// |       |      |   |   |
    /// |       |      |   |   |
    /// |       |  =>  |   +---+
    /// |       |      |   |   |
    /// |       |      |   |   |
    /// +-------+      +---+---+
    /// ```
    pub fn split(rect: &Rect, amount: usize, axis: &SplitAxis) -> Vec<Rect> {
        match axis {
            SplitAxis::Vertical => {
                let mut from_left = rect.x;
                Util::remainderless_division(rect.w as usize, amount)
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
                Util::remainderless_division(rect.h as usize, amount)
                    .iter()
                    .map(|height| {
                        let rect = Rect::new(rect.x, from_top, rect.w, *height as u32);
                        from_top += *height as i32;
                        rect
                    })
                    .collect()
            }
            SplitAxis::Both => {
                let cols = (amount as f64).sqrt().ceil() as usize;
                let col_tiles = Util::split(rect, cols, &SplitAxis::Vertical);
                // the minimum amount of rows per column
                let min_rows = (amount as f64 / cols as f64).floor() as usize;
                // the amount of columns in which there are only the minimum amount of rows
                let min_row_amount = col_tiles.len() - Util::divrem(amount, cols).1;

                col_tiles
                    .iter()
                    .enumerate()
                    .flat_map(|(i, col_tile)| {
                        let rows = if i < min_row_amount {
                            min_rows
                        } else {
                            min_rows + 1
                        };
                        Util::split(col_tile, rows, &SplitAxis::Horizontal)
                    })
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Util;

    #[test]
    fn divrem_100_by_3_gives_33_1() {
        let result = Util::divrem(100, 3);
        assert_eq!(result, (33, 1));
    }

    #[test]
    fn divrem_500_by_3_gives_166_2() {
        let result = Util::divrem(500, 3);
        assert_eq!(result, (166, 2));
    }

    #[test]
    fn remainderless_division_works_without_remainder() {
        let result = Util::remainderless_division(9, 3);
        assert_eq!(vec![3, 3, 3], result);
    }

    #[test]
    fn remainderless_division_works_with_remainders() {
        let result = Util::remainderless_division(5, 3);
        assert_eq!(vec![2, 2, 1], result);

        let result = Util::remainderless_division(10, 3);
        assert_eq!(vec![4, 3, 3], result);

        let result = Util::remainderless_division(29, 8);
        assert_eq!(vec![4, 4, 4, 4, 4, 3, 3, 3], result);
    }
}
