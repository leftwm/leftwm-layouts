use std::ops::{Add, Rem};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub h: i32, // QUESTION: why signed?
    pub w: i32, // QUESTION: why signed?
}

impl Rect {
    pub fn surface_area(&self) -> i32 {
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
        self.surface_area() + rhs.surface_area()
    }
}

struct Util;
impl Util {
    pub fn divrem(a: i32, b: i32) -> (i32, i32) {
        let division = a / b;
        let remainder = a.rem(b);
        (division, remainder)
    }
}

mod tests {
    use super::Util;

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
}