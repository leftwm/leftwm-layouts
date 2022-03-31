use crate::{
    geometry::{Rect, SplitAxis},
    Layout, LayoutModifiers, Util,
};

#[derive(Debug)]
pub struct Fibonacci;

impl Layout for Fibonacci {
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Rect> {
        let tiles: &mut Vec<Rect> = &mut Vec::new();
        if window_count == 0 {
            return tiles.to_vec();
        }

        let main_window_count = self.main_window_count(window_count, modifiers);
        let stack_window_count = self.stack_window_count(window_count, modifiers);

        let master_tile = if modifiers.master_window_count > 0 {
            match stack_window_count {
                0 => Some(modifiers.container_size),
                _ => Some(Rect {
                    w: (modifiers.container_size.w as f32 / 100.0
                        * modifiers.master_width_percentage) as u32,
                    ..modifiers.container_size
                }),
            }
        } else {
            None
        };

        if let Some(tile) = master_tile {
            tiles.append(&mut Util::split(
                &tile,
                main_window_count,
                &SplitAxis::Vertical,
            ));
        }

        if stack_window_count > 0 {
            let mut stack_tile = Rect {
                x: modifiers.container_size.x + master_tile.map_or(0, |t| t.w) as i32,
                w: modifiers.container_size.w - master_tile.map_or(0, |t| t.w),
                ..modifiers.container_size
            };
            let mut last_axis = SplitAxis::Vertical;
            for i in 0..stack_window_count {
                let has_next = i < stack_window_count - 1;
                last_axis = if last_axis == SplitAxis::Vertical {
                    SplitAxis::Horizontal
                } else {
                    SplitAxis::Vertical
                };
                if has_next {
                    let splitted_tiles = Util::split(&stack_tile, 2, &last_axis);
                    tiles.push(splitted_tiles[0]);
                    stack_tile = splitted_tiles[1];
                } else {
                    tiles.push(stack_tile);
                }
            }
        }

        Util::flip(modifiers.container_size, tiles, &modifiers.flipped);
        tiles.to_vec()
    }
}

mod tests {}
