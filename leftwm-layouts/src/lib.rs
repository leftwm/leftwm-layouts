use geometry::ColumnType;
use geometry::Rect;
use layouts::columns::{main_stack, stack, stack_main_stack};
pub use layouts::LayoutDefinition;
pub use layouts::Layouts;

pub mod geometry;
pub mod layouts;

pub fn apply(definition: &LayoutDefinition, window_count: usize, container: &Rect) -> Vec<Rect> {
    // calculate the layout
    let mut rects = match definition.column_type {
        ColumnType::Stack => stack(window_count, &container, definition),
        ColumnType::MainAndStack => main_stack(window_count, &container, definition),
        ColumnType::CenterMain => stack_main_stack(window_count, &container, definition),
    };

    // flip the layout (if necessary)
    geometry::flip(*container, &mut rects, &definition.flipped);

    // rotate the layout (if necessary)
    geometry::rotate(&mut rects, &definition.rotation);

    rects
}

#[cfg(test)]
mod tests {
    /*use crate::{
        apply, layouts::columns::ColumnType, LayoutDefinition, LayoutEnum, LayoutModifiers,
        LayoutOptions,
    };

    const MAIN_STACK: LayoutDefinition = LayoutDefinition {
        ..Default::default()
    };

    const CENTER_MAIN: LayoutDefinition = LayoutDefinition {
        column_type: ColumnType::CenterMain,

        ..Default::default()
    };

    const ALL_LAYOUTS: &[LayoutEnum] = &[
        LayoutEnum::Monocle,
        LayoutEnum::MainAndVertStack,
        LayoutEnum::CenterMain,
        LayoutEnum::Fibonacci,
    ];*/

    /*#[test]
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
    }*/

    // todo
    //fn no_overlap_of_rects() {
    //    todo!()
    //}

    /*#[test]
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
    }*/

    /*#[test]
    fn test_monocle_layout() {
        let modifiers: LayoutModifiers = LayoutModifiers::default();
        let options: LayoutOptions = LayoutOptions::default();
        let monocle = LayoutEnum::Monocle.get();
        let monocle_positions = monocle.apply(1, options.container_size, &modifiers);
        assert_eq!(monocle_positions.len(), 1);
    }*/
}
