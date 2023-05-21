use ascii_canvas::{style::{Style, BOLD, FG_BLUE}, AsciiCanvas, AsciiView};
use leftwm_layouts::{
    geometry::{Flip, Rect, Reserve, Rotation},
    layouts::{Columns, Main, Stack},
    Layout,
};

fn main() {
    println!("Hello, world!");

    let layout = demo_layout();

    for i in 1..6 {
        let ascii = draw(&layout, i, 42, 12);
        print!("{}\n", ascii);
    }
}

fn draw(layout: &Layout, windows: usize, w: usize, h: usize) -> String {
    let container = Rect::new(0, 0, w as u32, h as u32);
    let tiles = leftwm_layouts::apply(&layout, windows, &container);
    let mut canvas = AsciiCanvas::new(h + 2, w + 2);
    {
        let view: &mut dyn AsciiView = &mut canvas;
        //let canvas: &mut dyn AsciiView = &mut canvas;
        //let view: &mut dyn AsciiView = &mut canvas.shift(0, 0);
        view.draw_vertical_line(0..h + 1, 0);
        view.draw_vertical_line(0..h + 1, w);
        view.draw_horizontal_line(0, 0..w + 1);
        view.draw_horizontal_line(h, 0..w + 1);
        

        for (i, tile) in tiles.iter().enumerate() {
            view.draw_vertical_line(
                (tile.y as usize)..(tile.y as usize + tile.h as usize + 1),
                tile.x as usize,
            );
            view.draw_vertical_line(
                (tile.y as usize)..(tile.y as usize + tile.h as usize + 1),
                tile.x as usize + tile.w as usize,
            );
            view.draw_horizontal_line(
                tile.y as usize,
                (tile.x as usize)..(tile.x as usize + tile.w as usize + 1),
            );
            view.draw_horizontal_line(
                tile.y as usize + tile.h as usize,
                (tile.x as usize)..(tile.x as usize + tile.w as usize + 1),
            );
            let (col, row) = tile.center();
            let win_num = i + 1;
            view.write_chars(
                row as usize,
                col as usize,
                win_num.to_string().chars(),
                Style::new(),
            )
        }
    }

    /*if let Some(mut stdout) = term::stdout() {
        canvas.write_to(&mut *stdout);
    }*/

    let rows: Vec<String> = canvas
        .to_strings()
        .iter()
        .map(|row| row.to_string())
        .collect();
    return rows.join("\n");
}

fn demo_layout() -> Layout {
    leftwm_layouts::Layout {
        name: "Demo".to_string(),
        flip: Flip::None,
        rotate: Rotation::North,
        reserve: Reserve::Reserve,
        columns: Columns {
            flip: Flip::None,
            rotate: Rotation::South,
            main: Some(Main {
                count: 2,
                ..Default::default()
            }),
            stack: Stack {
                ..Default::default()
            },
            second_stack: None,
        },
    }
}
