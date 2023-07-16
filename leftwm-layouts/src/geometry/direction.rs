use super::Rect;

/// Represents the four different direction where we can search for a neighbor
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default]
    /// Search for neighbor starting from the top left of the current rect
    /// This is the default value.
    ///
    /// ```txt
    ///    North
    ///  ^
    ///  |
    /// +---------+
    /// |         |
    /// |         |
    /// |         |
    /// +---------+
    /// ```
    North,

    /// Search for neighbor starting from the right top of the current rect
    ///
    /// ```txt
    ///    East
    ///         ->
    /// +---------+
    /// |         |
    /// |         |
    /// |         |
    /// +---------+
    /// ```
    East,

    /// Search for neighbor starting from the bottom left of the current rect
    ///
    /// ```txt
    ///    South
    /// +---------+
    /// |         |
    /// |         |
    /// |         |
    /// +---------+
    ///  |
    ///  V
    /// ```
    South,

    /// Search for neighbor starting from the left top of the current rect
    ///
    /// ```txt
    ///     West
    ///  <-
    /// +---------+
    /// |         |
    /// |         |
    /// |         |
    /// +---------+
    /// ```
    West,
}

// Find the north neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_north(rects: &[Rect], current: usize) -> Option<usize> {
    let current_rect = rects.get(current).or(None).unwrap();
    // We are all the way up, no neighbor available
    if current_rect.y == 0 {
        return None;
    }

    for x in current_rect.x + 1..=current_rect.x + current_rect.w as i32 - 1 {
        for y in (0..=current_rect.y + 1).rev() {
            for (i, r) in rects.iter().enumerate() {
                if i != current && r.contains((x, y)) {
                    // found north neighbor
                    return Some(i);
                }
            }
        }
    }
    None
}

// Find the east neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_east(rects: &[Rect], current: usize, display_width: u32) -> Option<usize> {
    let current_rect = rects.get(current).or(None).unwrap();

    // We are all the way right, no neighbor available
    if current_rect.x + current_rect.w as i32 >= display_width as i32 {
        return None;
    }

    for y in current_rect.y + 1..=current_rect.y + current_rect.h as i32 - 1 {
        for x in current_rect.x + current_rect.w as i32 + 1..=display_width as i32 {
            for (i, r) in rects.iter().enumerate() {
                if i != current && r.contains((x, y)) {
                    // found east neighbor
                    return Some(i);
                }
            }
        }
    }
    None
}

// Find the south neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_south(rects: &[Rect], current: usize, display_height: u32) -> Option<usize> {
    let current_rect = rects.get(current).or( None).unwrap();

    // We are at the bottom, no neighbor available
    if current_rect.y + current_rect.h as i32 >= display_height as i32 {
        return None;
    }

    for x in current_rect.x + 1..=current_rect.x + current_rect.w as i32 - 1 {
        for y in current_rect.y + current_rect.h as i32..=display_height as i32 {
            for (i, r) in rects.iter().enumerate() {
                if i != current && r.contains((x, y)) {
                    // found south neighbor
                    return Some(i);
                }
            }
        }
    }

    None
}

// Find the west neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_west(rects: &[Rect], current: usize) -> Option<usize> {
    let current_rect = rects.get(current).or(None).unwrap();

    // We are all the way left; no neighbor available
    if current_rect.x <= 0 {
        return None;
    }

    for x in (0..=current_rect.x - 1).rev() {
        for y in current_rect.y + 1..=current_rect.y + current_rect.h as i32 - 1 {
            for (i, r) in rects.iter().enumerate() {
                if i != current && r.contains((x, y)) {
                    return Some(i);
                }
            }
        }
    }
    None
}

