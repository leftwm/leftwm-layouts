use std::cmp;
use std::str::FromStr;

use druid::widget::{Button, Flex, Painter};
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, Rect, RenderContext, Widget, WindowDesc,
};
use leftwm_layouts::geometry::Tile;
use leftwm_layouts::{LayoutModifiers, Layouts};

const WINDOW_TITLE: LocalizedString<DemoState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct DemoState {
    window_count: usize,
    master_width_percentage: f32,
    master_window_count: usize,
    max_column_width: Option<u32>,
    flipped_horizontal: bool,
    flipped_vertical: bool,
}

impl Default for DemoState {
    fn default() -> Self {
        Self {
            window_count: 5,
            master_width_percentage: 60.0,
            master_window_count: 1,
            max_column_width: None,
            flipped_horizontal: false,
            flipped_vertical: false,
        }
    }
}

impl DemoState {
    fn add_window(&mut self) {
        self.window_count += 1
    }

    fn remove_window(&mut self) {
        self.window_count = cmp::max(self.window_count - 1, 0);
    }

    fn increase_master_width(&mut self) {
        let new_width = self.master_width_percentage + 5.0;
        if new_width > 100.0 {
            self.master_width_percentage = 100.0;
        } else {
            self.master_width_percentage = new_width;
        }
    }

    fn decrease_master_width(&mut self) {
        let new_width = self.master_width_percentage - 5.0;
        if new_width < 0.0 {
            self.master_width_percentage = 0.0;
        } else {
            self.master_width_percentage = new_width;
        }
    }
}

impl From<&DemoState> for LayoutModifiers {
    fn from(value: &DemoState) -> Self {
        LayoutModifiers {
            master_width_percentage: value.master_width_percentage,
            master_window_count: value.master_window_count,
            max_column_width: value.max_column_width,
            flipped_horizontal: value.flipped_horizontal,
            flipped_vertical: value.flipped_vertical,
            ..Default::default()
        }
    }
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((1280.0, 720.0));

    // create the initial app state
    let initial_state = DemoState::default();

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<DemoState> {
    Flex::column()
        .with_flex_child(controls(), 1.0)
        .with_flex_child(layout_preview(), 1.0)
}

fn controls() -> impl Widget<DemoState> {
    let inc_master = Button::new("IncreaseMainWidth")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.increase_master_width());

    let dec_master = Button::new("DecreaseMainWidth")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.decrease_master_width());

    Flex::row()
        .with_flex_child(inc_master, 1.0)
        .with_flex_child(dec_master, 1.0)
}

fn layout_preview() -> impl Widget<DemoState> {
    Painter::new(|ctx, data: &DemoState, env| {
        let parent_rect = ctx.size().to_rect();

        let mut modifiers = LayoutModifiers::from(data);
        modifiers.container_size = Tile {
                x: parent_rect.x0 as i32,
                y: parent_rect.y0 as i32,
                w: (parent_rect.x1 - parent_rect.x0) as i32,
                h: (parent_rect.y1 - parent_rect.y0) as i32,
        };
        let layout = Layouts::from_str("MainAndVertStack");

        if let Ok(layout) = layout {
            let calcs = layout.get().apply(data.window_count, &modifiers);
            let mut master_count = layout.get().master_window_count(6, &modifiers);
            // println!("{:?}", calcs);
            calcs
                .into_iter()
                .filter(|o| o.is_some())
                .map(|o| o.unwrap())
                .for_each(|o| {
                    let rect = Rect::new(
                        o.x.into(),
                        o.y.into(),
                        (o.x + o.w).into(),
                        (o.y + o.h).into(),
                    );
                    if master_count > 0 {
                        ctx.fill(rect, &env.get(theme::PRIMARY_LIGHT));
                        master_count = master_count - 1;
                    } else {
                        ctx.fill(rect, &env.get(theme::PRIMARY_DARK));
                    }
                    ctx.stroke(rect.inset(-0.5), &Color::WHITE, 1.0);
                })
        }
    })

    /*
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox);

    // center the two widgets in the available space
    Align::centered(layout)

    */
}
