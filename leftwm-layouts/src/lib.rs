use std::str::FromStr;

use geometry::{Flipped, Rect};
use layouts::center_main::CenterMain;
use layouts::fibonacci::Fibonacci;
use layouts::main_and_vert_stack::MainAndVertStack;

use crate::layouts::monocle::Monocle;

pub mod geometry;
pub mod layouts;
mod util;

pub use util::Util;

#[derive(PartialEq)]
pub enum LayoutEnum {
    Monocle,
    MainAndVertStack,
    CenterMain,
    Fibonacci,
}

pub struct LayoutParseError;
impl FromStr for LayoutEnum {
    type Err = LayoutParseError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "Monocle" => Ok(LayoutEnum::Monocle),
            "MainAndVertStack" => Ok(LayoutEnum::MainAndVertStack),
            "CenterMain" => Ok(LayoutEnum::CenterMain),
            "Fibonacci" => Ok(LayoutEnum::Fibonacci),
            _ => Err(LayoutParseError),
        }
    }
}

// todo: might be better to use generics?

pub trait Layout {
    /// Get a list of calculated tiles where the windows must be placed.
    /// The list may be shorter than the provided `window_count` bit it will not be longer.
    /// A shorter list indicates that the provided amount of windows (`window_count`) exceeds
    /// the amount of windows that can possibly be displayed for the layout (eg. Monocle, MainAndDeck).
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Rect>;

    // QUESTION: might be helpful if the layout_manager can find out if the layout even supports
    // multiple_master_windows, some might not (monocle?, main_and_deck?)
    //fn supports_multiple_master_windows() -> bool;

    // helper method
    fn main_window_count(&self, window_count: usize, modifiers: &LayoutModifiers) -> usize {
        if window_count < modifiers.master_window_count {
            window_count
        } else {
            modifiers.master_window_count as usize
        }
    }

    // helper method
    fn stack_window_count(&self, window_count: usize, modifiers: &LayoutModifiers) -> usize {
        window_count.saturating_sub(self.main_window_count(window_count, modifiers))
    }
}

pub struct LayoutModifiers {
    pub container_size: Rect,
    pub master_width_percentage: f32,
    pub master_window_count: usize,
    pub max_column_width: Option<u32>,
    pub flipped: Flipped,
}

impl Default for LayoutModifiers {
    fn default() -> Self {
        Self {
            container_size: Rect::default(),
            master_width_percentage: 60.0,
            master_window_count: 1,
            max_column_width: None,
            flipped: Flipped::default(),
        }
    }
}

#[derive(Debug)]
pub struct LayoutNotFoundError;
impl LayoutEnum {
    pub fn get(&self) -> Box<dyn Layout> {
        match self {
            LayoutEnum::Monocle => Box::new(Monocle),
            LayoutEnum::MainAndVertStack => Box::new(MainAndVertStack),
            LayoutEnum::CenterMain => Box::new(CenterMain),
            LayoutEnum::Fibonacci => Box::new(Fibonacci),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LayoutEnum, LayoutModifiers};

    const ALL_LAYOUTS: &[LayoutEnum] = &[
        LayoutEnum::Monocle,
        LayoutEnum::MainAndVertStack,
        LayoutEnum::CenterMain,
        LayoutEnum::Fibonacci,
    ];

    #[test]
    fn returned_tiles_must_never_exceed_window_count() {
        let modifiers: LayoutModifiers = LayoutModifiers::default();
        for window_count in 0..25 {
            for layout in ALL_LAYOUTS {
                let layout = layout.get();
                let len = layout.apply(window_count, &modifiers).len();
                assert!(len <= window_count);
            }
        }
    }

    // todo
    //fn no_overlap_of_rects() {
    //    todo!()
    //}

    // QUESTION: is that a fair assumption?
    // -> follow-up: only works if remaining space is accounted for instead
    //               of rounding off
    //               eg. 3-column layout on 100px width results in 3x 33px leaving a 1px remainder
    //              this remainder should be attributed to one of the columns to fill up the entire width
    #[test]
    fn container_must_always_be_filled() {
        let modifiers: LayoutModifiers = LayoutModifiers::default();
        let container_area = modifiers.container_size.surface_area();
        for window_count in 1..10 {
            for layout in ALL_LAYOUTS {
                let layout = layout.get();
                let filled_area = layout
                    .apply(window_count, &modifiers)
                    .into_iter()
                    .fold(0u32, |a, b| a + b.surface_area());
                assert_eq!(container_area, filled_area);
            }
        }
    }

    #[test]
    fn test_monocle_layout() {
        let modifiers: LayoutModifiers = LayoutModifiers::default();
        let monocle = LayoutEnum::Monocle.get();
        let monocle_positions = monocle.apply(1, &modifiers);
        assert_eq!(monocle_positions.len(), 1);
    }
}
