use std::cmp;
use std::str::FromStr;

use druid::piet::{Text, TextLayout, TextLayoutBuilder};
use druid::widget::{Align, Button, Container, Flex, Label, Painter, Radio, RadioGroup, SizedBox};
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, Point, Rect, RenderContext, Widget,
    WidgetExt, WindowDesc,
};
use leftwm_layouts::geometry::Tile;
use leftwm_layouts::{LayoutEnum, LayoutModifiers};

const PRIMARY: Color = Color::rgb8(0x08, 0x0f, 0x0f);
//const ACCENT: Color = Color::rgb8(0x65, 0x64, 0xdb);
//const ACCENT_DARK: Color = Color::rgb8(0x2d, 0x2b, 0xb6);
const ACCENT: Color = Color::rgb8(0xff, 0xd6, 0x22);
const ACCENT_SHADE: Color = Color::rgb8(0xff, 0xe9, 0x85);

const WINDOW_TITLE: LocalizedString<DemoState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct DemoState {
    layout: LayoutOption,
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
            layout: LayoutOption::MainAndVertStack,
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

    fn toggle_flipped_horizontal(&mut self) {
        self.flipped_horizontal = !self.flipped_horizontal
    }

    fn toggle_flipped_vertical(&mut self) {
        self.flipped_vertical = !self.flipped_vertical
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

#[derive(Debug, Clone, Copy, Data, PartialEq)]
enum LayoutOption {
    Monocle,
    MainAndVertStack,
}

impl Into<LayoutEnum> for LayoutOption {
    fn into(self) -> LayoutEnum {
        match self {
            Self::Monocle => LayoutEnum::Monocle,
            Self::MainAndVertStack => LayoutEnum::MainAndVertStack,
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
    let selector = RadioGroup::new(vec![
        ("Monocle", LayoutOption::Monocle),
        ("MainAndVertStack", LayoutOption::MainAndVertStack),
    ])
    .lens(DemoState::layout);

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

    let flip_h = Button::new(|data: &DemoState, _env: &_| {
        format!("FlipHorziontal: {}", data.flipped_horizontal)
    })
    .on_click(move |_ctx, data: &mut DemoState, _env| data.toggle_flipped_horizontal());

    let flip_v = Button::new(|data: &DemoState, _env: &_| {
        format!("FlipVertical: {}", data.flipped_vertical)
    })
    .on_click(move |_ctx, data: &mut DemoState, _env| data.toggle_flipped_vertical());

    let flex = Flex::row()
        .with_flex_child(inc_master, 1.0)
        .with_flex_child(dec_master, 1.0)
        .with_flex_child(inc_master_count, 1.0)
        .with_flex_child(dec_master_count, 1.0)
        .with_flex_child(add_window, 1.0)
        .with_flex_child(remove_window, 1.0)
        .with_flex_child(flip_h, 1.0)
        .with_flex_child(flip_v, 1.0)
        .with_child(selector);

    flex.fix_height(60.0).background(PRIMARY)
}

fn layout_preview() -> impl Widget<DemoState> {
    Painter::new(|ctx, data: &DemoState, env| {
        let parent_size = ctx.size();
        let mut modifiers = LayoutModifiers::from(data);
        modifiers.container_size = Tile {
            x: 0,
            y: 0,
            w: parent_size.width as i32,
            h: parent_size.height as i32,
        };

        let layout: LayoutEnum = data.layout.into();

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
                    ctx.fill(rect, &ACCENT);
                    master_count = master_count - 1;
                } else {
                    ctx.fill(rect, &ACCENT_SHADE);
                }
                ctx.stroke(rect.inset(-0.5), &Color::WHITE, 1.0);

                let text = ctx.text();
                let font = text.font_family("monospace").unwrap();

                let text_layout = text
                    .new_text_layout(format!("{}", i + 1))
                    .text_color(Color::BLACK)
                    .font(font, 22.0)
                    .build()
                    .unwrap();

                let center = o.center();

                let pos = Point {
                    x: center.0 as f64 - (text_layout.size().width / 2.0),
                    y: center.1 as f64 - (text_layout.size().height / 2.0),
                };

                ctx.draw_text(&text_layout, pos);
            })
    })
    .expand()
}
