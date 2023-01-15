use std::cmp;

use serde::{Deserialize, Serialize};

use crate::geometry::{Flip, Reserve, Rotation, Size, Split};

use super::defaults::{
    center_main, center_main_balanced, center_main_fluid, dwindle, even_horizontal, even_vertical,
    fibonacci, grid, main_and_deck, main_and_horizontal_stack, main_and_vert_stack, monocle,
    right_main_and_vert_stack,
};

/// A helper struct that represents a set of layouts and provides
/// convenience methods
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layouts {
    pub layouts: Vec<Layout>,
}

impl Eq for Layouts {}

impl Layouts {
    pub fn get(&self, name: &str) -> Option<&Layout> {
        self.layouts.iter().find(|&l| l.name.as_str() == name)
    }

    pub fn get_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut Layout> {
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

/// Describes a layout or pattern in which tiles (windows) will be arranged.
/// The [`Layout`] allows to describe various types of "fixed" layouts used by a dynamic tiling manager.
/// Those include layouts like `MainAndStack`, `Fibonacci`, `Dwindle`, `CenterMain`, etc.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Layout {
    /// Name and identifier of the layout.
    /// This is user chosen and no two layouts can have the same name.
    pub name: LayoutName,

    /// Flips the entire result of tiles as a whole if specified to be anything other than [`Flip::None`]
    pub flip: Flip,

    /// Rotate the entire result of tiles as a whole, if specified to be anything other than [`Rotation::North`]
    pub rotate: Rotation,

    /// Defines the layouts behavior if certain "columns" (eg. main, stack, or second-stack) are empty.
    /// See [`Reserve`] for more information.
    pub reserve: Reserve,

    /// Configuration concerning the [`Main`], [`Stack`], and [`SecondStack`] columns.
    /// See [`Columns`] for more information.
    pub columns: Columns,
}

impl Layout {
    pub fn is_monocle(&self) -> bool {
        self.columns.main.is_none()
            && self.columns.second_stack.is_none()
            && self.columns.stack.split.is_none()
    }

    pub fn is_main_and_deck(&self) -> bool {
        match &self.columns.main {
            Some(main) => {
                self.columns.second_stack.is_none()
                    && main.split.is_none()
                    && self.columns.stack.split.is_none()
            }
            None => false,
        }
    }

    pub fn increase_main_size(&mut self, upper_bound: i32) {
        if let Some(main) = self.columns.main.as_mut() {
            main.size = match main.size {
                Size::Pixel(x) => Size::Pixel(cmp::min(x + 50, upper_bound)),
                Size::Ratio(x) => Size::Ratio(f32::min(1.0, x + 0.05)),
            };
        };
    }

    pub fn decrease_main_size(&mut self) {
        if let Some(main) = self.columns.main.as_mut() {
            main.size = match main.size {
                Size::Pixel(x) => Size::Pixel(cmp::max(0, x - 50)),
                Size::Ratio(x) => Size::Ratio(f32::max(0.0, x - 0.05)),
            };
        };
    }

    pub fn set_main_size(&mut self, px: i32) {
        if let Some(main) = self.columns.main.as_mut() {
            main.size = Size::Pixel(px);
        };
    }

    pub fn increase_main_window_count(&mut self) {
        if let Some(main) = self.columns.main.as_mut() {
            main.count = main.count.saturating_add(1);
        }
    }

    pub fn decrease_main_window_count(&mut self) {
        if let Some(main) = self.columns.main.as_mut() {
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

    pub fn check(&self) {
        if self.columns.second_stack.is_some() && self.columns.main.is_none() {
            // warning -> alternate_stack is ignored -> 1-column
        }
    }

    pub fn update_defaults(custom: &Vec<Layout>) -> Vec<Layout> {
        let mut layouts = Layouts::default().layouts;
        for custom_layout in custom {
            layouts.push(custom_layout.clone());
        }
        layouts
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            name: String::from("Default"),
            flip: Flip::None,
            rotate: Rotation::North,
            reserve: Reserve::None,
            columns: Columns::default(),
        }
    }
}

/// Describes the columns of a layout. There are only 3 columns which are a fixed part of
/// `leftwm_layouts`, those are `main`, `stack`, and `second_stack`.
///
/// ## Modifiers
/// Modifiers like [`Flip`] and [`Rotation`] are applied only to the columns themselves and not their contents.
///
/// For example, if you wish for the `Stack` to be on the left side instead of the right side
/// in a `MainAndStack` layout configuration, the [`Flip`] property could be set to [`Flip::Vertical`],
/// which results in the columns being flipped, **but not their contents**.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Columns {
    /// How the columns should be flipped, does not apply to their contents
    pub flip: Flip,

    /// How the columns should be rotated, does not apply to their contents
    pub rotate: Rotation,

    /// Configurations concerning the `main` column.
    /// This can be set to [`None`], in which case the layout
    /// will not have a main column. For example, in single-column
    /// layouts like `EvenVertical`, `Monocle`, etc.
    /// See [`Main`] for more information.
    pub main: Option<Main>,

    /// Configurations concerning the `stack` column.
    /// Other than `main` and `second_stack`, this column is always present.
    /// See [`Stack`] for more information.
    pub stack: Stack,

    /// Configurations concerning the `second_stack` column.
    /// This can be set to [`None`], in which case the layout
    /// is going to be a two-column layout like `MainAndStack`, `Fibonacci`, etc.
    ///
    /// *Note: If this is present but `main` is absent, it is condiered an invalid
    /// layout configuration. The `second_stack` configuration may be ignored if
    /// `main` is [`None`]*
    /// See [`SecondStack`] for more information.
    pub second_stack: Option<SecondStack>,
}

impl Default for Columns {
    fn default() -> Self {
        Self {
            flip: Flip::default(),
            rotate: Rotation::default(),
            main: Some(Main::default()),
            stack: Stack::default(),
            second_stack: None,
        }
    }
}

/// Configurations concerning the `main` column
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Main {
    /// The default amount of windows to occupy the `main` column (default: `1`)
    pub count: usize,

    /// The default size of the `main` column (default: `50%`)
    pub size: Size,

    /// Flip modifier to apply only to the `main` columns' contents
    pub flip: Flip,

    /// Rotation modifier to apply only to the `main` columns' contents
    pub rotate: Rotation,

    /// How tiles (windows) inside the `main` column should be split up,
    /// when there is more than one.
    ///
    /// *Note: This can be set to [`None`], in which case the `main` column can't
    /// contain more than one window (eg. `MainAndDeck`)*
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

/// Configurations concerning the `stack` column
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Stack {
    /// Flip modifier to apply only to the `stack` columns' contents
    pub flip: Flip,

    /// Rotation modifier to apply only to the `stack` columns' contents
    pub rotate: Rotation,

    /// How tiles (windows) inside the `stack` column should be split up,
    /// when there is more than one.
    ///
    /// *Note: This can be set to [`None`], in which case the `stack` column can't
    /// contain more than one window (eg. `Monocle`, `MainAndDeck`)*
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

/// Configurations concerning the `second_stack` column
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SecondStack {
    /// Flip modifier to apply only to the `second_stack` columns' contents
    pub flip: Flip,

    /// Rotation modifier to apply only to the `second_stack` columns' contents
    pub rotate: Rotation,

    /// How tiles (windows) inside the `second_stack` column should be split up,
    /// when there is more than one.
    pub split: Split,
}

impl Default for SecondStack {
    fn default() -> Self {
        Self {
            flip: Flip::default(),
            rotate: Rotation::default(),
            split: Split::Horizontal,
        }
    }
}
