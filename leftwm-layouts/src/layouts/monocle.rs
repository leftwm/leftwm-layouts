use crate::{geometry::Tile, Layout, LayoutModifiers};

#[derive(Debug)]
pub struct Monocle;

impl Layout for Monocle {
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Tile> {
        if window_count < 1 {
            return vec![];
        }
        vec![modifiers.container_size.to_owned()]
    }
}

#[cfg(test)]
mod tests {
    use crate::Monocle;
    use crate::{Layout, LayoutModifiers};

    #[test]
    fn monocle_returns_only_one_rect() {
        let rects = Monocle.apply(3, &LayoutModifiers::default());
        let present = rects.into_iter();
        assert_eq!(present.len(), 1);
    }
}
