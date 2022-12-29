use std::cmp;

use serde::{Deserialize, Serialize};

use crate::geometry::{ColumnType, Flipped, ReserveColumnSpace, Rotation, Size, SplitAxis};

type LayoutName = String;

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

    #[allow(dead_code)]
    fn append_or_overwrite(&mut self, layout: LayoutDefinition) {
        match self.layouts.iter().position(|x| x.name == layout.name) {
            None => self.layouts.insert(0, layout.clone()),
            Some(i) => {
                self.layouts[i] = layout;
            }
        }
    }

    // pub fn from_config_with_defaults(config: &str) -> Self {
    //     let mut layouts: Layouts = Layouts::default();
    //     let custom_layouts: Vec<LayoutDefinition> = ron::from_str(config).unwrap();
    //     for custom_layout in custom_layouts {
    //         layouts.append_or_overwrite(custom_layout);
    //     }
    //     layouts
    // }

    // pub fn from_config(config: &str) -> Self {
    //     let layouts: Vec<LayoutDefinition> = ron::from_str(config).unwrap();
    //     Self { layouts }
    // }
}

impl Default for Layouts {
    fn default() -> Self {
        let even_horizontal = LayoutDefinition {
            name: String::from("EvenHorizontal"),
            column_type: ColumnType::Stack,
            stack_split: SplitAxis::Vertical,
            ..LayoutDefinition::default()
        };
        let even_vertical = LayoutDefinition {
            name: String::from("EvenVertical"),
            column_type: ColumnType::Stack,
            stack_split: SplitAxis::Horizontal,
            ..LayoutDefinition::default()
        };
        let monocle = LayoutDefinition {
            name: String::from("Monocle"),
            column_type: ColumnType::Stack,
            stack_split: SplitAxis::None,
            main_split: SplitAxis::None,
            ..LayoutDefinition::default()
        };
        let grid = LayoutDefinition {
            name: String::from("Grid"),
            column_type: ColumnType::Stack,
            stack_split: SplitAxis::Grid,
            ..LayoutDefinition::default()
        };
        let main_and_vert_stack = LayoutDefinition {
            name: String::from("MainAndVertStack"),
            column_type: ColumnType::MainAndStack,
            stack_split: SplitAxis::Horizontal,
            ..LayoutDefinition::default()
        };
        let main_and_horizontal_stack = LayoutDefinition {
            name: String::from("MainAndHorizontalStack"),
            column_type: ColumnType::MainAndStack,
            stack_split: SplitAxis::Vertical,
            ..LayoutDefinition::default()
        };
        let right_main_and_vert_stack = LayoutDefinition {
            name: String::from("RightMainAndVertStack"),
            column_type: ColumnType::MainAndStack,
            stack_split: SplitAxis::Horizontal,
            flipped: Flipped::Vertical,
            ..LayoutDefinition::default()
        };
        let fibonacci = LayoutDefinition {
            name: String::from("Fibonacci"),
            column_type: ColumnType::MainAndStack,
            stack_split: SplitAxis::Fibonacci,
            ..LayoutDefinition::default()
        };
        let dwindle = LayoutDefinition {
            name: String::from("Dwindle"),
            column_type: ColumnType::MainAndStack,
            stack_split: SplitAxis::Dwindle,
            ..LayoutDefinition::default()
        };
        let main_and_deck = LayoutDefinition {
            name: String::from("MainAndDeck"),
            column_type: ColumnType::MainAndStack,
            stack_split: SplitAxis::None,
            main_split: SplitAxis::None,
            ..LayoutDefinition::default()
        };
        let center_main = LayoutDefinition {
            name: String::from("CenterMain"),
            column_type: ColumnType::CenterMain,
            stack_split: SplitAxis::Horizontal,
            ..LayoutDefinition::default()
        };
        let center_main_balanced = LayoutDefinition {
            name: String::from("CenterMainBalanced"),
            column_type: ColumnType::CenterMain,
            stack_split: SplitAxis::Dwindle,
            ..LayoutDefinition::default()
        };
        let center_main_fluid = LayoutDefinition {
            name: String::from("CenterMainFluid"),
            column_type: ColumnType::CenterMain,
            stack_split: SplitAxis::Horizontal,
            reserve_column_space: ReserveColumnSpace::Reserve,
            balance_stacks: false,
            ..LayoutDefinition::default()
        };
        let layouts = vec![
            even_horizontal,
            even_vertical,
            monocle,
            grid,
            main_and_vert_stack,
            main_and_horizontal_stack,
            right_main_and_vert_stack,
            fibonacci,
            dwindle,
            main_and_deck,
            center_main,
            center_main_balanced,
            center_main_fluid,
        ];
        Self { layouts }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LayoutDefinition {
    /// The unique identifier for the layout,
    /// there can be only one layout with the same name.
    /// If a layout is defined multiple times, it may be overwritten.
    pub name: LayoutName,

    /// The column type used in this layout.
    /// This usually isn't changed during runtime.
    /// See [`ColumnType`] for details.
    //#[serde(default = "default_column_type")]
    pub column_type: ColumnType,

    /// See [`Flipped`] for details.
    #[serde(default)]
    pub flipped: Flipped,

    /// See [`Rotation`] for details.
    #[serde(default)]
    pub rotation: Rotation,

    /// Determines the amount of windows to show in the
    /// `main` column of the layout. If the layout has no `main`
    /// column, this modifier will be ignored.
    #[serde(default = "default_main_window_count")]
    pub main_window_count: usize,

    /// The [`Size`] of the available space which the
    /// `main` column should occupy. If the layout has no `main` column,
    /// or no window in the `main` column, this modifier will be ignored.
    /// Value can either be absolute Pixels (`400px`) or a Ratio (eg. `0.5`).
    /// Defaults to `0.5`, meaning `50%`.
    #[serde(default = "default_main_size")]
    pub main_size: Size,

    /// The way to split windows in the main_column when there
    /// are more than one window. If the layout has no `main` column,
    /// or no window in the `main` column, this modifier will be ignored.
    /// See [`SplitAxis`] for details. Defaults to [`SplitAxis::Vertical`]
    #[serde(default = "default_main_split")]
    pub main_split: SplitAxis,

    /// The way to split windows in the stack_column(s) when
    /// there are more than one window. See [`SplitAxis`] for details.
    //#[serde(default)]
    pub stack_split: SplitAxis,

    /// The way to handle empty column space where there is no window.
    /// See [`ReserveColumnSpace`] for details.
    #[serde(default)]
    pub reserve_column_space: ReserveColumnSpace,

    /// When set to `true` stack windows are distributed evenly between stacks,
    /// when set to `false` the first stack gets a single window, and
    /// the rest of the windows go to the second stack.
    /// This modifier is ignored in layouts that have just one stack
    ///
    /// ## Demonstration
    /// When set to `true`
    /// ```txt
    /// +-----+-------+-----+
    /// |  2  |       |  4  |
    /// |     |       |     |
    /// |-----|   1   |-----|
    /// |  3  |       |  5  |
    /// |     |       |     |
    /// +-----+-------+-----+
    /// ```
    ///
    /// When set to `false`
    /// ```txt
    /// +-----+-------+-----+
    /// |     |       |  3  |
    /// |     |       |-----|
    /// |  2  |   1   |  4  |
    /// |     |       |-----|
    /// |     |       |  5  |
    /// +-----+-------+-----+
    /// ```
    #[serde(default = "default_balance_stacks")]
    pub balance_stacks: bool,
}

impl Default for LayoutDefinition {
    fn default() -> Self {
        LayoutDefinition {
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
}

impl LayoutDefinition {
    pub fn increase_main_size(&mut self, upper_bound: i32) {
        self.main_size = match self.main_size {
            Size::Pixel(x) => Size::Pixel(cmp::min(x + 50, upper_bound)),
            Size::Ratio(x) => Size::Ratio(f32::min(1.0, x + 0.05)),
        };
    }

    pub fn decrease_main_size(&mut self) {
        self.main_size = match self.main_size {
            Size::Pixel(x) => Size::Pixel(cmp::max(0, x - 50)),
            Size::Ratio(x) => Size::Ratio(f32::max(0.0, x - 0.05)),
        };
    }

    pub fn set_main_size(&mut self, px: i32) {
        self.main_size = Size::Pixel(px);
    }

    pub fn increase_main_window_count(&mut self) {
        self.main_window_count = self.main_window_count.saturating_add(1);
    }

    pub fn decrease_main_window_count(&mut self) {
        self.main_window_count = self.main_window_count.saturating_sub(1);
    }

    pub fn rotate(&mut self, clockwise: bool) {
        self.rotation = if clockwise {
            self.rotation.clockwise()
        } else {
            self.rotation.counter_clockwise()
        }
    }
}

fn default_main_split() -> SplitAxis {
    SplitAxis::Vertical
}

fn default_main_size() -> Size {
    Size::Ratio(0.5)
}

fn default_main_window_count() -> usize {
    1
}

fn default_balance_stacks() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use crate::Layouts;

    /*#[test]
    fn serialization_test() {
        let def: Vec<LayoutDefinition> = ron::from_str("[]").unwrap();
        println!("RON: {}", ron::to_string(&def).unwrap());
    }

    #[test]
    fn load_default_test() {
        let default_layouts = include_str!("layouts.ron");
        let def: Vec<LayoutDefinition> = ron::from_str(default_layouts).unwrap();
        println!("RON: {}", ron::to_string(&def).unwrap());
    }*/

    #[test]
    fn load_default() {
        let reg = Layouts::default();
        let len = reg.layouts.len();
        assert!(len > 0);
        assert!(reg.get("MainAndVertStack").is_some());
    }

    // #[test]
    // fn load_default_with_additional_layouts() {
    //     let default = Layouts::default();
    //     let len = default.layouts.len();

    //     let config: &str = "[(name: \"SomeCustomLayout\", column_type: MainAndStack, stack_split: Horizontal, main_split: Horizontal)]";
    //     let reg = Layouts::from_config_with_defaults(config);

    //     // because of the custom layout, there is one more than in the defaults
    //     assert_eq!(len + 1, reg.layouts.len());

    //     assert!(reg.get("SomeCustomLayout").is_some());
    // }

    // #[test]
    // fn load_default_with_customizing_defaults() {
    //     let default = Layouts::default();
    //     let len = default.layouts.len();

    //     let config: &str = "[(name: \"CenterMain\", column_type: MainAndStack, stack_split: Horizontal, main_split: Horizontal)]";
    //     let reg = Layouts::from_config_with_defaults(config);

    //     // because we are overwriting an existing default layout, the amount of layouts doesn't change
    //     assert_eq!(len, reg.layouts.len());
    // }
}
