use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub h: i32, // QUESTION: why signed?
    pub w: i32, // QUESTION: why signed?
}

impl Rect {
    pub fn area_size(&self) -> i32 {
        self.h * self.w
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            h: 250,
            w: 500,
        }
    }
}

impl Add for Rect {
    type Output = i32;

    fn add(self, rhs: Self) -> Self::Output {
        self.area_size() + rhs.area_size()
    }
}