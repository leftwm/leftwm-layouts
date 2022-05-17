use std::{cmp, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::geometry::{ColumnType, Flipped, ReserveColumnSpace, Rotation, Size, SplitAxis};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Layouts {
    #[serde(default = "default_layout_map")]
    pub layouts: HashMap<String, LayoutDefinition>,
}

impl Layouts {
    pub fn layout_names(&self) -> Vec<String> {
        self.layouts.keys().map(|x| x.to_owned()).collect()
    }
}

impl Default for Layouts {
    fn default() -> Self {
        let default_layouts = include_str!("default.ron");
        ron::from_str(default_layouts).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LayoutDefinition {
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
}

fn default_layout_map() -> HashMap<String, LayoutDefinition> {
    HashMap::from([])
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
    use crate::layouts::Layouts;

    #[test]
    fn serialization_test() {
        let def: Layouts = ron::from_str("()").unwrap();
        println!("RON: {}", ron::to_string(&def).unwrap());
    }

    // todo
    #[test]
    fn multiple_layout_definitions() {
        for l in Layouts::default().layouts {
            print!("{}: {:?}", l.0, l.1)
        }
    }
}
