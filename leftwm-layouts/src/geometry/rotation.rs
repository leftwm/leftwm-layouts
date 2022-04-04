use super::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents the four different possibilities of rotation.
/// The hinted "degrees" of rotation imply clock-wise rotation.
pub enum Rotation {
    North, // 0° rotation (No rotation)
    East,  // 90° rotation
    South, // 180° rotation
    West,  // 270° rotation
}

impl Rotation {
    /// Returns whether the aspect ratio for the provided container
    /// Rect changes with the given rotation, "squeezing" the contents.
    pub fn squeezes(&self, container: &Rect) -> bool {
        // if the container is not a square, and the rotation is
        // 90° or 270°, then the aspect ratio changes
        container.h != container.w && matches!(self, Self::West | Self::East)
    }

    pub fn anchor(&self, rect: &Rect) -> (u32, u32) {
        match self {
            Self::North => (rect.x as u32, rect.y as u32),
            Self::East => (rect.x as u32, rect.y as u32 + rect.h),
            Self::South => (rect.x as u32 + rect.w, rect.y as u32 + rect.h),
            Self::West => (rect.x as u32 + rect.w, rect.y as u32),
        }
    }

    pub fn x_rel(&self, container: &Rect, rect: &Rect) -> f32 {
        match self {
            Rotation::North => container.w as f32 / rect.x as f32,
            Rotation::East => (container.h as f32 / rect.y as f32),
            Rotation::South => todo!(),
            Rotation::West => todo!(),
        }
    }

    pub fn y_rel(&self, container: &Rect, rect: &Rect) -> f32 {
        match self {
            Rotation::North => container.h as f32 / rect.y as f32,
            Rotation::East => todo!(),
            Rotation::South => todo!(),
            Rotation::West => todo!(),
        }
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Rotation::North
    }
}
