use super::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents the four different possibilities of rotation.
/// The hinted "degrees" of rotation imply clock-wise rotation.
pub enum Rotation {
    North, // 0° rotation (No rotation)
    East, // 90° rotation
    South, // 180° rotation
    West, // 270° rotation
}

impl Rotation {
    /// Returns whether the aspect ratio for the provided container
    /// Rect changes with the given rotation, "squeezing" the contents.
    pub fn squeezes(&self, container: Rect) -> bool {
        // if the container is not a square, and the rotation is 
        // 90° or 270°, then the aspect ratio changes
        return container.h != container.w && matches!(self, Self::West | Self::East)
    }
}