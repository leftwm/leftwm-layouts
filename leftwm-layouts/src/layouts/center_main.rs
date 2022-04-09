use crate::{geometry::Rect, Layout, LayoutModifiers};

use crate::layouts::columns;

/// ## Demonstration
/// 1 window
/// ```text
/// +-----------------------+
/// |                       |
/// |                       |
/// |           1           |
/// |                       |
/// |                       |
/// +-----------------------+
/// ```
/// 2 windows
/// ```text
/// +-----------+-----------+
/// |           |           |
/// |           |           |
/// |     2     |     1     |
/// |           |           |
/// |           |           |
/// +-----------+-----------+
/// ```
/// 3 windows
/// ```text
/// +-----+-----------+-----+
/// |     |           |     |
/// |     |           |     |
/// |  2  |     1     |  3  |
/// |     |           |     |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
/// 4 windows
/// ```text
/// +-----+-----------+-----+
/// |  2  |           |     |
/// |     |           |     |
/// +-----+     1     |  4  |
/// |  3  |           |     |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
/// 5 windows
/// ```text
/// +-----+-----------+-----+
/// |  2  |           |  4  |
/// |     |           |     |
/// +-----+     1     +-----+
/// |  3  |           |  5  |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
#[derive(Debug)]
pub struct CenterMain;

impl Layout for CenterMain {
    fn apply(
        &self,
        window_count: usize,
        container: Rect,
        modifiers: &LayoutModifiers,
    ) -> Vec<Rect> {
        columns::stack_main_stack(window_count, container, modifiers)
    }
}

mod tests {}
