use std::cmp;

use serde::{Deserialize, Serialize};

use crate::geometry::{Flip, Reserve, Rotation, Size, Split};

use super::defaults::{
    center_main, center_main_balanced, center_main_fluid, dwindle, even_horizontal, even_vertical,
    fibonacci, grid, main_and_deck, main_and_horizontal_stack, main_and_vert_stack, monocle,
    right_main_and_vert_stack,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layouts {
    pub layouts: Vec<LayoutDefinition>,
}

impl Eq for Layouts {}

impl Layouts {
    pub fn get(&self, name: &str) -> Option<&LayoutDefinition> {
        self.layouts.iter().find(|&l| l.name.as_str() == name)
    }

    pub fn get_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut LayoutDefinition> {
        self.layouts.iter_mut().find(|l| l.name.as_str() == name)
    }

    pub fn names(&self) -> Vec<String> {
        self.layouts.iter().map(|x| x.name.clone()).collect()
    }

    pub fn len(&self) -> usize {
        self.layouts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.layouts.is_empty()
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.layouts.iter().position(|l| l.name.as_str() == name)
    }
}

impl Default for Layouts {
    fn default() -> Self {
        Self {
            layouts: vec![
                even_horizontal(),
                even_vertical(),
                monocle(),
                grid(),
                main_and_vert_stack(),
                main_and_horizontal_stack(),
                right_main_and_vert_stack(),
                fibonacci(),
                dwindle(),
                main_and_deck(),
                center_main(),
                center_main_balanced(),
                center_main_fluid(),
            ],
        }
    }
}

type LayoutName = String;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LayoutDefinition {
    pub name: LayoutName,

    pub flip: Flip,
    pub rotate: Rotation,
    pub reserve: Reserve,

    pub root: Columns,
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
        if let Some(main) = self.main.as_mut() {
            main.size = match main.size {
                Size::Pixel(x) => Size::Pixel(cmp::min(x + 50, upper_bound)),
                Size::Ratio(x) => Size::Ratio(f32::min(1.0, x + 0.05)),
            };
        };
    }

    pub fn decrease_main_size(&mut self) {
        if let Some(main) = self.main.as_mut() {
            main.size = match main.size {
                Size::Pixel(x) => Size::Pixel(cmp::max(0, x - 50)),
                Size::Ratio(x) => Size::Ratio(f32::max(0.0, x - 0.05)),
            };
        };
    }

    pub fn set_main_size(&mut self, px: i32) {
        if let Some(main) = self.main.as_mut() {
            main.size = Size::Pixel(px);
        };
    }

    pub fn increase_main_window_count(&mut self) {
        if let Some(main) = self.main.as_mut() {
            main.count = main.count.saturating_add(1);
        }
    }

    pub fn decrease_main_window_count(&mut self) {
        if let Some(main) = self.main.as_mut() {
            main.count = main.count.saturating_sub(1);
        }
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
        if self.alternate_stack.is_some() && self.main.is_none() {
            // warning -> alternate_stack is ignored -> 1-column
        }
    }

    pub fn update_defaults(custom: &Vec<LayoutDefinition>) -> Vec<LayoutDefinition> {
        let mut layouts = Layouts::default().layouts;
        for custom_layout in custom {
            layouts.push(custom_layout.clone());
        }
        layouts
    }
}

impl Default for LayoutDefinition {
    fn default() -> Self {
        Self {
            name: String::from("Default"),
            flip: Flip::None,
            rotate: Rotation::North,
            reserve: Reserve::None,
            root: Columns::default(),
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Columns {
    pub flip: Flip,
    pub rotate: Rotation,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Main {
    pub count: usize,
    pub size: Size,
    pub flip: Flip,
    pub rotate: Rotation,
    pub split: Option<Split>,
}

impl Default for Main {
    fn default() -> Self {
        Self {
            count: 1,
            size: Size::Ratio(0.5),
            flip: Flip::default(),
            rotate: Rotation::default(),
            split: Some(Split::Vertical),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Stack {
    pub flip: Flip,
    pub rotate: Rotation,
    pub split: Option<Split>,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            flip: Flip::default(),
            rotate: Rotation::default(),
            split: Some(Split::Horizontal),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AlternateStack {
    pub flip: Flip,
    pub rotate: Rotation,
    pub split: Split,
}

impl Default for AlternateStack {
    fn default() -> Self {
        Self {
            flip: Flip::default(),
            rotate: Rotation::default(),
            split: Split::Horizontal,
        }
    }
}
