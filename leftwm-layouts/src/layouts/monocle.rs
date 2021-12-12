use crate::{Layout, geometry::Tile, LayoutModifiers};


#[derive(Debug)]
pub struct Monocle;

impl Layout for Monocle {
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Tile> {
        if window_count < 1 { 
            return vec![] 
        }
        vec![modifiers.container_size.to_owned()]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Layout, LayoutModifiers, geometry::Tile};
    use crate::Monocle;

    #[test]
    fn monocle_returns_only_one_rect() {
        let rects = Monocle.apply(3, &LayoutModifiers::default());
        let present: Vec<Tile> = rects.into_iter().collect();
        assert_eq!(present.len(), 1);
    }
}