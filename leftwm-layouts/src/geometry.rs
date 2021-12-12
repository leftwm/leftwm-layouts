use std::ops::{Add, Rem};

use crate::Flipped;

#[derive(PartialEq)]
pub enum SplitAxis {
    Horizontal,
    Vertical,
    Both, // splits in a grid
}

impl SplitAxis {
    pub fn split_horizontally(&self) -> bool {
        self == &Self::Horizontal || self == &Self::Both
    }

    pub fn split_vertically(&self) -> bool {
        self == &Self::Vertical || self == &Self::Both
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub h: i32, // QUESTION: why signed?
    pub w: i32, // QUESTION: why signed?
}

impl Tile {
    pub fn surface_area(&self) -> i32 {
        self.h * self.w
    }

    pub fn center(&self) -> (i32, i32) {
        let x = self.x + (self.w / 2);
        let y = self.y + (self.h / 2);
        (x, y)
    }

    pub fn split(&self, amount: usize, axis: SplitAxis) -> Vec<Tile> {
        match axis {
            SplitAxis::Vertical => {
                let mut from_left = self.x;
                Util::remainderless_division(self.w as usize, amount)
                    .iter()
                    .map(|width| {
                        let tile = Tile {
                            x: from_left,
                            y: self.y,
                            h: self.h,
                            w: *width as i32,
                        };
                        from_left += *width as i32;
                        tile
                    })
                    .collect()
            }
            SplitAxis::Horizontal => {
                let mut from_top = self.y;
                Util::remainderless_division(self.h as usize, amount)
                    .iter()
                    .map(|height| {
                        let tile = Tile {
                            x: self.x,
                            y: from_top,
                            h: *height as i32,
                            w: self.w,
                        };
                        from_top += *height as i32;
                        tile
                    })
                    .collect()
            }
            SplitAxis::Both => {
                let cols = (amount as f64).sqrt().ceil() as usize;
                let col_tiles = self.split(cols, SplitAxis::Vertical);
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
                        col_tile.split(rows, SplitAxis::Horizontal)
                    })
                    .collect()
            }
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            h: 250,
            w: 500,
        }
    }
}

impl Add for Tile {
    type Output = i32;

    fn add(self, rhs: Self) -> Self::Output {
        self.surface_area() + rhs.surface_area()
    }
}

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

    /// Flip the given list of tiles according to the provided flipped parameter
    pub fn flip(container: Tile, tiles: &mut Vec<Tile>, flipped: &Flipped) {
        tiles.iter_mut().for_each(|tile| {
            if flipped.is_flipped_horizontal() {
                // from left edge as far away as right side is from right edge before being flipped
                let right_window_edge = tile.x + tile.w;
                let right_container_edge = container.x + container.w;
                tile.x = right_container_edge - right_window_edge;
            }
            if flipped.is_flipped_vertical() {
                // from top edge as far away as bottom side was from bottom edge before being flipped
                let bottom_window_edge = tile.y + tile.h;
                let bottom_container_edge = container.y + container.h;
                tile.y = bottom_container_edge - bottom_window_edge;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::Util;

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

    // todo: test the flip() method

    // todo: test split() method
}
