use crate::{
    geometry::{Rect, SplitAxis},
    Layout, LayoutModifiers,
};

use super::basic::main_stack;

#[derive(Debug)]
pub struct Fibonacci;

impl Layout for Fibonacci {
    fn apply(
        &self,
        window_count: usize,
        container: Rect,
        modifiers: &LayoutModifiers,
    ) -> Vec<Rect> {

        main_stack::main_stack(
            window_count,
            container,
            modifiers.master_window_count,
            modifiers.master_width_percentage,
            SplitAxis::Vertical,
            SplitAxis::Fibonacci,
            modifiers.reserve_space,
        )
    }
}

mod tests {}
