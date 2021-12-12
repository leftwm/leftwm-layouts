use crate::{Layout, geometry::Tile, LayoutModifiers};


#[derive(Debug)]
pub struct Monocle;

impl Layout for Monocle {
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Option<Tile>> {
        let mut vec: Vec<Option<Tile>> = Vec::new();
        vec.push(Some(modifiers.container_size.to_owned()));
        for _ in 1..window_count {
            vec.push(None);
        }
        vec
    }
}

mod tests {
    use crate::{Layout, LayoutModifiers, geometry::Tile};
    use crate::Monocle;

    #[test]
    fn monocle_returns_only_one_rect() {
        let rects = Monocle.apply(3, &LayoutModifiers::default());
        let present: Vec<Tile> = rects.into_iter().filter_map(|e| e).collect();
        assert_eq!(present.len(), 1);
    }
}