/// A `Rect` represents a rectangle with a position (`x`,`y`)
/// and dimensions (`w`: width, `h`: height).
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn surface_area(&self) -> u32 {
        self.w * self.h
    }

    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    /// Get the coordinate at the center of the `Rect`.
    /// 
    /// The center coordinate is rounded to the nearest integer
    /// and might not be at the exact center position.
    pub fn center(&self) -> (i32, i32) {
        let x = self.x + (self.w / 2) as i32;
        let y = self.y + (self.h / 2) as i32;
        (x, y)
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
