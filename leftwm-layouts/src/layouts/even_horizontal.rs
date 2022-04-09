use crate::{
    geometry::{Rect, SplitAxis},
    Layout, LayoutModifiers,
};

use crate::layouts::columns;

#[derive(Debug)]
pub struct EvenHorizontal;

impl Layout for EvenHorizontal {
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
                first_stack_split: SplitAxis::Vertical,
                ..*modifiers
            },
        )
    }
}

mod tests {}
