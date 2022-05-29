use std::cmp;

use serde::{Deserialize, Serialize};

use crate::geometry::{ColumnType, Flipped, ReserveColumnSpace, Rotation, Size, SplitAxis};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layouts {
    layouts: Vec<LayoutDefinition>,
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
        self.layouts.iter().map(|x| x.name.to_owned()).collect()
    }

    fn append_or_overwrite(&mut self, layout: LayoutDefinition) {
        match self.layouts.iter().position(|x| x.name == layout.name) {
            None => self.layouts.insert(0, layout.to_owned()),
            Some(i) => {
                self.layouts[i] = layout;
            }
        }
    }

    pub fn load_with_defaults(config: &str) -> Self {
        let mut reg: Layouts = Layouts::default();
        let layouts: Vec<LayoutDefinition> = ron::from_str(config).unwrap();
        for layout in layouts {
            reg.append_or_overwrite(layout);
        }
        reg
    }
}

impl Default for Layouts {
    fn default() -> Self {
        let default_layouts = include_str!("layouts.ron");
        let layouts: Vec<LayoutDefinition> = ron::from_str(default_layouts).unwrap();
        Self { layouts }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LayoutDefinition {
    /// The unique identifier for the layout,
    /// there can be only one layout with the same name.
    /// If a layout is defined multiple times, it may be overwritten.
    pub name: String,

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

    pub fn increase_main_window_count(&mut self) {
        self.main_window_count = self.main_window_count.saturating_add(1);
    }

    pub fn decrease_main_window_count(&mut self) {
        self.main_window_count = self.main_window_count.saturating_sub(1);
    }

    pub fn fallback() -> Self {
        Self {
            name: String::from("MainAndStack"),
            column_type: ColumnType::MainAndStack,
            flipped: Flipped::None,
            rotation: Rotation::North,
            main_window_count: 1,
            main_size: Size::Ratio(0.5),
            main_split: SplitAxis::Vertical,
            stack_split: SplitAxis::Horizontal,
            reserve_column_space: ReserveColumnSpace::None,
            balance_stacks: true,
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

    #[test]
    fn load_default_with_additional_layouts() {
        let default = Layouts::default();
        let len = default.layouts.len();

        let config: &str = "[(name: \"SomeCustomLayout\", column_type: MainAndStack, stack_split: Horizontal, main_split: Horizontal)]";
        let reg = Layouts::load_with_defaults(config);

        // because of the custom layout, there is one more than in the defaults
        assert_eq!(len + 1, reg.layouts.len());

        assert!(reg.get("SomeCustomLayout").is_some());
    }

    #[test]
    fn load_default_with_customizing_defaults() {
        let default = Layouts::default();
        let len = default.layouts.len();

        let config: &str = "[(name: \"CenterMain\", column_type: MainAndStack, stack_split: Horizontal, main_split: Horizontal)]";
        let reg = Layouts::load_with_defaults(config);

        // because we are overwriting an existing default layout, the amount of layouts doesn't change
        assert_eq!(len, reg.layouts.len());
    }
}
