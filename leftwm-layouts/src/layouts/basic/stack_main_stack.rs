use crate::{
    geometry::{Rect, SplitAxis},
    Util,
};

fn stack_main_stack(
    window_count: usize,
    container: Rect,
    main_count: u8,
    main_split: SplitAxis,
    left_stack_split: SplitAxis,
    right_stack_split: SplitAxis,
    balance_stacks: bool, // true: divide windows evenly between stacks / false: first stack has one window, rest goes to second stack
    reserve_space: bool,
) -> Vec<Rect> {
    todo!()
}
