use serde::{Deserialize, Serialize};

use super::{divrem, remainderless_division, split, Rect, Rotation};

/// Describes different ways a [`crate::geometry::Rect`] can be split.
///
/// *Disclaimer: As it may be confusing - The terms vertical/horizontal are referring to the "splits"
/// not the orientation of the resulting stack. For example, [`Split::Horizontal`]
/// splits a rect by **horizontal cuts**, resulting in a "vertically stacked" list of rects.
/// See the variants' documentation for clarification.*
#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Split {
    /// Rectangle is split by `horizontal` cuts.
    ///
    /// ```txt
    /// +--------+      +--------+
    /// |        |      |        |
    /// |        |      +--------+
    /// |        |  =>  |        |
    /// |        |      +--------+
    /// |        |      |        |
    /// +--------+      +--------+
    /// ```
    Horizontal,

    /// Rectangle is split by `vertical` cuts.
    ///
    /// ```txt
    /// +--------+      +--+--+--+
    /// |        |      |  |  |  |
    /// |        |      |  |  |  |
    /// |        |  =>  |  |  |  |
    /// |        |      |  |  |  |
    /// |        |      |  |  |  |
    /// +--------+      +--+--+--+
    /// ```
    Vertical,

    /// Rectangle is split in a "Grid" pattern while still accounting for
    /// all of the available space, resulting in some rectangles being larger.
    ///
    /// ```txt
    /// +-------+      +---+---+
    /// |       |      |   |   |
    /// |       |      |   |   |
    /// |       |  =>  |   +---+
    /// |       |      |   |   |
    /// |       |      |   |   |
    /// +-------+      +---+---+
    /// ```
    Grid,

    /// Rectangle is split in a "Fibonacci" pattern.
    ///
    /// ```txt
    /// +-------+      +---+---+
    /// |       |      |   |   |
    /// |       |      |   |   |
    /// |       |  =>  |   +-+-+
    /// |       |      |   |_| |
    /// |       |      |   | | |
    /// +-------+      +---+---+
    /// ```
    Fibonacci,

    /// Rectangle is split in a "Fibonacci"-like pattern.
    /// But instead of spiraling into the middle, it spirals into the bottom right.
    ///
    /// ```txt
    /// +-------+      +---+---+
    /// |       |      |   |   |
    /// |       |      |   |   |
    /// |       |  =>  |   +-+-+
    /// |       |      |   | |_|
    /// |       |      |   | |||
    /// +-------+      +---+---+
    /// ```
    Dwindle,
}

pub fn vertical(rect: &Rect, amount: usize) -> Vec<Rect> {
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

pub fn horizontal(rect: &Rect, amount: usize) -> Vec<Rect> {
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

pub fn grid(rect: &Rect, amount: usize) -> Vec<Rect> {
    let cols = (amount as f64).sqrt().ceil() as usize;
    let col_tiles = vertical(rect, cols);
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
            horizontal(col_tile, rows)
        })
        .collect()
}

