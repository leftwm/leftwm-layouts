use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Size {
    Pixel(i32),
    Ratio(f32),
}

impl Size {
    pub fn into_absolute(self, whole: u32) -> i32 {
        match self {
            Size::Pixel(x) => x,
            Size::Ratio(x) => (whole as f32 * x).floor() as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Size;

    #[test]
    fn absolute_size_into_absolute_stays_same() {
        let i = 256i32;
        let size = Size::Pixel(i);
        let absolute = size.into_absolute(1000);
        assert_eq!(absolute, i)
    }

    #[test]
    fn relative_size_into_absolute() {
        let size = Size::Ratio(0.6);
        let absolute = size.into_absolute(1000);
        assert_eq!(absolute, 600);
    }

    #[test]
    fn relative_size_into_absolute_is_floored() {
        let size = Size::Ratio(0.5);
        let absolute = size.into_absolute(33);
        assert_eq!(absolute, 16);
    }
}
