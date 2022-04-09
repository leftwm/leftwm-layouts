use crate::{geometry::Rect, Layout, LayoutModifiers};

use crate::layouts::columns;

#[derive(Debug)]
pub struct MainAndVertStack;

impl Layout for MainAndVertStack {
    fn apply(
        &self,
        window_count: usize,
        container: Rect,
        modifiers: &LayoutModifiers,
    ) -> Vec<Rect> {
        columns::main_stack(window_count, container, modifiers)
    }
}

mod tests {}
