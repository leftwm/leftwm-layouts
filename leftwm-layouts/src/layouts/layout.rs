use std::cmp;

use serde::{Deserialize, Serialize};

use crate::geometry::{Flip, Reserve, Rotation, Size, Split};

use super::defaults::{
    center_main, center_main_balanced, center_main_fluid, dwindle, even_horizontal, even_vertical,
    fibonacci, grid, main_and_deck, main_and_horizontal_stack, main_and_vert_stack, monocle,
    right_main_and_vert_stack,
};

const DEFAULT_MAIN_SIZE_CHANGE_PIXEL: i32 = 50;
const DEFAULT_MAIN_SIZE_CHANGE_PERCENTAGE: i32 = 5;

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
    #[serde(default)]
    pub flip: Flip,

    /// Rotate the entire result of tiles as a whole, if specified to be anything other than [`Rotation::North`]
    #[serde(default)]
    pub rotate: Rotation,

    /// Defines the layouts behavior if certain "columns" (eg. main, stack, or second-stack) are empty.
    /// See [`Reserve`] for more information.
    #[serde(default)]
    pub reserve: Reserve,

    /// Configuration concerning the [`Main`], [`Stack`], and [`SecondStack`] columns.
    /// See [`Columns`] for more information.
    #[serde(default)]
    pub columns: Columns,
}

impl Layout {
    /// Returns `true` if the layout must be considered a `Monocle` layout.
    ///
    /// The `Monocle` layout is a special layout that always consists
    /// of 0 or 1 windows. If there is a window, it is shown full screen.
    pub fn is_monocle(&self) -> bool {
        self.columns.main.is_none()
            && self.columns.second_stack.is_none()
            && self.columns.stack.split.is_none()
    }

    /// Returns `true` if the layout must be considered a `MainAndDeck` layout.
    ///
    /// The `MainAndDeck` layout is a special layout that always consists
    /// of 0, 1 or 2 windows.
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

    // Get the size of the [`Main`] column,
    // may return [`None`] if there is no [`Main`] column.
    pub fn main_size(&self) -> Option<Size> {
        self.columns.main.as_ref().map(|m| m.size)
    }

    // Get the amount of window spaces of the [`Main`] column,
    // may return [`None`] if there is no [`Main`] column.
    pub fn main_window_count(&self) -> Option<usize> {
        self.columns.main.as_ref().map(|m| m.count)
    }

    /// Set the [`Size`] of the [`Main`] column to a specific value
    pub fn set_main_size(&mut self, size: Size) {
        if let Some(main) = self.columns.main.as_mut() {
            main.size = size;
        }
    }

    /// Increase the [`Size`] of the [`Main`] column, but to no
    /// larger value than what is set in `upper_bound`.
    ///
    /// The column is increased by a default amount,
    /// either [`DEFAULT_MAIN_SIZE_CHANGE_PIXEL`] or
    /// [`DEFAULT_MAIN_SIZE_CHANGE_PERCENTAGE`] depending
    /// on whether the current [`Size`] is a [`Size::Pixel`] or [`Size::Ratio`].
    ///
    /// If the current layout has no [`Main`] column, nothing happens
    pub fn increase_main_size(&mut self, upper_bound: i32) {
        if let Some(main) = self.columns.main.as_mut() {
            match main.size {
                Size::Pixel(_) => {
                    self.change_main_size(DEFAULT_MAIN_SIZE_CHANGE_PIXEL, upper_bound);
                }
                Size::Ratio(_) => {
                    self.change_main_size(DEFAULT_MAIN_SIZE_CHANGE_PERCENTAGE, upper_bound);
                }
            };
        };
    }

    /// Decrease the [`Size`] of the [`Main`] column, but to no
    /// smaller value than zero.
    ///
    /// The column is decreased by a default amount,
    /// either [`DEFAULT_MAIN_SIZE_CHANGE_PIXEL`] or
    /// [`DEFAULT_MAIN_SIZE_CHANGE_PERCENTAGE`] depending
    /// on whether the current [`Size`] is a [`Size::Pixel`] or [`Size::Ratio`].
    ///
    /// If the current layout has no [`Main`] column, nothing happens
    pub fn decrease_main_size(&mut self) {
        if let Some(main) = self.columns.main.as_mut() {
            // note: upper bound doesn't matter when we're decreasing,
            // so just set it to i32::MAX
            match main.size {
                Size::Pixel(_) => self.change_main_size(-DEFAULT_MAIN_SIZE_CHANGE_PIXEL, i32::MAX),
                Size::Ratio(_) => {
                    self.change_main_size(-DEFAULT_MAIN_SIZE_CHANGE_PERCENTAGE, i32::MAX);
                }
            };
        };
    }

