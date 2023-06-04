use std::cmp;
use std::vec;

use geometry::Rect;
use geometry::Split;
use layouts::three_column;
use layouts::two_column;
pub use layouts::Layout;
use layouts::Main;
use layouts::SecondStack;

pub mod geometry;
pub mod layouts;

pub fn apply(definition: &Layout, window_count: usize, container: &Rect) -> Vec<Rect> {
    if window_count == 0 {
        return vec![];
    }

    let mut rects = match (&definition.columns.main, &definition.columns.second_stack) {
        (None, _) => stack(container, window_count, definition.columns.stack.split),
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
    definition: &Layout,
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
            stack_tile = Some(v[0]);
        }
        (Some(a), None) => {
            let mut v = vec![a];
            geometry::rotate(&mut v, definition.columns.rotate, container);
            geometry::flip(&mut v, definition.columns.flip, container);
            main_tile = Some(v[0]);
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
        main_tiles.append(&mut geometry::split(&tile, usize::min(main.count, window_count), main.split));
        geometry::rotate(&mut main_tiles, main.rotate, container);
        geometry::flip(&mut main_tiles, main.flip, container);
    }

    let mut stack_tiles = vec![];
    if let Some(tile) = stack_tile {
        stack_tiles.append(&mut geometry::split(
            &tile,
            window_count.saturating_sub(main.count),
            definition.columns.stack.split,
        ));
        geometry::rotate(&mut stack_tiles, definition.columns.stack.rotate, container);
        geometry::flip(&mut stack_tiles, definition.columns.stack.flip, container);
    }

    let mut all = vec![];
    all.append(&mut main_tiles);
    all.append(&mut stack_tiles);
    all
}

fn stack_main_stack(
    container: &Rect,
    window_count: usize,
    definition: &Layout,
    main: &Main,
    alternate_stack: &SecondStack,
) -> Vec<Rect> {
    let main_window_count = cmp::min(main.count, window_count);
    let stack_window_count = window_count.saturating_sub(main_window_count);
    let balance_stacks = definition.columns.stack.split.is_some();
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
            definition.columns.stack.split,
        ));
        geometry::rotate(&mut left_tiles, definition.columns.stack.rotate, container);
        geometry::flip(&mut left_tiles, definition.columns.stack.flip, container);
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
    use crate::{
        apply,
        geometry::{Rect, Split},
        layouts::{Columns, SecondStack, Stack, Layouts},
        Layout,
    };

    #[test]
    fn single_column_works_with_offset() {
        let layout = Layout {
            columns: Columns {
                main: None,
                stack: Stack {
                    split: Some(Split::Horizontal),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let rect = Rect::new(2560, 1440, 2560, 1440);
        let rects = apply(&layout, 3, &rect);

        assert_eq!(Rect::new(2560, 1440, 2560, 480), rects[0]);
        assert_eq!(Rect::new(2560, 1920, 2560, 480), rects[1]);
        assert_eq!(Rect::new(2560, 2400, 2560, 480), rects[2]);
    }

    #[test]
    fn main_stack_works_with_offset() {
        let layout = Layout::default();
        let rect = Rect::new(2560, 1440, 2560, 1440);
        let rects = apply(&layout, 3, &rect);

        assert_eq!(Rect::new(2560, 1440, 1280, 1440), rects[0]);
        assert_eq!(Rect::new(3840, 1440, 1280, 720), rects[1]);
        assert_eq!(Rect::new(3840, 2160, 1280, 720), rects[2]);
    }

    #[test]
    fn stack_main_stack_works_with_offset() {
        let layout = Layout {
            columns: Columns {
                second_stack: Some(SecondStack::default()),
                ..Default::default()
            },
            ..Default::default()
        };
        let rect = Rect::new(2560, 1440, 2560, 1440);
        let rects = apply(&layout, 3, &rect);
        assert_eq!(Rect::new(3200, 1440, 1280, 1440), rects[0]);
        assert_eq!(Rect::new(2560, 1440, 640, 1440), rects[1]);
        assert_eq!(Rect::new(4480, 1440, 640, 1440), rects[2]);
    }

    #[test]
    fn should_never_return_more_rects_than_windows_for_any_layout() {
        let container = Rect::new(0,0,40,20);
        let mut layouts = Layouts::default().layouts;

        // this specific layout does not exists in the defaults, 
        // but has lead to the issue tested here in the past when user-defined
        layouts.push(Layout {
            name: "MultiMain".to_string(),
            columns: Columns {
                main: Some(crate::layouts::Main {
                    count: 2,
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        });

        for layout in layouts {
            for i in 0usize..6 {
                let rects = apply(&layout, i, &container);
                assert!(rects.len() <= i, "got {}, expected <= {}, layout {}", rects.len(), i, &layout.name);
            }
        }
    }
}
