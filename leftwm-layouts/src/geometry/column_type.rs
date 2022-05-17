use serde::{Deserialize, Serialize};

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
