use crate::{geometry::Tile, Layout, LayoutModifiers};

#[derive(Debug)]
pub struct MainAndVertStack;

impl Layout for MainAndVertStack {
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Tile> {
        let tiles: &mut Vec<Tile> = &mut Vec::new();
        if window_count == 0 {
            return tiles.to_vec();
        }

        let master_tile = if modifiers.master_window_count > 0 {
            match window_count - modifiers.master_window_count {
                0 => Some(modifiers.container_size),
                _ => Some(Tile {
                    w: (modifiers.container_size.w as f32 / 100.0
                        * modifiers.master_width_percentage) as i32,
                    ..modifiers.container_size
                }),
            }
        } else {
            None
        };

        if let Some(tile) = master_tile {
            tiles.append(&mut tile.split(
                modifiers.master_window_count,
                crate::geometry::SplitAxis::Both,
            ));
        }

        let stack_window_count = window_count - modifiers.master_window_count;
        if stack_window_count > 0 {
            let stack_tile = Tile {
                x: modifiers.container_size.x + master_tile.map_or(0, |t| t.w),
                w: modifiers.container_size.w - master_tile.map_or(0, |t| t.w),
                ..modifiers.container_size
            };
            tiles.append(
                &mut stack_tile.split(stack_window_count, crate::geometry::SplitAxis::Horizontal),
            );
        }

        crate::geometry::Util::flip(modifiers.container_size, tiles, &modifiers.flipped);
        tiles.to_vec()
    }
}

mod tests {}