pub fn fibonacci(rect: &Rect, amount: usize) -> Vec<Rect> {
    let tiles: &mut Vec<Rect> = &mut Vec::new();
    let mut remaining_tile = *rect;
    let mut direction = Rotation::East;
    for i in 0..amount {
        let has_next = i < amount - 1;
        direction = direction.clockwise();
        if has_next {
            let split_axis = match direction {
                Rotation::North | Rotation::South => Split::Horizontal,
                Rotation::East | Rotation::West => Split::Vertical,
            };
            let backwards = match direction {
                Rotation::East | Rotation::South => false,
                Rotation::West | Rotation::North => true,
            };
            let splitted_tiles = split(&remaining_tile, 2, Some(split_axis));
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

pub fn dwindle(rect: &Rect, amount: usize) -> Vec<Rect> {
    let tiles: &mut Vec<Rect> = &mut Vec::new();
    let mut remaining_tile = *rect;
    let mut last_axis = Split::Vertical;
    for i in 0..amount {
        let has_next = i < amount - 1;
        last_axis = if last_axis == Split::Vertical {
            Split::Horizontal
        } else {
            Split::Vertical
        };
        if has_next {
            let splitted_tiles = split(&remaining_tile, 2, Some(last_axis));
            tiles.push(splitted_tiles[0]);
            remaining_tile = splitted_tiles[1];
        } else {
            tiles.push(remaining_tile);
        }
    }
    tiles.clone()
}

#[cfg(test)]
mod tests {
    use crate::geometry::{
        split::{dwindle, fibonacci, grid, horizontal, vertical},
        Rect,
    };

    const CONTAINER: Rect = Rect {
        x: 0,
        y: 0,
        w: 400,
        h: 200,
    };

    #[test]
    fn split_vertical_two_windows() {
        let rects = vertical(&CONTAINER, 2);
        assert_eq!(rects.len(), 2);
        let expected_first = Rect::new(0, 0, 200, 200);
        let expected_second = Rect::new(200, 0, 200, 200);
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
    }

    #[test]
    fn split_vertical_three_windows() {
        let rects = vertical(&CONTAINER, 3);
        assert_eq!(rects.len(), 3);
        // first window must be larger because of the remainderless division
        let expected_first = Rect::new(0, 0, 134, 200);
        let expected_second = Rect::new(134, 0, 133, 200);
        let expected_third = Rect::new(267, 0, 133, 200);
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
    }

    #[test]
    fn split_horizontal_two_windows() {
        let rects = horizontal(&CONTAINER, 2);
        assert_eq!(rects.len(), 2);
        let expected_first = Rect::new(0, 0, 400, 100);
        let expected_second = Rect::new(0, 100, 400, 100);
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
    }

    #[test]
    fn split_horizontal_three_windows() {
        let rects = horizontal(&CONTAINER, 3);
        assert_eq!(rects.len(), 3);
        // first two windows must be taller because of remainderless division
        let expected_first = Rect::new(0, 0, 400, 67);
        let expected_second = Rect::new(0, 67, 400, 67);
        let expected_third = Rect::new(0, 134, 400, 66);
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
    }

    #[test]
    fn split_grid_three_windows() {
        let rects = grid(&CONTAINER, 3);
        assert_eq!(rects.len(), 3);
        let expected_first = Rect::new(0, 0, 200, 200);
        let expected_second = Rect::new(200, 0, 200, 100);
        let expected_third = Rect::new(200, 100, 200, 100);
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
    }

    #[test]
    fn split_grid_four_windows() {
        let rects = grid(&CONTAINER, 4);
        assert_eq!(rects.len(), 4);
        let expected_first = Rect::new(0, 0, 200, 100);
        let expected_second = Rect::new(0, 100, 200, 100);
        let expected_third = Rect::new(200, 0, 200, 100);
        let expected_fourth = Rect::new(200, 100, 200, 100);
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
        assert!(rects[3].eq(&expected_fourth));
    }

    #[test]
    fn split_fibonacci_four_windows() {
        let rects = fibonacci(&CONTAINER, 4);
        assert_eq!(rects.len(), 4);
        let expected_first = Rect::new(0, 0, 400, 100);
        let expected_second = Rect::new(200, 100, 200, 100);
        let expected_third = Rect::new(0, 150, 200, 50);
        let expected_fourth = Rect::new(0, 100, 200, 50);
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
        assert!(rects[3].eq(&expected_fourth));
    }

    #[test]
    fn split_fibonacci_five_windows() {
        let rects = fibonacci(&CONTAINER, 5);
        assert_eq!(rects.len(), 5);
        let expected_first = Rect::new(0, 0, 400, 100);
        let expected_second = Rect::new(200, 100, 200, 100);
        let expected_third = Rect::new(0, 150, 200, 50);
        let expected_fourth = Rect::new(0, 100, 100, 50);
        let expected_fifth = Rect::new(100, 100, 100, 50);
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
        assert!(rects[3].eq(&expected_fourth));
        assert!(rects[4].eq(&expected_fifth));
    }

    #[test]
    fn split_dwindle_four_windows() {
        let rects = dwindle(&CONTAINER, 4);
        assert_eq!(rects.len(), 4);
        let expected_first = Rect::new(0, 0, 400, 100);
        let expected_second = Rect::new(0, 100, 200, 100);
        let expected_third = Rect::new(200, 100, 200, 50);
        let expected_fourth = Rect::new(200, 150, 200, 50);
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
        assert!(rects[3].eq(&expected_fourth));
    }

    #[test]
    fn split_dwindle_five_windows() {
        let rects = dwindle(&CONTAINER, 5);
        assert_eq!(rects.len(), 5);
        let expected_first = Rect::new(0, 0, 400, 100);
        let expected_second = Rect::new(0, 100, 200, 100);
        let expected_third = Rect::new(200, 100, 200, 50);
        let expected_fourth = Rect::new(200, 150, 100, 50);
        let expected_fifth = Rect::new(300, 150, 100, 50);
        assert!(rects[0].eq(&expected_first));
        assert!(rects[1].eq(&expected_second));
        assert!(rects[2].eq(&expected_third));
        assert!(rects[3].eq(&expected_fourth));
        assert!(rects[4].eq(&expected_fifth));
    }
}
