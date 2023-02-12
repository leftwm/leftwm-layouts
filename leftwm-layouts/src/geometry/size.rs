use serde::{Deserialize, Serialize};

/// Helper enum to represent a size which can be
/// an absolute pixel value or a relative ratio value
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Size {
    /// Size in pixels (ie. 10 means 10 pixels)
    Pixel(i32),

    /// Relative size as a ratio between 0 to 1 (ie. 0.5 means 50%)
    Ratio(f32),
}

impl Size {
    /// Turn the size into an absolute value.
    ///
    /// A pixel value will be returned as is, a ratio
    /// value will be multiplied by the provided
    /// `whole` to calculate the absolute value.
    ///
    /// ## Hint
    /// A negative ratio value will be converted into
    /// an absolute number before being applied.
    pub fn into_absolute(self, whole: u32) -> i32 {
        match self {
            Size::Pixel(x) => x,
            Size::Ratio(x) => (whole as f32 * x.abs()).round() as i32,
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
    fn relative_size_of_small_whole_is_rounded() {
        let size = Size::Ratio(0.6);
        let absolute = size.into_absolute(1);
        assert_eq!(absolute, 1);
    }

    #[test]
    fn relative_size_of_zero_doesnt_panic() {
        let size = Size::Ratio(0.6);
        let absolute = size.into_absolute(0);
        assert_eq!(absolute, 0);
    }

    #[test]
    fn negative_relative_size_must_return_same_as_positive() {
        let whole = 1000;
        let pos_ratio = Size::Ratio(0.6);
        let neg_ratio = Size::Ratio(-0.6);
        let pos_absolute = pos_ratio.into_absolute(whole);
        let neg_absolute = neg_ratio.into_absolute(whole);
        assert_eq!(pos_absolute, neg_absolute);
    }

    #[test]
    fn relative_size_into_absolute_is_rounded() {
        let size = Size::Ratio(0.5);
        let absolute = size.into_absolute(33);
        assert_eq!(absolute, 17);
    }
}
