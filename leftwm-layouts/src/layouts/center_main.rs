use crate::{
    geometry::{Rect, SplitAxis},
    Layout, LayoutModifiers, Util,
};

/// ## Demonstration
/// 1 window
/// ```text
/// +-----------------------+
/// |                       |
/// |                       |
/// |           1           |
/// |                       |
/// |                       |
/// +-----------------------+
/// ```
/// 2 windows
/// ```text
/// +-----------+-----------+
/// |           |           |
/// |           |           |
/// |     2     |     1     |
/// |           |           |
/// |           |           |
/// +-----------+-----------+
/// ```
/// 3 windows
/// ```text
/// +-----+-----------+-----+
/// |     |           |     |
/// |     |           |     |
/// |  2  |     1     |  3  |
/// |     |           |     |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
/// 4 windows
/// ```text
/// +-----+-----------+-----+
/// |  2  |           |     |
/// |     |           |     |
/// +-----+     1     |  4  |
/// |  3  |           |     |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
/// 5 windows
/// ```text
/// +-----+-----------+-----+
/// |  2  |           |  4  |
/// |     |           |     |
/// +-----+     1     +-----+
/// |  3  |           |  5  |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
#[derive(Debug)]
pub struct CenterMain;

impl Layout for CenterMain {
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Rect> {
        let tiles: &mut Vec<Rect> = &mut Vec::new();
        if window_count == 0 {
            return tiles.to_vec();
        }

        let main_window_count = self.main_window_count(window_count, modifiers);
        let stack_window_count = self.stack_window_count(window_count, modifiers);

        // the column widths [main/single-stack, stack1, stack2]
        let column_widths: Vec<u32> = match (main_window_count, stack_window_count) {
            (1.., 0) | (0, 1..) => vec![modifiers.container_size.w], // single column
            (1.., 1) => {
                // two column
                let main_width = (modifiers.container_size.w as f32 / 100.0
                    * modifiers.master_width_percentage) as u32;
                let stack1_width = modifiers.container_size.w - main_width;
                vec![main_width, stack1_width]
            }
            (1.., 2..) => {
                // three column
                let main_width = (modifiers.container_size.w as f32 / 100.0
                    * modifiers.master_width_percentage) as u32;
                let remaining_space = (modifiers.container_size.w - main_width) as usize;
                let stack_widths = Util::remainderless_division(remaining_space, 2);
                vec![main_width, stack_widths[0] as u32, stack_widths[1] as u32]
            }
            (_, _) => vec![],
        };

        let main_tile = if modifiers.master_window_count > 0 {
            match stack_window_count {
                0 => Some(modifiers.container_size),
                1 => Some(Rect {
                    w: column_widths[0],
                    x: column_widths[1] as i32,
                    ..modifiers.container_size
                }),
                _ => {
                    Some(Rect {
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
            tiles.append(&mut Util::split(
                &tile,
                main_window_count,
                &SplitAxis::Vertical,
            ));
        }

        match (main_window_count, stack_window_count) {
            (0, 1..) => {
                let stack_tile = Rect {
                    x: modifiers.container_size.x + main_tile.map_or(0, |t| t.w) as i32,
                    w: modifiers.container_size.w - main_tile.map_or(0, |t| t.w),
                    ..modifiers.container_size
                };
                tiles.append(&mut Util::split(
                    &stack_tile,
                    stack_window_count,
                    &SplitAxis::Horizontal,
                ));
            }
            (1.., 1) => {
                // only one stack window means only one "stack" on the left
                tiles.push(Rect {
                    x: 0,
                    w: column_widths[1],
                    ..modifiers.container_size
                });
            }
            (1.., 2..) => {
                let master_tile = main_tile.unwrap();
                let left_stack = Rect {
                    x: modifiers.container_size.x,
                    w: column_widths[2],
                    ..modifiers.container_size
                };
                let right_stack = Rect {
                    x: master_tile.x + master_tile.w as i32,
                    w: column_widths[1],
                    ..modifiers.container_size
                };
                let window_distribution = Util::remainderless_division(stack_window_count, 2);
                tiles.append(&mut Util::split(
                    &left_stack,
                    window_distribution[0],
                    &SplitAxis::Horizontal,
                ));
                tiles.append(&mut Util::split(
                    &right_stack,
                    window_distribution[1],
                    &SplitAxis::Horizontal,
                ));
            }
            (_, _) => {}
        }

        Util::flip(modifiers.container_size, tiles, &modifiers.flipped);
        tiles.to_vec()
    }
}

mod tests {}
