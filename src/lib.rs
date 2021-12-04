use crate::geometry::Rect;
use crate::layouts::monocle::Monocle;

pub mod geometry;
pub mod layouts;

pub trait Layout {
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Option<Rect>>;
}

pub struct LayoutModifiers {
    pub container_size: Rect,
    pub master_width_percentage: f32,
    pub master_window_count: u8,
}

impl Default for LayoutModifiers {
    fn default() -> Self {
        Self { 
            container_size: Rect::default(), 
            master_width_percentage: Default::default(), 
            master_window_count: Default::default() 
        }
    }
}

#[derive(Debug)]
pub struct LayoutNotFoundError;
pub struct Layouts;
impl Layouts {
    pub fn get_layout(name: &str) -> Result<&impl Layout, LayoutNotFoundError> {
        match name {
            "Monocle" => Ok(&Monocle),
            _ => Err(LayoutNotFoundError),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LayoutModifiers, Layouts, Layout};

    #[test]
    fn test_monocle_layout() {
        let modifiers: LayoutModifiers = LayoutModifiers::default();
        let monocle = Layouts::get_layout("monocle").unwrap();
        let monocle_positions = monocle.apply(1, &modifiers);
        assert_eq!(monocle_positions.len(), 1);
    }
}
