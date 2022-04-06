use crate::{
    geometry::{Rect, SplitAxis},
    Util,
};

fn stack(window_count: usize, container: Rect, split: SplitAxis) -> Vec<Rect> {
    Util::split(&container, window_count, &split)
}