    /// Change the [`Size`] of the [`Main`] column by a `delta` value.
    ///
    /// The `delta` value can be positive or negative and is interpreted
    /// as either [`Size::Pixel`] or [`Size::Ratio`] based on the current
    /// [`Size`] of the [`Main`] column.
    ///
    /// When the current [`Size`] is a [`Size::Pixel`], the delta is
    /// interpreted as a pixel value.
    ///
    /// ```
    /// use leftwm_layouts::Layout;
    /// use leftwm_layouts::geometry::Size;
    ///
    /// let mut layout = Layout::default();
    /// layout.set_main_size(Size::Pixel(200));
    /// layout.change_main_size(100, 500);
    /// assert_eq!(Size::Pixel(300), layout.columns.main.unwrap().size);
    /// ```
    ///
    /// When the current [`Size`] is a [`Size::Ratio`], the delta is
    /// interpreted as a percentage value and converted into a ratio
    /// (i.e. `5` (percent) => `Size::Ratio(0.05)`).
    ///
    /// ```
    /// use leftwm_layouts::Layout;
    /// use leftwm_layouts::geometry::Size;
    ///
    /// let mut layout = Layout::default();
    /// layout.set_main_size(Size::Ratio(0.5));
    /// layout.change_main_size(5, 500);
    /// assert_eq!(Size::Ratio(0.55), layout.columns.main.unwrap().size);
    /// ```
    pub fn change_main_size(&mut self, delta: i32, upper_bound: i32) {
        if let Some(main) = self.columns.main.as_mut() {
            main.size = match main.size {
                Size::Pixel(px) => Size::Pixel(cmp::max(0, cmp::min(upper_bound, px + delta))),
                Size::Ratio(ratio) => {
                    Size::Ratio(f32::max(0.0, f32::min(1.0, ratio + (delta as f32 * 0.01))))
                }
            }
        }
    }

    //pub fn change_main_size_enum(&mut self, amount: Size, upper_bound: i32) {
    //    if let Some(main) = self.columns.main.as_mut() {
    //        match (main.size, amount) {
    //            (Size::Pixel(_), Size::Pixel(px)) => self.change_main_size(px, upper_bound),
    //            (Size::Pixel(_), Size::Ratio(_)) => todo!(), // ?
    //            (Size::Ratio(_), Size::Pixel(_)) => todo!(), // ?
    //            (Size::Ratio(_), Size::Ratio(ratio)) => {
    //                self.change_main_size((ratio * 100.0).round() as i32, upper_bound)
    //            }
    //        }
    //    };
    //    amount.into_absolute(upper_bound.unsigned_abs());
    //}

    // Set the amount of main windows to a specific amount
    pub fn set_main_window_count(&mut self, count: usize) {
        if let Some(main) = self.columns.main.as_mut() {
            main.count = cmp::max(0, count);
        }
    }

    // Increase the amount of main windows by 1
    pub fn increase_main_window_count(&mut self) {
        if let Some(main) = self.columns.main.as_mut() {
            main.count = main.count.saturating_add(1);
        }
    }

    // Decrease the amount of main windows by 1
    pub fn decrease_main_window_count(&mut self) {
        if let Some(main) = self.columns.main.as_mut() {
            main.count = main.count.saturating_sub(1);
        }
    }

