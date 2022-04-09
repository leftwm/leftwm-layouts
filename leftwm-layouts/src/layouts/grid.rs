use crate::{
    geometry::{Rect, SplitAxis},
    Layout, LayoutModifiers,
};

use crate::layouts::columns;

#[derive(Debug)]
pub struct Grid;

impl Layout for Grid {
    fn apply(
        &self,
        window_count: usize,
        container: Rect,
        modifiers: &LayoutModifiers,
    ) -> Vec<Rect> {
        columns::stack(
            window_count,
            container,
            &LayoutModifiers {
                first_stack_split: SplitAxis::Grid,
                ..*modifiers
            },
        )
    }
}

mod tests {}
