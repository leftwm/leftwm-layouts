use super::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents the four different possibilities of rotation.
/// The hinted "degrees" of rotation imply clock-wise rotation.
///
/// ## Demonstration
/// ```txt
///    North          East           South           West
/// +---------+    +---------+    +---------+    +---------+
/// |    ^    |    |         |    |         |    |         |
/// |         |    |        >|    |         |    |<        |
/// |         |    |         |    |    v    |    |         |
/// +---------+    +---------+    +---------+    +---------+
///      0°            90°            180°           270°
/// ```
pub enum Rotation {
    North,
    East,
    South,
    West,
}

impl Rotation {
    /// Returns whether the aspect ratio for the provided container
    /// Rect changes with the given rotation, "squeezing" the contents.
    pub fn aspect_ratio_changes(&self, container: &Rect) -> bool {
        // if the container is not a square, and the rotation is
        // 90° or 270°, then the aspect ratio changes
        container.h != container.w && matches!(self, Self::West | Self::East)
    }

    /// Returns the (x, y) coordinate of the point which will be
    /// the Rect's anchor when it is rotated.
    pub fn anchor(&self, rect: &Rect) -> (u32, u32) {
        match self {
            Self::North => (rect.x as u32, rect.y as u32), // top-left
            Self::East => (rect.x as u32, rect.y as u32 + rect.h), // bottom-left
            Self::South => (rect.x as u32 + rect.w, rect.y as u32 + rect.h), // bottom-right
            Self::West => (rect.x as u32 + rect.w, rect.y as u32), // top-right
        }
    }

    /// Get the next rotation when rotating clockwise
    pub fn clockwise(&self) -> Self {
        match self {
            Rotation::North => Rotation::East,
            Rotation::East => Rotation::South,
            Rotation::South => Rotation::West,
            Rotation::West => Rotation::North,
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
    use crate::geometry::Rect;

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
        let rect = Rect::new(0, 0, 1920, 1080);
        let anchor = Rotation::North.anchor(&rect);
        assert_eq!(anchor, (0, 0));
    }

    #[test]
    fn calc_anchor_east() {
        let rect = Rect::new(0, 0, 1920, 1080);
        let anchor = Rotation::East.anchor(&rect);
        assert_eq!(anchor, (0, 1080));
    }

    #[test]
    fn calc_anchor_south() {
        let rect = Rect::new(0, 0, 1920, 1080);
        let anchor = Rotation::South.anchor(&rect);
        assert_eq!(anchor, (1920, 1080));
    }

    #[test]
    fn calc_anchor_west() {
        let rect = Rect::new(0, 0, 1920, 1080);
        let anchor = Rotation::West.anchor(&rect);
        assert_eq!(anchor, (1920, 0));
    }
}
