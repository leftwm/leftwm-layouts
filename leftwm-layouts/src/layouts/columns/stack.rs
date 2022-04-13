use crate::{
    geometry::{self, Rect},
    LayoutModifiers,
};

/// The stack column layout is just one single column.
/// Most of the LayoutModifiers will be ignored, as they don't apply
/// to a single-stack layout, the only modifier being applied is `first_stack_split`
/// to determine the SplitAxis for the stack.
///
/// ```txt
/// +-----------+
/// |           |
/// |   STACK   |
/// |           |
/// +-----------+
/// ```
#[allow(dead_code)]
pub fn stack(window_count: usize, container: Rect, modifiers: &LayoutModifiers) -> Vec<Rect> {
    geometry::split(&container, window_count, &modifiers.first_stack_split)
}

#[cfg(test)]
mod tests {}
