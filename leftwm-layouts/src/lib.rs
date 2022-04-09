use std::str::FromStr;

use geometry::Rotation;
use geometry::{Flipped, Rect};
use layouts::CenterMain;
use layouts::Fibonacci;
use layouts::MainAndVertStack;
use layouts::Monocle;

pub mod geometry;
mod layouts;
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
    fn apply(&self, window_count: usize, container: Rect, modifiers: &LayoutModifiers)
        -> Vec<Rect>;

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

pub fn apply(
    layout: &LayoutEnum,
    window_count: usize,
    options: &LayoutOptions,
    modifiers: &LayoutModifiers,
) -> Vec<Rect> {
    let aspect_ratio_changes = options
        .rotation
        .aspect_ratio_changes(&options.container_size);

    // if the aspect-ratio changes with the provided rotation,
    // create a new rect with a swapped aspect-ratio.
    // This makes it easier to rotate the layout later.
    let container = if aspect_ratio_changes {
        Rect {
            h: options.container_size.w,
            w: options.container_size.h,
            ..options.container_size
        }
    } else {
        options.container_size
    };

    // calculate the layout
    let mut rects = layout.get().apply(window_count, container, modifiers);

    // rotate the layout (if necessary)
    rects
        .iter_mut()
        .for_each(|rect| Util::translate_rotation(container, rect, &options.rotation));

    // flip the layout (if necessary)
    Util::flip(options.container_size, &mut rects, &options.flipped);

    rects
}

/// LayoutOptions influence the final result of the layout.
/// They are not passed down to the Layout calculations, but
/// rather applied after the calculations have been done.
pub struct LayoutOptions {
    pub container_size: Rect,
    pub flipped: Flipped,
    pub rotation: Rotation,
}

/// LayoutModifiers are passed down to the layouts.
/// They SHOULD influence the calculations of the various layouts,
/// although not all modifiers might make sense on all layouts.
pub struct LayoutModifiers {
    pub master_width_percentage: f32,
    pub master_window_count: usize,
    pub max_column_width: Option<u32>,
    pub reserve_space: bool,
}

impl Default for LayoutModifiers {
    fn default() -> Self {
        Self {
            master_width_percentage: 60.0,
            master_window_count: 1,
            max_column_width: None,
            reserve_space: false,
        }
    }
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            container_size: Rect::default(),
            flipped: Flipped::None,
            rotation: Rotation::North,
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
    use crate::{apply, LayoutEnum, LayoutModifiers, LayoutOptions};

    const ALL_LAYOUTS: &[LayoutEnum] = &[
        LayoutEnum::Monocle,
        LayoutEnum::MainAndVertStack,
        LayoutEnum::CenterMain,
        LayoutEnum::Fibonacci,
    ];

    #[test]
    fn returned_tiles_must_never_exceed_window_count() {
        let modifiers: LayoutModifiers = LayoutModifiers::default();
        let options: LayoutOptions = LayoutOptions::default();
        for window_count in 0..25 {
            for layout in ALL_LAYOUTS {
                let layout = layout.get();
                let len = layout
                    .apply(window_count, options.container_size, &modifiers)
                    .len();
                assert!(len <= window_count);
            }
        }
    }

    // todo
    //fn no_overlap_of_rects() {
    //    todo!()
    //}

    #[test]
    fn container_must_always_be_filled() {
        let modifiers: LayoutModifiers = LayoutModifiers::default();
        let options: LayoutOptions = LayoutOptions::default();
        let container_area = options.container_size.surface_area();
        for window_count in 1..10 {
            for layout in ALL_LAYOUTS {
                let filled_area = apply(layout, window_count, &options, &modifiers)
                    .into_iter()
                    .fold(0u32, |a, b| a + b.surface_area());
                assert_eq!(container_area, filled_area);
            }
        }
    }

    #[test]
    fn test_monocle_layout() {
        let modifiers: LayoutModifiers = LayoutModifiers::default();
        let options: LayoutOptions = LayoutOptions::default();
        let monocle = LayoutEnum::Monocle.get();
        let monocle_positions = monocle.apply(1, options.container_size, &modifiers);
        assert_eq!(monocle_positions.len(), 1);
    }
}
