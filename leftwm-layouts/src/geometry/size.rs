#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Size {
    Pixel(u32),
    Percentage(f32),
}

impl Size {
    pub fn into_absolute(self, whole: u32) -> u32 {
        match self {
            Size::Pixel(x) => x,
            Size::Percentage(x) => (whole as f32 * (x / 100.0)).floor() as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Size;

    #[test]
    fn absolute_size_into_absolute_stays_same() {
        let i = 256u32;
        let size = Size::Pixel(i);
        let absolute = size.into_absolute(1000);
        assert_eq!(absolute, i)
    }

    #[test]
    fn relative_size_into_absolute() {
        let size = Size::Percentage(60.0);
        let absolute = size.into_absolute(1000);
        assert_eq!(absolute, 600);
    }

    #[test]
    fn relative_size_into_absolute_is_floored() {
        let size = Size::Percentage(50.0);
        let absolute = size.into_absolute(33);
        assert_eq!(absolute, 16);
    }
}
