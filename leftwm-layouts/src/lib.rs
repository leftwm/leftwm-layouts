use std::cmp;
use std::vec;

use geometry::Rect;
use geometry::Split;
use layouts::three_column;
use layouts::two_column;
pub use layouts::LayoutDefinition;
use layouts::Main;
use layouts::SecondStack;

pub mod geometry;
pub mod layouts;

pub fn apply(definition: &LayoutDefinition, window_count: usize, container: &Rect) -> Vec<Rect> {
    if window_count == 0 {
        return vec![];
    }

    let mut rects = match (&definition.main, &definition.second_stack) {
        (None, _) => stack(container, window_count, definition.stack.split),
        (Some(main), None) => main_stack(container, window_count, definition, main),
        (Some(main), Some(alternate_stack)) => {
            stack_main_stack(container, window_count, definition, main, alternate_stack)
        }
    };

    // flip the whole layout
    geometry::flip(&mut rects, definition.flip, container);

    // rotate the whole layout
    geometry::rotate(&mut rects, definition.rotate, container);

    rects
}

fn stack(container: &Rect, window_count: usize, split: Option<Split>) -> Vec<Rect> {
    geometry::split(container, window_count, split)
}

fn main_stack(
    container: &Rect,
    window_count: usize,
    definition: &LayoutDefinition,
    main: &Main,
) -> Vec<Rect> {
    let (mut main_tile, mut stack_tile) = two_column(
        window_count,
        container,
        main.count,
        main.size,
        definition.reserve,
    );

    // root rotation
    match (main_tile, stack_tile) {
        (None, None) => {}
        (None, Some(b)) => {
            let mut v = vec![b];
            geometry::rotate(&mut v, definition.columns.rotate, container);
            geometry::flip(&mut v, definition.columns.flip, container);
            main_tile = Some(v[0]);
        }
        (Some(a), None) => {
            let mut v = vec![a];
            geometry::rotate(&mut v, definition.columns.rotate, container);
            geometry::flip(&mut v, definition.columns.flip, container);
            stack_tile = Some(v[0]);
        }
        (Some(a), Some(b)) => {
            let mut v = vec![a, b];
            geometry::rotate(&mut v, definition.columns.rotate, container);
            geometry::flip(&mut v, definition.columns.flip, container);
            main_tile = Some(v[0]);
            stack_tile = Some(v[1]);
        }
    }

    //geometry::flip(container, &mut rects, definition.flip);

    let mut main_tiles = vec![];
    if let Some(tile) = main_tile {
        main_tiles.append(&mut geometry::split(&tile, main.count, main.split));
        geometry::rotate(&mut main_tiles, main.rotate, container);
        geometry::flip(&mut main_tiles, main.flip, container);
    }

    let mut stack_tiles = vec![];
    if let Some(tile) = stack_tile {
        stack_tiles.append(&mut geometry::split(
            &tile,
            window_count.saturating_sub(main.count),
            definition.stack.split,
        ));
        geometry::rotate(&mut stack_tiles, definition.stack.rotate, container);
        geometry::flip(&mut stack_tiles, definition.stack.flip, container);
    }

    let mut all = vec![];
    all.append(&mut main_tiles);
    all.append(&mut stack_tiles);
    all
}

fn stack_main_stack(
    container: &Rect,
    window_count: usize,
    definition: &LayoutDefinition,
    main: &Main,
    alternate_stack: &SecondStack,
) -> Vec<Rect> {
    let main_window_count = cmp::min(main.count, window_count);
    let stack_window_count = window_count.saturating_sub(main_window_count);
    let balance_stacks = definition.stack.split.is_some();
    let (left_window_count, right_window_count) = if balance_stacks {
        let counts = geometry::remainderless_division(stack_window_count, 2);
        (counts[0], counts[1])
    } else {
        (1, cmp::max(0, stack_window_count.saturating_sub(1)))
    };

    let (left_column, main_column, right_column) = three_column(
        window_count,
        container,
        main_window_count,
        main.size,
        definition.reserve,
        balance_stacks,
    );

    let mut columns = vec![];
    if let Some(col) = left_column {
        columns.push(col);
    }
    if let Some(col) = main_column {
        columns.push(col);
    }
    if let Some(col) = right_column {
        columns.push(col);
    }

    // root rotation
    geometry::rotate(&mut columns, definition.columns.rotate, container);
    geometry::flip(&mut columns, definition.columns.flip, container);

    let mut main_tiles = vec![];
    if let Some(tile) = main_column {
        main_tiles.append(&mut geometry::split(&tile, main_window_count, main.split));
        geometry::rotate(&mut main_tiles, main.rotate, container);
        geometry::flip(&mut main_tiles, main.flip, container);
    }

    let mut left_tiles = vec![];
    if let Some(tile) = left_column {
        left_tiles.append(&mut geometry::split(
            &tile,
            left_window_count,
            definition.stack.split,
        ));
        geometry::rotate(&mut left_tiles, definition.stack.rotate, container);
        geometry::flip(&mut left_tiles, definition.stack.flip, container);
    }

    let mut right_tiles = vec![];
    if let Some(tile) = right_column {
        right_tiles.append(&mut geometry::split(
            &tile,
            right_window_count,
            Some(alternate_stack.split),
        ));
        geometry::rotate(&mut right_tiles, alternate_stack.rotate, container);
        geometry::flip(&mut right_tiles, alternate_stack.flip, container);
    }

    let mut tiles = vec![];
    tiles.append(&mut main_tiles);
    tiles.append(&mut left_tiles);
    tiles.append(&mut right_tiles);
    tiles
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
