use crate::{Layout, geometry::Tile, LayoutModifiers};


#[derive(Debug)]
pub struct MainAndVertStack;

impl Layout for MainAndVertStack {
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Option<Tile>> {
        if window_count == 0 {
            return vec![];
        }

        // QUESTION: where should the width limiter be implemented?
        //let column_count = match window_count {
        //    1 => 1,
        //    _ => 2,
        //};

        

        let master_width = match window_count {
            1 => modifiers.container_size.w,
            _ => (modifiers.container_size.w as f32 / 100.0 * modifiers.master_width_percentage) as i32,
        };
        let stack_width = modifiers.container_size.w - master_width;

        let mut master_x = modifiers.container_size.x;
        let stack_x = if modifiers.flipped_horizontal {
            match window_count {
                1 => 0,
                _ => {
                    master_x = modifiers.container_size.x + stack_width;
                    modifiers.container_size.x
                }
            }
        } else {
            modifiers.container_size.x + master_width
        };


        // build the master window
        let mut vec: Vec<Option<Tile>> = Vec::new();
        vec.push(Some(Tile {
            x: master_x,
            y: modifiers.container_size.y,
            w: master_width,
            h: modifiers.container_size.h,
        }));

        // stack all the others
        let height_f = modifiers.container_size.h as f32 / (window_count - 1) as f32;
        let height = height_f.floor() as i32;
        let mut y = 0;
        for _ in 1..window_count {
            vec.push(Some(Tile {
                x: stack_x,
                y: modifiers.container_size.y + y,
                w: stack_width,
                h: height,
            }));
            y += height
        }

        vec
    }
}

mod tests {
    
}