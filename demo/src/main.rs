use std::cmp;
use std::str::FromStr;

use druid::piet::{TextLayout, TextLayoutBuilder, Text};
use druid::widget::{Align, Button, Container, Flex, Painter, SizedBox, Label};
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, Rect, RenderContext, Widget, WidgetExt,
    WindowDesc, Point,
};
use leftwm_layouts::geometry::Tile;
use leftwm_layouts::{LayoutModifiers, Layouts};

const PRIMARY: Color = Color::rgb8(0x08, 0x0f, 0x0f);
const ACCENT: Color = Color::rgb8(0x65, 0x64, 0xdb);

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
        self.window_count += 1;
    }

    fn remove_window(&mut self) {
        let new_count = if self.window_count > 0 {
            self.window_count - 1
        } else {
            0
        };
        self.window_count = new_count;
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

    fn increase_master_count(&mut self) {
        self.master_window_count += 1;
    }

    fn decrease_master_count(&mut self) {
        let new_count = if self.master_window_count > 0 {
            self.master_window_count - 1
        } else {
            0
        };
        self.master_window_count = new_count;
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
        .with_child(controls())
        .with_flex_child(Container::new(layout_preview()).background(Color::RED), 2.0)
}

fn controls() -> impl Widget<DemoState> {
    let inc_master = Button::new("IncreaseMainWidth")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.increase_master_width());

    let dec_master = Button::new("DecreaseMainWidth")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.decrease_master_width());

    let add_window = Button::new("AddWindow")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.add_window());

    let remove_window = Button::new("RemoveWindow")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.remove_window());

    let inc_master_count = Button::new("IncreaseMasterCount")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.increase_master_count());

    let dec_master_count = Button::new("DecreaseMasterCount")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.decrease_master_count());

    Flex::row()
        .with_flex_child(inc_master, 1.0)
        .with_flex_child(dec_master, 1.0)
        .with_flex_child(inc_master_count, 1.0)
        .with_flex_child(dec_master_count, 1.0)
        .with_flex_child(add_window, 1.0)
        .with_flex_child(remove_window, 1.0)
        .fix_height(60.0)
        .background(PRIMARY)
}

fn layout_preview() -> impl Widget<DemoState> {

    let painter = Painter::new(|ctx, data: &DemoState, env| {
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
                .enumerate()
                .filter(|(_, o)| o.is_some())
                .map(|(i, o)| (i, o.unwrap()))
                .for_each(|(i, o)| {
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


                    let text = ctx.text();
                    let font = text.font_family("monospace").unwrap();

                    let text_layout = text
                        .new_text_layout(format!("{}", i+1))
                        .text_color(Color::WHITE)
                        .font(font, 22.0)
                        .build()
                        .unwrap();

                    let center = o.center();
                    
                    
                    let pos = Point {
                        x: center.0 as f64 - (text_layout.size().width / 2.0),
                        y: center.1 as f64 - (text_layout.size().height / 2.0)
                    };

                    
                    
                    ctx.draw_text(&text_layout, pos);
                })
                
        }
    });


    painter.expand()


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
