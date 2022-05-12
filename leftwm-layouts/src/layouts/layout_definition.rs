use std::{collections::HashMap, cmp};

use serde::{Deserialize, Serialize};

use crate::geometry::{Flipped, ReserveColumnSpace, Rotation, Size, SplitAxis};

use super::columns::ColumnType;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Layouts {
    pub layouts: HashMap<String, LayoutDefinition>,
}

impl Layouts {
    pub fn layout_names(&self) -> Vec<String> {
        self.layouts.keys().map(|x| x.to_owned()).collect()
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
    pub fn increase_main_size(&mut self) {
        self.main_size = match self.main_size {
            Size::Pixel(x) => Size::Pixel(x + 50), // todo: upper limit to container size
            Size::Ratio(x) => Size::Ratio(f32::min(1.0, x + 0.05)),
        };
    }

    pub fn decrease_main_size(&mut self) {
        self.main_size = match self.main_size {
            Size::Pixel(x) => Size::Pixel(cmp::max(0, x - 50)),
            Size::Ratio(x) => Size::Ratio(f32::max(0.0, x - 0.05)),
        };
    }
}

/*impl Default for LayoutDefinition {
    fn default() -> Self {
        Self {
            column_type: ColumnType::MainAndStack,
            flipped: Flipped::None,
            rotation: Rotation::North,
            main_window_count: 1,
            main_size: Size::Ratio(0.5),
            main_split: SplitAxis::Vertical,
            first_stack_split: SplitAxis::Horizontal,
            second_stack_split: SplitAxis::Horizontal,
            reserve_column_space: ReserveColumnSpace::None,
            balance_stacks: true,
        }
    }
}*/

/*fn default_column_type() -> ColumnType {
    ColumnType::MainAndStack
}*/

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

pub fn default_layouts() -> Layouts {
    let default_layouts = include_str!("default.ron");
    let conf: Layouts = ron::from_str(default_layouts).unwrap();
    conf
    /*Layouts {
        layouts: HashMap::from([
            ("Dwindle".to_string(), LayoutDefinition { 
                column_type: ColumnType::MainAndStack, 
                flipped: Flipped::None, 
                rotation: Rotation::North, 
                main_window_count: 1, 
                main_size: Size::Ratio(0.5), 
                main_split: SplitAxis::Vertical, 
                stack_split: SplitAxis::Dwindle, 
                reserve_column_space: ReserveColumnSpace::None, 
                balance_stacks: true 
            }),
        ]),
    }*/
}

#[cfg(test)]
mod tests {
    use crate::layouts::{layout_definition::Layouts, LayoutDefinition};

    use super::default_layouts;

    #[test]
    fn serialization_test() {
        let def: LayoutDefinition = ron::from_str("()").unwrap();
        println!("RON: {}", ron::to_string(&def).unwrap());
    }

    #[test]
    fn multiple_layout_definitions() {
        for l in default_layouts().layouts {
            print!("{}: {:?}", l.0, l.1)
        }
        /*let defs: HashMap<String, LayoutDefinition> = ron::from_str(conf).unwrap();
        println!("RON: {}", ron::to_string(&defs).unwrap())*/
    }
}
