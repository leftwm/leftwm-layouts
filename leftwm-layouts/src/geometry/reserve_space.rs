use std::cmp;

use super::{Rect, remainderless_division};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReserveColumnSpace {
    None,
    Reserve,
    ReserveAndCenter,
}

impl ReserveColumnSpace {
    pub fn is_reserved(&self) -> bool {
        match self {
            ReserveColumnSpace::None => false,
            ReserveColumnSpace::Reserve | ReserveColumnSpace::ReserveAndCenter => true,
        }
    }
}

impl Default for ReserveColumnSpace {
    fn default() -> Self {
        ReserveColumnSpace::None
    }
}

fn shift_offsets(
    full_width: u32,
    main_column: Rect,
    left_column: Option<Rect>,
    right_column: Option<Rect>,
    window_count: usize,
    main_window_count: usize
) {
    let main_count = cmp::min(window_count, main_window_count);
    let stack_count = window_count.saturating_sub(main_count);
    let mut free_space = 0u32;
    if main_count == 0 {
        free_space += main_column.w;
    }
    if let Some(left) = left_column {
        if stack_count == 0 {
            free_space += left.w
        }
    }
    if let Some(right) = right_column {
        if stack_count == 1 {
            free_space += right.w
        }
    }
    let left_right_space = remainderless_division(free_space as usize, 2);
    
}
