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
    ///
    /// +---------+
    /// | ^       |
    /// |         |
    /// |         |
    /// +---------+
    /// ```
    North,

    /// Search for neighbor starting from the right top of the current rect
    ///
    /// ```txt
    ///    East
    ///
    /// +---------+
    /// |       > |
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
    /// | V       |
    /// +---------+
    ///
    /// ```
    South,

    /// Search for neighbor starting from the left top of the current rect
    ///
    /// ```txt
    ///     West
    ///
    /// +---------+
    /// | <       |
    /// |         |
    /// |         |
    /// +---------+
    /// ```
    West,
}

// Find the north neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_north(rects: &[Rect], current: usize) -> Option<usize> {
    let Some(current_rect) = rects.get(current).or(None) else { return None };

    // We are all the way up, no neighbor available
    if current_rect.top_edge() <= 0 {
        return None;
    }

    let mut nearest_rect: Option<usize> = None;
    let mut min_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;

    for (i, r) in rects.iter().enumerate() {
        if r == current_rect || // skip current rect
        r.right_edge() - 1 < current_rect.left_edge() || // skip too right
        r.left_edge() + 1 > current_rect.right_edge() || // skip too left
        r.top_edge() + 1 > current_rect.bottom_edge()
        // skip too low
        {
            continue;
        }

        let x_distance = current_rect.left_edge() - r.right_edge();
        let y_distance = current_rect.top_edge() - r.bottom_edge();

        find_nearest_rect(
            &mut min_x,
            &mut min_y,
            &mut nearest_rect,
            x_distance,
            y_distance,
            i,
            true,
        );
    }

    nearest_rect
}

// Find the east neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_east(rects: &[Rect], current: usize, display_width: u32) -> Option<usize> {
    let Some(current_rect) = rects.get(current).or(None) else { return None };

    // We are all the way right, no neighbor available
    if current_rect.right_edge() >= display_width as i32 {
        return None;
    }

    let mut nearest_rect: Option<usize> = None;
    let mut min_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;

    for (i, r) in rects.iter().enumerate() {
        if r == current_rect || // skip current rect
        r.right_edge() - 1 < current_rect.right_edge() || // skip too left
        r.bottom_edge() - 1 < current_rect.top_edge() || // skip too high
        r.top_edge() + 1 > current_rect.bottom_edge()
        // skip too low
        {
            continue;
        }

        let x_distance = r.left_edge() - current_rect.right_edge();
        let y_distance = r.top_edge() - current_rect.bottom_edge();

        find_nearest_rect(
            &mut min_x,
            &mut min_y,
            &mut nearest_rect,
            x_distance,
            y_distance,
            i,
            false,
        );
    }

    nearest_rect
}

// Find the south neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_south(rects: &[Rect], current: usize, display_height: u32) -> Option<usize> {
    let Some(current_rect) = rects.get(current).or(None) else { return None };

    // We are at the bottom, no neighbor available
    if current_rect.y + current_rect.h as i32 >= display_height as i32 {
        return None;
    }

    let mut nearest_rect: Option<usize> = None;
    let mut min_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;

    for (i, r) in rects.iter().enumerate() {
        if r == current_rect || // skip current rect
        r.right_edge() - 1 < current_rect.left_edge() || // skip too left
        r.left_edge() + 1 > current_rect.right_edge() || // skip too right
        r.bottom_edge() - 1 < current_rect.top_edge()
        // skip too high
        {
            // skip current rect
            continue;
        }

        let x_distance = current_rect.left_edge() - r.right_edge();
        let y_distance = r.top_edge() - current_rect.bottom_edge();

        find_nearest_rect(
            &mut min_x,
            &mut min_y,
            &mut nearest_rect,
            x_distance,
            y_distance,
            i,
            true,
        );
    }

    nearest_rect
}

// Find the west neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_west(rects: &[Rect], current: usize) -> Option<usize> {
    let Some(current_rect) = rects.get(current).or(None) else { return None };

    // We are all the way left; no neighbor available
    if current_rect.left_edge() <= 0 {
        return None;
    }

    let mut nearest_rect: Option<usize> = None;
    let mut min_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;

    for (i, r) in rects.iter().enumerate() {
        if r == current_rect || // skip current rect
         r.left_edge() + 1 > current_rect.right_edge() || // skip too right
         r.bottom_edge() - 1 < current_rect.top_edge() || // skip too high
         r.top_edge() + 1 > current_rect.bottom_edge()
        // skip too low
        {
            // skip current rect
            continue;
        }

        let x_distance = current_rect.left_edge() - r.right_edge();
        let y_distance = r.top_edge() - current_rect.bottom_edge();

        find_nearest_rect(
            &mut min_x,
            &mut min_y,
            &mut nearest_rect,
            x_distance,
            y_distance,
            i,
            false,
        );
    }

    nearest_rect
}

// Find the nearest `Rect`. If updown is true, evaluate y_distance and then x_distance. If updown
// is false, evaluate x_distance and then y_distance.
fn find_nearest_rect(
    min_x: &mut Option<i32>,
    min_y: &mut Option<i32>,
    nearest_rect: &mut Option<usize>,
    x_distance: i32,
    y_distance: i32,
    index: usize,
    updown: bool,
) {
    if min_x.is_none() {
        *min_x = Some(x_distance);
        *nearest_rect = Some(index);
    }

    if min_y.is_none() {
        *min_y = Some(y_distance);
        *nearest_rect = Some(index);
    }

    if updown {
        if y_distance < min_y.unwrap() {
            // take the nearest up/down
            *min_y = Some(y_distance);
            *nearest_rect = Some(index);
        } else if y_distance == min_y.unwrap() && x_distance < min_x.unwrap() {
            // take the left most
            *min_x = Some(x_distance);
            *nearest_rect = Some(index);
        }
    } else if x_distance < min_x.unwrap() {
        // take the nearest left/right
        *min_x = Some(x_distance);
        *nearest_rect = Some(index);
    } else if x_distance == min_x.unwrap() && y_distance < min_y.unwrap() {
        // take the higher
        *min_y = Some(y_distance);
        *nearest_rect = Some(index);
    }
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
    // |+---+ +---+ +---+|
    // || 1 | | 6 | |   ||
    // |+---+ +---+ |   ||
    // |+---+       | 5 ||
    // || 2 |       |   ||
    // |+---+       +---+|
    // +-----------------+
    const ARRAY: [Rect; 7] = [
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
        Rect {
            x: 200,
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
        let res = Direction::find_neighbor(&ARRAY, 6, Direction::North, &CONTAINER);
        assert_eq!(res, Some(3));
    }

    #[test]
    fn east_neighbor() {
        let res = Direction::find_neighbor(&ARRAY, 0, Direction::East, &CONTAINER);
        assert_eq!(res, Some(3));
        let res = Direction::find_neighbor(&ARRAY, 1, Direction::East, &CONTAINER);
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor(&ARRAY, 2, Direction::East, &CONTAINER);
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor(&ARRAY, 3, Direction::East, &CONTAINER);
        assert_eq!(res, Some(4));
        let res = Direction::find_neighbor(&ARRAY, 4, Direction::East, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 5, Direction::East, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 6, Direction::East, &CONTAINER);
        assert_eq!(res, Some(5));
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
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor(&ARRAY, 4, Direction::South, &CONTAINER);
        assert_eq!(res, Some(5));
        let res = Direction::find_neighbor(&ARRAY, 5, Direction::South, &CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 6, Direction::South, &CONTAINER);
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
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor(&ARRAY, 6, Direction::West, &CONTAINER);
        assert_eq!(res, Some(1));
    }
}
