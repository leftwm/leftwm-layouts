use crate::{geometry::Rect, Layout, LayoutModifiers};

#[derive(Debug)]
pub struct Monocle;

impl Layout for Monocle {
    fn apply(
        &self,
        window_count: usize,
        container: Rect,
        _modifiers: &LayoutModifiers,
    ) -> Vec<Rect> {
        if window_count < 1 {
            return vec![];
        }
        vec![container.to_owned()]
    }
}

#[cfg(test)]
mod tests {
    use crate::{geometry::Rect, layouts::Monocle, Layout, LayoutModifiers};

    #[test]
    fn monocle_returns_only_one_rect() {
        let rects = Monocle.apply(3, Rect::default(), &LayoutModifiers::default());
        let present = rects.into_iter();
        assert_eq!(present.len(), 1);
    }
}
