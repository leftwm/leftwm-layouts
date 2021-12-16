use crate::{
    geometry::{Tile, Util},
    Layout, LayoutModifiers,
};

#[derive(Debug)]
pub struct CenterMain;

impl Layout for CenterMain {
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Tile> {
        let tiles: &mut Vec<Tile> = &mut Vec::new();
        if window_count == 0 {
            return tiles.to_vec();
        }

        let main_window_count = self.main_window_count(window_count, modifiers);
        let stack_window_count = self.stack_window_count(window_count, modifiers);

        // the column widths [main/single-stack, stack1, stack2]
        let column_widths: Vec<i32> = match (main_window_count, stack_window_count) {
            (1.., 0) | (0, 1..) => vec![modifiers.container_size.w], // single column
            (1.., 1) => {
                // two column
                let main_width = (modifiers.container_size.w as f32 / 100.0
                    * modifiers.master_width_percentage) as i32;
                let stack1_width = modifiers.container_size.w - main_width;
                vec![main_width, stack1_width]
            }
            (1.., 2..) => {
                // three column
                let main_width = (modifiers.container_size.w as f32 / 100.0
                    * modifiers.master_width_percentage) as i32;
                let remaining_space = (modifiers.container_size.w - main_width) as usize;
                let stack_widths = Util::remainderless_division(remaining_space, 2);
                vec![main_width, stack_widths[0] as i32, stack_widths[1] as i32]
            }
            (_, _) => vec![],
        };

        let main_tile = if modifiers.master_window_count > 0 {
            match stack_window_count {
                0 => Some(modifiers.container_size),
                1 => Some(Tile {
                    w: column_widths[0],
                    ..modifiers.container_size
                }),
                _ => {
                    Some(Tile {
                        w: column_widths[0],
                        x: column_widths[2] as i32, // right of stack2
                        ..modifiers.container_size
                    })
                }
            }
        } else {
            None
        };

        if let Some(tile) = main_tile {
            tiles.append(&mut tile.split(main_window_count, &crate::geometry::SplitAxis::Vertical));
        }

        match (main_window_count, stack_window_count) {
            (0, 1..) => {
                let stack_tile = Tile {
                    x: modifiers.container_size.x + main_tile.map_or(0, |t| t.w),
                    w: modifiers.container_size.w - main_tile.map_or(0, |t| t.w),
                    ..modifiers.container_size
                };
                tiles.append(
                    &mut stack_tile
                        .split(stack_window_count, &crate::geometry::SplitAxis::Horizontal),
                );
            }
            (1.., 1) => {
                // only one stack window means only one "stack" on the right
                let main_tile = main_tile.unwrap();
                tiles.push(Tile {
                    x: main_tile.x + main_tile.w,
                    w: column_widths[1],
                    ..modifiers.container_size
                });
            }
            (1.., 2..) => {
                let master_tile = main_tile.unwrap();
                let left_stack = Tile {
                    x: modifiers.container_size.x,
                    w: column_widths[2],
                    ..modifiers.container_size
                };
                let right_stack = Tile {
                    x: master_tile.x + master_tile.w,
                    w: column_widths[1],
                    ..modifiers.container_size
                };
                let window_distribution = Util::remainderless_division(stack_window_count, 2);
                tiles.append(&mut right_stack.split(
                    window_distribution[0],
                    &crate::geometry::SplitAxis::Horizontal,
                ));
                tiles.append(&mut left_stack.split(
                    window_distribution[1],
                    &crate::geometry::SplitAxis::Horizontal,
                ));
            }
            (_, _) => {}
        }

        crate::geometry::Util::flip(modifiers.container_size, tiles, &modifiers.flipped);
        tiles.to_vec()
    }
}

mod tests {}
