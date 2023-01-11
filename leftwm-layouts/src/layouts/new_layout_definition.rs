use std::cmp;

use serde::{Deserialize, Serialize};

use crate::geometry::{Flipped, ReserveColumnSpace, Rotation, Size, SplitAxis};

type LayoutName = String;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LayoutDefinition {
    pub name: LayoutName,

    pub flip: Flipped,
    pub rotate: Rotation,
    pub reserve: ReserveColumnSpace,

    pub root: Root,
    pub main: Option<Main>,
    pub stack: Stack,
    pub alternate_stack: Option<AlternateStack>,
}

impl LayoutDefinition {
    pub fn is_monocle(&self) -> bool {
        self.main.is_none() && self.alternate_stack.is_none() && self.stack.split.is_none()
    }

    pub fn is_main_and_deck(&self) -> bool {
        match &self.main {
            Some(main) => {
                self.alternate_stack.is_none() && main.split.is_none() && self.stack.split.is_none()
            }
            None => false,
        }
    }

    pub fn increase_main_size(&mut self, upper_bound: i32) {
        self.main.as_mut().map(|main| {
            main.size = match main.size {
                Size::Pixel(x) => Size::Pixel(cmp::min(x + 50, upper_bound)),
                Size::Ratio(x) => Size::Ratio(f32::min(1.0, x + 0.05)),
            };
        });
    }

    pub fn decrease_main_size(&mut self) {
        self.main.as_mut().map(|main| {
            main.size = match main.size {
                Size::Pixel(x) => Size::Pixel(cmp::max(0, x - 50)),
                Size::Ratio(x) => Size::Ratio(f32::max(0.0, x - 0.05)),
            };
        });
    }

    pub fn set_main_size(&mut self, px: i32) {
        self.main.as_mut().map(|main| main.size = Size::Pixel(px));
    }

    pub fn increase_main_window_count(&mut self) {
        self.main
            .as_mut()
            .map(|main| main.count = main.count.saturating_add(1));
    }

    pub fn decrease_main_window_count(&mut self) {
        self.main
            .as_mut()
            .map(|main| main.count = main.count.saturating_sub(1));
    }

    pub fn rotate(&mut self, clockwise: bool) {
        self.rotate = if clockwise {
            self.rotate.clockwise()
        } else {
            self.rotate.counter_clockwise()
        }
    }

    /*pub fn column_count(&self) -> usize {
        match (&self.main, &self.alternate_stack) {
            (None, None) => 1,
            (None, Some(_)) => 1,
            (Some(_), None) => 2,
            (Some(_), Some(_)) => 3,
        }
    }*/

    pub fn check(&self) {
        if let Some(_) = &self.alternate_stack {
            if self.main.is_none() {
                // warning -> alternate_stack is ignored -> 1-column
            }
        }
    }
}

impl Default for LayoutDefinition {
    fn default() -> Self {
        Self {
            name: String::from("Default"),
            flip: Flipped::None,
            rotate: Rotation::North,
            reserve: ReserveColumnSpace::None,
            root: Root::default(),
            main: Some(Main::default()),
            stack: Stack::default(),
            alternate_stack: None,
        }
    }
}

/*impl Default for NewLayoutDefinition {
    fn default() -> Self {
        Self {
            name: String::from("Default"),
            column_type: ColumnType::MainAndStack,
            flipped: Flipped::default(),
            rotation: Rotation::default(),
            main_window_count: 1,
            main_size: Size::Ratio(0.5),
            main_split: SplitAxis::Vertical,
            stack_split: SplitAxis::Horizontal,
            reserve_column_space: ReserveColumnSpace::None,
            balance_stacks: true,
        }
    }
}*/

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Root {
    pub flip: Flipped,
    pub rotate: Rotation,
}

impl Default for Root {
    fn default() -> Self {
        Self {
            flip: Default::default(),
            rotate: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Main {
    pub count: usize,
    pub size: Size,
    pub flip: Flipped,
    pub rotate: Rotation,
    pub split: Option<SplitAxis>,
}

impl Default for Main {
    fn default() -> Self {
        Self {
            count: 1,
            size: Size::Ratio(0.5),
            flip: Default::default(),
            rotate: Default::default(),
            split: Some(SplitAxis::Vertical),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Stack {
    pub flip: Flipped,
    pub rotate: Rotation,
    pub split: Option<SplitAxis>,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            flip: Default::default(),
            rotate: Default::default(),
            split: Some(SplitAxis::Horizontal),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AlternateStack {
    pub flip: Flipped,
    pub rotate: Rotation,
    pub split: SplitAxis,
}

impl Default for AlternateStack {
    fn default() -> Self {
        Self {
            flip: Default::default(),
            rotate: Default::default(),
            split: SplitAxis::Horizontal,
        }
    }
}
