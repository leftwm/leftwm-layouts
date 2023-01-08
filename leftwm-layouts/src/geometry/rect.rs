/// Represents a rectangle with a position ([`Rect::x`], [`Rect::y`])
/// and dimensions ([`Rect::w`], [`Rect::h`]).
///
/// ## Demonstration
/// ```txt
/// (x/y)
///   x-------. ^
///   |       | |
///   |       | | h
///   |       | |
///   '-------' v
///   <------->
///       w
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    /// X-Coordinate, can be negative
    pub x: i32,

    /// Y-Coordinate, can be negative
    pub y: i32,

    /// Width, can not be negative
    pub w: u32,

    /// Height, can not be negative
    pub h: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    /// Calculate the surface area of the `Rect`
    pub fn surface_area(&self) -> u32 {
        self.w * self.h
    }

    /// Get the coordinate at the center of the `Rect`.
    ///
    /// The center coordinate is rounded to the nearest integer
    /// and might not be at the exact center position.
    pub fn center(&self) -> (i32, i32) {
        let x = self.x + (self.w as f32 / 2.0).round() as i32;
        let y = self.y + (self.h as f32 / 2.0).round() as i32;
        (x, y)
    }

    /// Check whether a point is contained in a `Rect`.
    ///
    /// The boundary counts as part of the `Rect`.
    pub fn contains(&self, point: (i32, i32)) -> bool {
        self.x <= point.0
            && point.0 <= self.x + self.w as i32
            && self.y <= point.1
            && point.1 <= self.y + self.h as i32
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            w: 500,
            h: 250,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;

    #[test]
    fn surface_area_calculation() {
        let rect = Rect::new(0, 0, 1920, 1080);
        assert_eq!(rect.surface_area(), 2073600);
    }

    #[test]
    fn center_calculation() {
        let rect = Rect::new(0, 0, 1920, 1080);
        assert_eq!(rect.center(), (960, 540));
    }

    #[test]
    fn center_calculation_with_offset() {
        let rect = Rect::new(200, 120, 1920, 1080);
        assert_eq!(rect.center(), (1160, 660));
    }

    #[test]
    fn center_calculation_with_negative_offset() {
        let rect = Rect::new(-200, -120, 1920, 1080);
        assert_eq!(rect.center(), (760, 420));
    }

    #[test]
    fn center_calculation_at_rounded_position() {
        let rect = Rect::new(100, 100, 387, 399);
        assert_eq!(rect.center(), (294, 300))
    }

    #[test]
    fn contains_boundary() {
        let rect = Rect::new(100, 100, 400, 100);
        assert!(rect.contains((100, 100)));
        assert!(rect.contains((500, 100)));
        assert!(rect.contains((500, 200)));
        assert!(rect.contains((100, 200)));
    }

    #[test]
    fn does_not_contain_points_outside_rect() {
        let rect = Rect::new(100, 100, 400, 100);
        assert!(!rect.contains((99, 100)));
        assert!(!rect.contains((501, 100)));
        assert!(!rect.contains((501, 200)));
        assert!(!rect.contains((99, 200)));
        assert!(!rect.contains((100, 99)));
        assert!(!rect.contains((500, 99)));
        assert!(!rect.contains((500, 201)));
        assert!(!rect.contains((100, 201)));
    }
}
