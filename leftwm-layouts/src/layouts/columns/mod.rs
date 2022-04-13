mod main_stack;
mod stack;
mod stack_main_stack;

pub use main_stack::main_stack;
pub use stack::stack;
pub use stack_main_stack::stack_main_stack;

pub enum ColumnLayoutEnum {
    Stack,
    MainStack,
    StackMainStack,
}

pub fn apply(col: ColumnLayoutEnum) {}
