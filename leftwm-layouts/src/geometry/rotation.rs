use serde::{Deserialize, Serialize};

use super::{Float, Rect};

/// Represents the four different possibilities of rotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rotation {
    /// A rotation of 0° (ie. no rotation).
    /// This is the default value.
    ///
    /// ```txt
    ///    North
    /// +---------+
    /// |    ^    |
    /// |         |
    /// |         |
    /// +---------+
    ///      0°
    /// ```
    North,

    /// A rotation of 90° clockwise.
    ///
    /// ```txt
    ///    East
    /// +---------+
    /// |         |
    /// |        >|
    /// |         |
    /// +---------+
    ///     90°
    /// ```
    East,

    /// A rotation of 180°.
    ///
    /// ```txt
    ///    South
    /// +---------+
    /// |         |
    /// |         |
    /// |    v    |
    /// +---------+
    ///     180°
    /// ```
    South,

    /// A rotation of 270° clockwise.
    ///
    /// ```txt
    ///     West
    /// +---------+
    /// |         |
    /// |<        |
    /// |         |
    /// +---------+
    ///     270°
    /// ```
    West,
}

impl Rotation {
    /// Returns whether the aspect ratio of the provided
    /// Rect changes with the given rotation.
    pub fn aspect_ratio_changes(&self, rect: &Rect) -> bool {
        // if the rect is not a square, and the rotation is
        // 90° or 270°, then the aspect ratio changes
        rect.h != rect.w && matches!(self, Self::West | Self::East)
    }

    /// Returns the (x, y) coordinate of the point which will be
    /// the Rect's anchor after it is rotated.
    ///
    /// ## Explanation
    /// The anchor point of a [`FloatRect`] is usually the top-left (x,y).
    /// When a [`FloatRect`] is rotated inside a layout, then another corner
    /// of the [`FloatRect`] will become the new anchor point after the rotation.
    /// This method returns the current position of that corner.
    pub fn next_anchor(&self, rect: &Float) -> (f32, f32) {
        match self {
            Self::North => (rect.x, rect.y),                   // top-left
            Self::East => (rect.x, rect.y + rect.h),           // bottom-left
            Self::South => (rect.x + rect.w, rect.y + rect.h), // bottom-right
            Self::West => (rect.x + rect.w, rect.y),           // top-right
        }
    }

    /// Get the next rotation variant when rotating clockwise
    #[must_use]
    pub fn clockwise(&self) -> Self {
        match self {
            Rotation::North => Rotation::East,
            Rotation::East => Rotation::South,
            Rotation::South => Rotation::West,
            Rotation::West => Rotation::North,
        }
    }

    /// Get the next rotation variant when rotating counter clockwise
    #[must_use]
    pub fn counter_clockwise(&self) -> Self {
        match self {
            Rotation::North => Rotation::West,
            Rotation::West => Rotation::South,
            Rotation::South => Rotation::East,
            Rotation::East => Rotation::North,
        }
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Rotation::North
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Float, Rect};

    use super::Rotation;

    const SQUARE: Rect = Rect {
        x: 0,
        y: 0,
        w: 200,
        h: 200,
    };

    const RECTANGLE: Rect = Rect {
        x: 0,
        y: 0,
        w: 400,
        h: 200,
    };

    #[test]
    fn square_never_changes_aspect_ratio() {
        let rotations = vec![
            Rotation::North,
            Rotation::East,
            Rotation::South,
            Rotation::West,
        ];
        for rotation in rotations {
            assert!(!rotation.aspect_ratio_changes(&SQUARE))
        }
    }

    #[test]
    fn non_square_changes_aspect_ratio_east_west() {
        assert!(Rotation::East.aspect_ratio_changes(&RECTANGLE));
        assert!(Rotation::West.aspect_ratio_changes(&RECTANGLE));
    }

    #[test]
    fn non_square_doesnt_change_aspect_ratio_north_south() {
        assert!(!Rotation::North.aspect_ratio_changes(&RECTANGLE));
        assert!(!Rotation::South.aspect_ratio_changes(&RECTANGLE));
    }

    #[test]
    fn calc_anchor_north() {
        let rect = Float::new(0.0, 0.0, 1920.0, 1080.0);
        let anchor = Rotation::North.next_anchor(&rect);
        assert_eq!(anchor, (0.0, 0.0));
    }

    #[test]
    fn calc_anchor_east() {
        let rect = Float::new(0.0, 0.0, 1920.0, 1080.0);
        let anchor = Rotation::East.next_anchor(&rect);
        assert_eq!(anchor, (0.0, 1080.0));
    }

    #[test]
    fn calc_anchor_south() {
        let rect = Float::new(0.0, 0.0, 1920.0, 1080.0);
        let anchor = Rotation::South.next_anchor(&rect);
        assert_eq!(anchor, (1920.0, 1080.0));
    }

    #[test]
    fn calc_anchor_west() {
        let rect = Float::new(0.0, 0.0, 1920.0, 1080.0);
        let anchor = Rotation::West.next_anchor(&rect);
        assert_eq!(anchor, (1920.0, 0.0));
    }
}
