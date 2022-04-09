use std::cmp;

use crate::{
    geometry::{Rect, SplitAxis},
    Util,
};

pub fn main_stack(
    window_count: usize,
    container: Rect,
    main_count: usize,
    main_percentage: f32,
    main_split: SplitAxis,
    stack_split: SplitAxis,
    reserve_space: bool,
) -> Vec<Rect> {
    let tiles: &mut Vec<Rect> = &mut Vec::new();
    if window_count == 0 {
        return tiles.to_vec();
    }

    let main_window_count = cmp::min(main_count, window_count);;
    let stack_window_count = window_count.saturating_sub(main_window_count);
    let main_width = (container.w as f32 / 100.0 * main_percentage) as u32;

    let main_tile = if main_window_count > 0 {
        match stack_window_count {
            0 if !reserve_space => Some(container),
            _ => Some(Rect {
                w: main_width,
                ..container
            }),
        }
    } else {
        None
    };

    if let Some(tile) = main_tile {
        tiles.append(&mut Util::split(&tile, main_window_count, &main_split));
    }

    if stack_window_count > 0 {
        let offset = if main_window_count > 0 || reserve_space {
            main_width
        } else {
            0
        };
        let stack_tile = Rect {
            x: container.x + offset as i32,
            w: container.w - offset,
            ..container
        };
        tiles.append(&mut Util::split(
            &stack_tile,
            stack_window_count,
            &stack_split,
        ));
    }
    tiles.to_vec()
}

mod tests {}
