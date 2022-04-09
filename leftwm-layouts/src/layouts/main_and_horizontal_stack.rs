use crate::geometry::SplitAxis;
use crate::{geometry::Rect, Layout, LayoutModifiers};

use crate::layouts::columns;

#[derive(Debug)]
pub struct MainAndHorizontalStack;

impl Layout for MainAndHorizontalStack {
    fn apply(
        &self,
        window_count: usize,
        container: Rect,
        modifiers: &LayoutModifiers,
    ) -> Vec<Rect> {
        columns::main_stack(
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