impl Direction {
    /// Find the neighbor in a given direction (`North`, `East`, `South`, `West`), starting from a
    /// given `Rect` identified by the index `current` in an array of [`Rect`]
    pub fn find_neighbor(
        rects: &[Rect],
        current: usize,
        direction: Direction,
        container: &Rect,
    ) -> Option<usize> {
        if current >= rects.len() {
            return None;
        }

        match direction {
            Direction::North => find_north(rects, current),
            Direction::East => find_east(rects, current, container.w),
            Direction::South => find_south(rects, current, container.h),
            Direction::West => find_west(rects, current),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Direction, Rect};

    const CONTAINER: Rect = Rect {
        x: 0,
        y: 0,
        w: 600,
        h: 600,
    };

    // Test layout
    // +-----------------+
    // |+---+ +---+ +---+|
    // || 0 | | 3 | | 4 ||
    // |+---+ +---+ +---+|
    // |+---+       +---+|
    // || 1 |       |   ||
    // |+---+       |   ||
    // |+---+       | 5 ||
    // || 2 |       |   ||
    // |+---+       +---+|
    // +-----------------+
    const ARRAY: [Rect; 6] = [
        Rect {
            x: 0,
            y: 0,
            w: 200,
            h: 200,
        },
        Rect {
            x: 0,
            y: 200,
            w: 200,
            h: 200,
        },
        Rect {
            x: 0,
            y: 400,
            w: 200,
            h: 200,
        },
        Rect {
            x: 200,
            y: 0,
            w: 200,
            h: 200,
        },
        Rect {
            x: 400,
            y: 0,
            w: 200,
            h: 200,
        },
        Rect {
            x: 400,
            y: 200,
            w: 200,
            h: 400,
        },
    ];

    #[test]
    fn north_neighbor() {
        let res = Direction::find_neighbor(&ARRAY, 0, Direction::North, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 1, Direction::North, &CONTAINER);
        assert_eq!(res, Some(0));
        let res = Direction::find_neighbor(&ARRAY, 2, Direction::North, &CONTAINER);
        assert_eq!(res, Some(1));
        let res = Direction::find_neighbor(&ARRAY, 3, Direction::North, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 4, Direction::North, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 5, Direction::North, &CONTAINER);
        assert_eq!(res, Some(4));
    }

    #[test]
    fn east_neighbor() {
        let res = Direction::find_neighbor(&ARRAY, 0, Direction::East, &CONTAINER);
        assert_eq!(res, Some(3));
        let res = Direction::find_neighbor(&ARRAY, 1, Direction::East, &CONTAINER);
        assert_eq!(res, Some(5));
        let res = Direction::find_neighbor(&ARRAY, 2, Direction::East, &CONTAINER);
        assert_eq!(res, Some(5));
        let res = Direction::find_neighbor(&ARRAY, 3, Direction::East, &CONTAINER);
        assert_eq!(res, Some(4));
        let res = Direction::find_neighbor(&ARRAY, 4, Direction::East, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 5, Direction::East, &CONTAINER);
        assert_eq!(res, None);
    }

    #[test]
    fn south_neighbor() {
        let res = Direction::find_neighbor(&ARRAY, 0, Direction::South, &CONTAINER);
        assert_eq!(res, Some(1));
        let res = Direction::find_neighbor(&ARRAY, 1, Direction::South, &CONTAINER);
        assert_eq!(res, Some(2));
        let res = Direction::find_neighbor(&ARRAY, 2, Direction::South, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 3, Direction::South, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 4, Direction::South, &CONTAINER);
        assert_eq!(res, Some(5));
        let res = Direction::find_neighbor(&ARRAY, 5, Direction::South, &CONTAINER);
        assert_eq!(res, None);
    }

    #[test]
    fn west_neighbor() {
        let res = Direction::find_neighbor(&ARRAY, 0, Direction::West, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 1, Direction::West, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 2, Direction::West, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 3, Direction::West, &CONTAINER);
        assert_eq!(res, Some(0));
        let res = Direction::find_neighbor(&ARRAY, 4, Direction::West, &CONTAINER);
        assert_eq!(res, Some(3));
        let res = Direction::find_neighbor(&ARRAY, 5, Direction::West, &CONTAINER);
        assert_eq!(res, Some(1));
    }
}