    // Rotate the layout as a whole.
    // Rotates clockwise if `true` and counter-clockwise if `false`.
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
/// ```txt
/// +------+------+------+
/// |      |      |      |
/// |      |      |      |
/// |      |      |      |
/// +------+------+------+
///  stack   main  second
///                stack
/// ```
///
/// ## Modifiers
/// Modifiers like [`Flip`] and [`Rotation`] are applied only to the columns themselves and not their contents.
///
/// For example, if you wish for the `Stack` to be on the left side instead of the right side
/// in a `MainAndStack` layout configuration, the [`Flip`] property could be set to [`Flip::Vertical`],
/// which results in the columns being flipped, **but not their contents**.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(default)]
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
    #[serde(default = "default_opt_main")]
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

fn default_opt_main() -> Option<Main> {
    Some(Main::default())
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
#[serde(default)]
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

#[cfg(test)]
mod tests {
    use crate::{
        geometry::Size,
        layouts::{
            layout::{DEFAULT_MAIN_SIZE_CHANGE_PERCENTAGE, DEFAULT_MAIN_SIZE_CHANGE_PIXEL},
            Layouts,
        },
        Layout,
    };

    #[test]
    fn monocle_layout_is_monocle() {
        let layouts = Layouts::default();
        let layout = layouts.get("Monocle").unwrap();
        assert!(layout.is_monocle());
    }

    #[test]
    fn main_and_deck_layout_is_main_and_deck() {
        let layouts = Layouts::default();
        let layout = layouts.get("MainAndDeck").unwrap();
        assert!(layout.is_main_and_deck());
    }

    #[test]
    fn set_main_size_works() {
        let mut layout = Layout::default();
        layout.set_main_size(Size::Ratio(0.5));
        assert_eq!(Some(Size::Ratio(0.5)), layout.main_size());
    }

    #[test]
    fn increase_main_size_percentage_works() {
        let mut layout = Layout::default();
        layout.set_main_size(Size::Ratio(0.5));
        layout.increase_main_size(500);
        assert_eq!(
            Some(Size::Ratio(
                0.5 + (DEFAULT_MAIN_SIZE_CHANGE_PERCENTAGE as f32 * 0.01)
            )),
            layout.main_size()
        );
    }

    #[test]
    fn decrease_main_size_percentage_works() {
        let mut layout = Layout::default();
        layout.set_main_size(Size::Ratio(0.5));
        layout.decrease_main_size();
        assert_eq!(
            Some(Size::Ratio(
                0.5 - (DEFAULT_MAIN_SIZE_CHANGE_PERCENTAGE as f32 * 0.01)
            )),
            layout.main_size()
        );
    }

    #[test]
    fn increase_main_size_pixel_works() {
        let mut layout = Layout::default();
        layout.set_main_size(Size::Pixel(200));
        layout.increase_main_size(500);
        assert_eq!(
            Some(Size::Pixel(200 + DEFAULT_MAIN_SIZE_CHANGE_PIXEL)),
            layout.main_size()
        );
    }

    #[test]
    fn decrease_main_size_pixel_works() {
        let mut layout = Layout::default();
        layout.set_main_size(Size::Pixel(200));
        layout.decrease_main_size();
        assert_eq!(
            Some(Size::Pixel(200 - DEFAULT_MAIN_SIZE_CHANGE_PIXEL)),
            layout.main_size()
        );
    }

    #[test]
    fn change_main_size_percentage_negative_works() {
        let mut layout = Layout::default();
        layout.set_main_size(Size::Ratio(0.5));
        layout.change_main_size(-5, 500);
        assert_eq!(Some(Size::Ratio(0.45)), layout.main_size());
    }

    #[test]
    fn change_main_size_percentage_positive_works() {
        let mut layout = Layout::default();
        layout.set_main_size(Size::Ratio(0.5));
        layout.change_main_size(5, 500);
        assert_eq!(Some(Size::Ratio(0.55)), layout.main_size());
    }

    #[test]
    fn change_main_size_pixel_negative_works() {
        let mut layout = Layout::default();
        layout.set_main_size(Size::Pixel(200));
        layout.change_main_size(-5, 500);
        assert_eq!(Some(Size::Pixel(195)), layout.main_size());
    }

    #[test]
    fn change_main_size_pixel_positive_works() {
        let mut layout = Layout::default();
        layout.set_main_size(Size::Pixel(200));
        layout.change_main_size(5, 500);
        assert_eq!(Some(Size::Pixel(205)), layout.main_size());
    }

    #[test]
    fn decrease_main_size_does_not_go_below_zero() {
        let mut layout = Layout::default();
        layout.set_main_size(Size::Pixel(200));
        layout.change_main_size(-200, 500);
        assert_eq!(Some(Size::Pixel(0)), layout.main_size());
        layout.change_main_size(-200, 500);
        assert_eq!(Some(Size::Pixel(0)), layout.main_size());
    }

    #[test]
    fn decrease_main_size_does_not_go_above_upper_bound() {
        let mut layout = Layout::default();
        layout.set_main_size(Size::Pixel(200));
        layout.change_main_size(200, 500);
        assert_eq!(Some(Size::Pixel(400)), layout.main_size());
        layout.change_main_size(200, 500);
        assert_eq!(Some(Size::Pixel(500)), layout.main_size());
    }

    #[test]
    fn set_main_window_count_works() {
        let mut layout = Layout::default();
        layout.set_main_window_count(5);
        assert_eq!(Some(5), layout.main_window_count());
    }

    #[test]
    fn increase_main_window_count_works() {
        let mut layout = Layout::default();
        layout.set_main_window_count(5);
        layout.increase_main_window_count();
        assert_eq!(Some(6), layout.main_window_count());
    }

    #[test]
    fn decrease_main_window_count_works() {
        let mut layout = Layout::default();
        layout.set_main_window_count(5);
        layout.decrease_main_window_count();
        assert_eq!(Some(4), layout.main_window_count());
    }

    #[test]
    fn main_window_count_does_not_go_below_zero() {
        let mut layout = Layout::default();
        layout.set_main_window_count(1);
        layout.decrease_main_window_count();
        assert_eq!(Some(0), layout.main_window_count());
        layout.decrease_main_window_count();
        assert_eq!(Some(0), layout.main_window_count());
    }
}
