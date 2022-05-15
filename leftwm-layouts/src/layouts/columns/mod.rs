mod main_stack;
mod stack;
mod stack_main_stack;
mod three_column;
mod two_column;

use serde::{Deserialize, Serialize};
use three_column::three_column;
use two_column::two_column;

pub use main_stack::main_stack;
pub use stack::stack;
pub use stack_main_stack::stack_main_stack;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ColumnType {
    Stack,
    MainAndStack,
    CenterMain,
}

// Stack (-)
// MainStack (flip -> StackMain)
// CenterMain (-)
// LeftMain (flip -> RightMain)

impl Default for ColumnType {
    fn default() -> Self {
        ColumnType::MainAndStack
    }
}
