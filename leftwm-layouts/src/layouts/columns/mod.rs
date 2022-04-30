mod main_stack;
mod stack;
mod stack_main_stack;
mod three_column;
mod two_column;

pub use three_column::three_column;
pub use two_column::two_column;

pub use main_stack::main_stack;
pub use stack::stack;
pub use stack_main_stack::stack_main_stack;

/*pub enum ColumnLayoutEnum {
    Stack,
    MainStack,
    StackMainStack,
}

pub fn apply(
    window_count: usize,
    container: Rect,
    col: ColumnLayoutEnum,
    modifiers: &LayoutModifiers,
) -> Vec<Rect> {
    let rects = match col {
        ColumnLayoutEnum::Stack => stack(window_count, container, modifiers),
        ColumnLayoutEnum::MainStack => main_stack(window_count, container, modifiers),
        ColumnLayoutEnum::StackMainStack => stack_main_stack(window_count, container, modifiers),
    };

    rects
}

struct Column {
    offset: usize,
    width: usize,
    has_windows: bool,
}*/
