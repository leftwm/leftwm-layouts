#[derive(Debug, PartialEq, Eq, Clone)]
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
